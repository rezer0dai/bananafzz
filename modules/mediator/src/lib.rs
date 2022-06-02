#![feature(map_first_last)]

use std::collections::BTreeMap;

extern crate serde_derive;
extern crate serde;

extern crate rand;
use rand::Rng;

extern crate core;

use core::exec::call::Call;
use core::banana::observer::{ICallObserver, IStateObserver, WantedMask};
use core::state::state::StateInfo;

extern crate common;

struct Mediator {
    stats: BTreeMap<u64, usize>,
    wanted: Option<WantedMask>,
}
impl Mediator {
    fn new() -> Self {
        Self {
            stats: BTreeMap::new(),
            wanted: None,
        }
    }
    fn notify(&mut self, state: &StateInfo, _: &mut Call) -> Result<bool, WantedMask> {
        if self.wanted.is_some() {
            if let Some(ref mask) = self.wanted {
                if 0 != mask.uid && state.uid() != mask.uid {
                    return Err(mask.clone())
                }
                if 0 != mask.sid && 0 == u64::from(state.id) & mask.sid {
                    return Err(mask.clone())
                }
            }
            self.wanted = None;
        }
        if self.notify_impl(&u64::from(state.id)).unwrap_or(true) {
            return Ok(true)
        }
        let sids = self.stats
                    .iter()
                    .map(|(k, _)| *k)
                    .collect::<Vec<u64>>();

        let mask = sids.iter()
                    .filter(|&sid| self.notify_impl(sid).is_ok())
                    .fold(0, |mask, sid| mask | sid);

        Err(WantedMask{
            mid: 2,
            uid: 0,
            sid: mask,
            cid: 0,
        })
    }

    fn notify_impl(&mut self, sid: &u64) -> Result<bool, ()> {
        let cur = self.stats.get_key_value(sid).ok_or(())?.1;
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
    pub fn dtor(&mut self, _state: &StateInfo) { }
    pub fn revert(&mut self, _info: &StateInfo, _call: &Call, mask: WantedMask) { 
        self.wanted.insert(mask);
    }
}

common::callback_proxy!(Mediator);

pub fn observers() -> (
    Option<Box<dyn IStateObserver>>,
    Option<Box<dyn ICallObserver>>,
) {
    let lookup = Arc::new(RwLock::new(Mediator::new()));
    (
        Some(Box::new(Proxy::new(Arc::clone(&lookup)))),
        Some(Box::new(Proxy::new(Arc::clone(&lookup)))),
    )
}
