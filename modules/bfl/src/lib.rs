#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate rand;

extern crate libc;

use std::{rc::Rc, sync::RwLock};

mod bfl;
pub mod poc;
pub mod repro;
pub mod shmem;
use bfl::BananizedFuzzyLoop;
pub mod info;
pub use info::BananizedFuzzyLoopConfig;
pub mod crossover;

extern crate generic;

extern crate core;

use core::banana::observer::{ICallObserver, IStateObserver};
use core::exec::call::Call;
use core::state::state::StateInfo;

struct BflProxy {
    lookup: Rc<RwLock<BananizedFuzzyLoop>>,
}
impl BflProxy {
    fn new(lookup: Rc<RwLock<BananizedFuzzyLoop>>) -> BflProxy {
        BflProxy { lookup: lookup }
    }
}
impl ICallObserver for BflProxy {
    fn notify(&self, state: &StateInfo, call: &mut Call) -> bool {
        match self.lookup.write() {
            Ok(mut bfl) => bfl.notify_locked(state, call),
            Err(_) => panic!("[BFL] lock failed - CALLS"),
        }
    }
    fn aftermath(&self, state: &StateInfo, call: &mut Call) {
        match self.lookup.write() {
            Ok(mut bfl) => bfl.aftermath_locked(state, call),
            Err(_) => panic!("[BFL] lock failed - CALLS"),
        }
    }
}
impl IStateObserver for BflProxy {
    fn notify_ctor(&self, state: &StateInfo) -> bool {
        match self.lookup.write() {
            Ok(mut bfl) => bfl.notify_ctor_locked(state),
            Err(_) => panic!("[BFL] lock failed - CTORS"),
        }
    }
}

pub fn observers(
    cfg: &Option<BananizedFuzzyLoopConfig>,
) -> (
    Option<Box<dyn IStateObserver>>,
    Option<Box<dyn ICallObserver>>,
) {
    match *cfg {
        Some(ref cfg) => {
            let lookup = Rc::new(RwLock::new(BananizedFuzzyLoop::new(cfg)));
            (
                Some(Box::new(BflProxy::new(Rc::clone(&lookup)))),
                Some(Box::new(BflProxy::new(Rc::clone(&lookup)))),
            )
        }
        _ => (None, None),
    }
}
