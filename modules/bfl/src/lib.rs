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

extern crate common;

common::callback_proxy!(BananizedFuzzyLoop);

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
                Some(Box::new(Proxy::new(Rc::clone(&lookup)))),
                Some(Box::new(Proxy::new(Rc::clone(&lookup)))),
            )
        }
        _ => (None, None),
    }
}
