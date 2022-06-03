#![feature(integer_atomics)]

extern crate core;

use core::banana::observer::{ICallObserver, IStateObserver, WantedMask};
use core::exec::call::Call;
use core::state::state::StateInfo;

extern crate log;
use log::info;

use std::sync::{
    atomic::{AtomicU64, Ordering},
};

struct Syncer {
    wildcard: AtomicU64,
    wanted: Option<WantedMask>,
}

impl Syncer {
    fn revert(&mut self, state: &StateInfo, _: &Call, mask: WantedMask) {
        log::trace!("getting revoked : {mask:?}");
        let _ = self.wildcard
                    .compare_exchange(
                        state.uid(),
                        0, // pikachu failed, yo charizard ?
                        Ordering::SeqCst,
                        Ordering::SeqCst,
                    );

        self.wanted.insert(mask);
    }

    fn notify(&mut self, state: &StateInfo, call: &mut Call) -> Result<bool, WantedMask> {
        log::trace!("syncer");
        if self.wanted.is_some() {
            if let Some(ref mask) = self.wanted {
                if 0 != mask.uid && state.uid() != mask.uid {
                    return Err(mask.clone())
                }
                if 0 != mask.sid && 0 == u64::from(state.id) & mask.sid {
                    return Err(mask.clone())
                }
                log::trace!("APPROVED : {mask:?} :: {:?} {:?} | uid: {:?}", state.id, call.id(), state.uid());
            }
            self.wanted = None;
        }
        
        let uid = self.wildcard.compare_exchange(
            0,
            state.uid(), // ok pikachu we choose you
            Ordering::SeqCst,
            Ordering::SeqCst,
        ).unwrap_or(state.uid());

        if 0 == uid { // uid taken
            log::trace!("pass <{} :: {uid}> wait for : <{}>", state.uid(), self.wildcard.load(Ordering::SeqCst));
            return Ok(true)
        }

        let _ = self.wildcard.compare_exchange(
            state.uid(), 
            0, // pikachu you failed ?
            Ordering::SeqCst,
            Ordering::SeqCst,
        ).unwrap_or(0);
        log::trace!("deny <{} :: {uid}> wait for : <{}>", state.uid(), self.wildcard.load(Ordering::SeqCst));
        Err(WantedMask{
            mid: 1,
            ..WantedMask::default()
        })
    }
    fn aftermath(&self, state: &StateInfo, _call: &mut Call) {
        let _ = self.ctor(state); // ok to treat it the same
    }

    fn ctor(&self, state: &StateInfo) -> bool {
        if 0 == state.total {
            // racers have free pass,
            return true; // as no actuall call is invoked here anyway
        }
        //let ret = {
        if let Ok(v) =
            self
                .wildcard
                .compare_exchange(
                    state.uid(), 
                    0, // pikachu suceed
                    Ordering::SeqCst, 
                    Ordering::SeqCst)
        {
            //println!("CTOR:: compare {v:?} vs {:?}", state.uid());
            v == state.uid()
        } else {
            //println!("CTOR:: yop, it is the thing ..");
            false //false// its OK, it can be previous allowed
        }
        //}; if ret {println!("X2-GOGOGGOGO");} ret
    }

    fn dtor(&mut self, state: &StateInfo) {
        //println!("DEAD -> {:?}", state.uid());
        if let Ok(v) = self.wildcard.compare_exchange(
            state.uid(),
            0, // pikachu was killed
            Ordering::SeqCst,
            Ordering::SeqCst,
        ) {
            info!("[syncer] pikachu is done for <{v}>")
        }
    }

    fn new() -> Self {
        Syncer {
            wildcard: AtomicU64::new(0u64),
            wanted: None,
        }
    }
}

common::callback_proxy!(Syncer);

pub fn observers() -> (
    Option<Box<dyn IStateObserver>>,
    Option<Box<dyn ICallObserver>>,
) {
    let lookup = Arc::new(RwLock::new(Syncer::new()));
    (
        Some(Box::new(Proxy::new(Arc::clone(&lookup)))),
        Some(Box::new(Proxy::new(Arc::clone(&lookup)))),
    )
}
