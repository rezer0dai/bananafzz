#![feature(integer_atomics)]

#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate core;

use core::exec::call::Call;
use core::banana::observer::{ICallObserver, IStateObserver};
use core::banana::bananaq;
use core::state::state::StateInfo;

use std::sync::atomic::{AtomicU32, Ordering};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct LimiterConfig {
    num_of_calls: u32,
    failed_limit: u32,
}
struct Limiter {
    cfg: LimiterConfig,
    counter: AtomicU32,
    n_total: AtomicU32,
}

impl ICallObserver for Limiter {
    fn notify(&self, state: &StateInfo, _: &mut Call) -> bool {
        if self.n_total.fetch_add(1, Ordering::Relaxed) < self.cfg.failed_limit { 
            return true 
        }

        bananaq::stop(&state.bananaq).unwrap();
        false
    }
    fn aftermath(&self, state: &StateInfo, _: &mut Call) {
        if self.counter.fetch_add(1, Ordering::Relaxed) > self.cfg.num_of_calls  {
            bananaq::stop(&state.bananaq).unwrap()
        }
    }
}

impl Limiter {
    fn new(cfg: &LimiterConfig) -> Limiter {
        Limiter {
            cfg: *cfg,
            counter: AtomicU32::new(0),
            n_total: AtomicU32::new(0),
        }
    }
}

pub fn observers(
    cfg: &Option<LimiterConfig>,
) -> (
    Option<Box<dyn IStateObserver>>,
    Option<Box<dyn ICallObserver>>,
) {
    match *cfg {
        Some(ref cfg) => (None, Some(Box::new(Limiter::new(&cfg)))),
        _ => (None, None),
    }
}
