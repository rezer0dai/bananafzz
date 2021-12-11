#![feature(integer_atomics)]

#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate rand;
use rand::Rng;

extern crate core;

use core::exec::call::Call;
use core::banana::observer::{ICallObserver, IStateObserver};
use core::state::state::StateInfo;

extern crate common;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct MediatorConfig {
    mediate_ctor_weight: u8,
}
struct Mediator {
    cfg: MediatorConfig,
}

impl ICallObserver for Mediator {
    fn notify(&self, state: &StateInfo, _call: &Call) -> bool {
        if !state.fd.is_invalid() {
            return true;
        }
        // this is due bad fuzzer implementation and no good config reflecting that
        //   ctors overhlming fuzzing, and core logic is ommited
        0 == rand::thread_rng().gen::<u8>() % self.cfg.mediate_ctor_weight
    }
}

impl Mediator {
    pub(crate) fn new(cfg: &MediatorConfig) -> Mediator {
        Mediator { cfg: *cfg }
    }
}

pub fn observers(
    cfg: &Option<MediatorConfig>,
) -> (
    Option<Box<dyn IStateObserver>>,
    Option<Box<dyn ICallObserver>>,
) {
    match *cfg {
        Some(ref cfg) => (None, Some(Box::new(Mediator::new(&cfg)))),
        _ => (None, None),
    }
}
