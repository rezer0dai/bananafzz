#![feature(integer_atomics)]

extern crate core;

use core::exec::call::Call;
use core::banana::observer::{ICallObserver, IStateObserver};
use core::state::state::StateInfo;

use std::sync::{
    Arc, 
    atomic::{AtomicU64, Ordering},
};

struct Syncer {
    syncer: Option<Arc<Syncer>>,
    wildcard: AtomicU64,
}

impl ICallObserver for Syncer {
    fn notify(&self, state: &StateInfo, _call: &mut Call) -> bool {
        let syncer = self.syncer.as_ref().unwrap();
//println!("TRY TO GO : {:?} -> {:?}", (state.uid(), state.id), syncer.wildcard.load(Ordering::SeqCst));

        if let Ok(v) = if 0 == syncer.wildcard.load(Ordering::SeqCst) {
            syncer.wildcard.compare_exchange(0, state.uid(), // ok pikachu we choose you
                Ordering::SeqCst, Ordering::SeqCst)
        } else if state.uid() == syncer.wildcard.load(Ordering::SeqCst) {
            syncer.wildcard.compare_exchange(state.uid(), 0,// pikachu failed us, charizard ?
                Ordering::SeqCst, Ordering::SeqCst)
        } else {
//println!("not loaded wildcard {:?}", syncer.wildcard.load(Ordering::SeqCst));
            return false
        } { 
//            println!("=>{:?} push trough {v:?}", 0 == v);
            0 == v 
        } else { false }
    }
    fn aftermath(&self, state: &StateInfo, _call: &mut Call) { 
        if self.notify_ctor(state) { // ok to treat it the same
//            println!("X1-GOGOGGOGO");
        }
    }
}

impl IStateObserver for Syncer {
    fn notify_ctor(&self, state: &StateInfo) -> bool {
        if 0 == state.total { // racers have free pass,
            return true // as no actuall call is invoked here anyway
        }
        let syncer = self.syncer.as_ref().unwrap();
//let ret = {
        if let Ok(v) = syncer.wildcard.compare_exchange(state.uid(), 0,
                Ordering::SeqCst, Ordering::SeqCst)
        { v == state.uid() } else { false }
//}; if ret {println!("X2-GOGOGGOGO");} ret
    }

    fn notify_dtor(&self, state: &StateInfo) { 
        let syncer = self.syncer.as_ref().unwrap();
        if let Ok(v) = syncer.wildcard.compare_exchange(state.uid(), 0,// pikachu was killed
            Ordering::SeqCst, Ordering::SeqCst) {
            println!("[syncer] pikachu is done for <{v}>")
        }
    }
}

impl Syncer {
    fn new() -> Self {
        Syncer {
            syncer: None,
            wildcard: AtomicU64::new(0u64),
        }
    }
    fn fork(syncer: &Arc<Syncer>) -> Self {
        Syncer {
            syncer: Some(Arc::clone(syncer)),
            wildcard: AtomicU64::new(0u64),
        }
    }
}

pub fn observers(
) -> (
    Option<Box<dyn IStateObserver>>,
    Option<Box<dyn ICallObserver>>,
    ) 
{
    let syncer = Arc::new(Syncer::new());
    let syncer_o = Syncer::fork(&Arc::clone(&syncer));
    let syncer_s = Syncer::fork(&Arc::clone(&syncer));

    assert!(!syncer_o.syncer.is_none() && !syncer_s.syncer.is_none());

    (
    Some(Box::new(syncer_s)),
    Some(Box::new(syncer_o)),
    )
}
