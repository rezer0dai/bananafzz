#![feature(integer_atomics)]

#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate core;

use core::exec::call::Call;
use core::banana::observer::{ICallObserver, IStateObserver};
use core::state::state::StateInfo;

use std::sync::atomic::{AtomicU32, Ordering};

extern crate common;
use self::common::ModuleCallbacks;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct LimiterConfig {
    num_of_calls: u32,
}
struct Limiter {
    cfg: LimiterConfig,
    counter: AtomicU32,
    callbacks: Box<dyn ModuleCallbacks>,
}

impl ICallObserver for Limiter {
    fn notify(&self, _: &StateInfo, call: &mut Call) -> bool {
        // if 0 == state.sucess {//we dont care about ctors - ouuuch we care in mini pocs ...
        //     return true
        // }
        if call.id().is_default() {
          return true
        }
        if self.counter.fetch_add(1, Ordering::Relaxed) < self.cfg.num_of_calls {
//            println!("LIMITER passtrough! : {:?} -> {} [{:?} vs {}]", call.id(), call.name(), self.counter, self.cfg.num_of_calls);
            return true;
        }
        self.callbacks.stop_fuzzing();
        false
    }
}

impl Limiter {
    pub(crate) fn new(cfg: &LimiterConfig, callbacks: Box<dyn ModuleCallbacks>) -> Limiter {
        Limiter {
            cfg: *cfg,
            counter: AtomicU32::new(0),
            callbacks: callbacks,
        }
    }
}

pub fn observers(
    cfg: &Option<LimiterConfig>,
    callbacks: Box<dyn ModuleCallbacks>,
) -> (
    Option<Box<dyn IStateObserver>>,
    Option<Box<dyn ICallObserver>>,
) {
    match *cfg {
        Some(ref cfg) => (None, Some(Box::new(Limiter::new(&cfg, callbacks)))),
        _ => (None, None),
    }
}
