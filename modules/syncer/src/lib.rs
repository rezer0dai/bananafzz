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
        println!("getting revoked : {mask:?}");
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
        if self.wanted.is_some() {
            if let Some(ref mask) = self.wanted {
                if 0 != mask.uid && state.uid() != mask.uid {
                    return Err(mask.clone())
                }
                if 0 != mask.sid && 0 == u64::from(state.id) & mask.sid {
                    return Err(mask.clone())
                }
                println!("APPROVED : {mask:?} :: {:?} {:?} | uid: {:?}", state.id, call.id(), state.uid());
            }
            self.wanted = None;
        }
        let uid = self.wildcard.load(Ordering::SeqCst);
        if state.uid() == uid { // owned
            return Ok(true)
        }
        let uid = self.notify_exec(state, call);
        if 0 == uid { // uid taken
            //println!("pass <{} :: {uid}> wait for : <{}>", state.uid(), self.wildcard.load(Ordering::SeqCst));
            return Ok(true)
        }
        //println!("deny <{} :: {uid}> wait for : <{}>", state.uid(), self.wildcard.load(Ordering::SeqCst));
        Err(WantedMask{
            mid: 1,
            uid: uid,
            sid: 0,
            cid: 0,
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

    fn notify_exec(&mut self, state: &StateInfo, _call: &mut Call) -> u64 {
        let uid = self.wildcard.load(Ordering::SeqCst);
        if 0 != uid { // busy
            return uid
        }
        self.wildcard.compare_exchange(
            0,
            state.uid(), // ok pikachu we choose you
            Ordering::SeqCst,
            Ordering::SeqCst,
        ).unwrap_or(uid)
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
