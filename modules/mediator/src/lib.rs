#![feature(map_first_last)]

use std::{rc::Rc, sync::RwLock, collections::BTreeMap};

extern crate serde_derive;
extern crate serde;

extern crate rand;
use rand::Rng;

extern crate core;

use core::exec::call::Call;
use core::banana::observer::{ICallObserver, IStateObserver};
use core::state::state::StateInfo;

extern crate common;

struct Mediator {
    stats: BTreeMap<u64, usize>
}
impl Mediator {
    fn new() -> Self {
        Self {
            stats: BTreeMap::new(),
        }
    }
    fn notify(&mut self, state: &StateInfo, call: &mut Call) -> bool {
        self.notify_impl(state, call).unwrap_or(true)
    }

    fn notify_impl(&mut self, state: &StateInfo, _call: &mut Call) -> Result<bool, ()> {
        let cur = self.stats.get_key_value(&u64::from(state.id)).ok_or(())?.1;
        let min = self.stats.first_key_value().ok_or(())?.1;
        if cur == min {
            return Ok(true)
        }
        // objects hard to fuzz we want to push little more
        let prob = f64::max(0.1, 1.0 / (cur - min) as f64);
        Ok(rand::thread_rng().gen_bool(prob))
    }
    fn aftermath(&mut self, state: &StateInfo, call: &mut Call) -> bool {
        if !call.ok() {
            return false
        }
        self.ctor(state)
    }
    fn ctor(&mut self, state: &StateInfo) -> bool {
        let sid = u64::from(state.id);
        if !self.stats.contains_key(&sid) {
            self.stats.insert(sid, 0);
        }
        *self.stats.get_mut(&sid).unwrap() += 1;
        true
    }
}

common::callback_proxy!(Mediator);

pub fn observers() -> (
    Option<Box<dyn IStateObserver>>,
    Option<Box<dyn ICallObserver>>,
) {
    let lookup = Rc::new(RwLock::new(Mediator::new()));
    (
        Some(Box::new(Proxy::new(Rc::clone(&lookup)))),
        Some(Box::new(Proxy::new(Rc::clone(&lookup)))),
    )
}
