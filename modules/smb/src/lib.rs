#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate core;
use core::exec::call::Call;
use core::banana::observer::{ICallObserver, IStateObserver};
use core::state::state::StateInfo;

use std::sync::atomic::{AtomicU64, Ordering};

struct SuperMarioBros2 {
    theone: AtomicU64,
}

impl SuperMarioBros2 {
    fn new() -> Self {
        SuperMarioBros2 {
            theone: AtomicU64::new(0),
        }
    }
}

impl ICallObserver for SuperMarioBros2 {
    fn notify(&self, state: &StateInfo, _: &mut Call) -> bool {
//println!("--> notify {:?}+{:?} :: {:?} :: {:?}", state.uid(), u64::from(state.id), state.level, state.fd);
        if state.uid() == self.theone.load(Ordering::SeqCst) {
            return true
        }
        if 0 == state.level {
            return true//ctors may pass through by default
        }
        if 1 != state.level {
//println!("--> refusing {:?}", state.level);
            return false//we wait for leader at first!
        }
        if let Ok(v) = self.theone.compare_exchange(0, state.uid(), 
            Ordering::SeqCst, Ordering::SeqCst) {
//if v!=0 { println!("--> not ready {:?}", v); }
            return 0 == v
        }
//println!("--> nope {:?} x {:?}", self.theone.load(Ordering::SeqCst), state.uid);
        false
    }

    fn aftermath(&self, state: &StateInfo, _: &mut Call) {
        if state.uid() != self.theone.load(Ordering::SeqCst) {
            return
        }
        assert!(state.uid() == self.theone.load(Ordering::SeqCst), 
            "[SMB2] aftermath with different target {:?} vs {:?}",
            state.uid(), self.theone);
        if 1 != state.level {
            return//reset it when we made a move and we are bck at lead_pos level
        }
        self.theone.store(0, Ordering::SeqCst)
    }
}

pub fn observer() -> Box<dyn ICallObserver> { 
    Box::new(SuperMarioBros2::new()) }
