#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate libc;

use std::{
    rc::Rc,
    sync::RwLock,
};

mod shmem;
mod poc;
mod repro;
mod bfl;
use bfl::BananizedFuzzyLoop;
pub use bfl::BananizedFuzzyLoopConfig;
mod splice;

extern crate generic;

extern crate core;

use core::exec::call::Call;
use core::banana::observer::{ICallObserver, IStateObserver};
use core::state::state::StateInfo;

struct BflProxy {
    lookup: Rc<RwLock<BananizedFuzzyLoop>>,
}
impl BflProxy {
    fn new(lookup: Rc<RwLock<BananizedFuzzyLoop>>) -> BflProxy {
        BflProxy {
            lookup: lookup,
        }
    }
}
impl ICallObserver for BflProxy {
    fn notify(&self, state: &StateInfo, call: &mut Call) -> bool {
        match self.lookup.write() {
            Ok(mut bfl) => bfl.notify_locked(state, call),
            Err(_) => panic!("[BFL] lock failed - CALLS")
        }
    }
}
impl IStateObserver for BflProxy {
    fn notify_ctor(&self, state: &StateInfo) -> bool {
        match self.lookup.write() {
            Ok(mut bfl) => bfl.notify_ctor_locked(state),
            Err(_) => panic!("[BFL] lock failed - CTORS")
        }
    }
    fn notify_dtor(&self, _: &StateInfo) {}
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
                Some(Box::new(BflProxy::new(Rc::clone(&lookup))))
            )
        }
        _ => (None, None),
    }
}
