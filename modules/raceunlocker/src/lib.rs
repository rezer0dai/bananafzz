#[macro_use]
extern crate serde_derive;

extern crate rand;
use rand::Rng;

use std::{
    time,
    thread, 
    sync::Weak,
};

extern crate core;

use core::banana::observer::{ICallObserver, IStateObserver};
use core::state::{state::StateInfo, id::StateTableId};
use core::exec::fd_info::Fd;
use core::banana::bananaq::FuzzyQ;

#[allow(improper_ctypes)]
extern "C" {
    pub fn push_state(bananaq: &Weak<FuzzyQ>, state: StateTableId, fd: &Fd);
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct RaceUnlockConfig {
    sleep: u64,
    racer_count: usize,
}
struct RaceUnlock {
    cfg: RaceUnlockConfig,
}

impl IStateObserver for RaceUnlock {
    fn notify_ctor(&self, state: &StateInfo) -> bool {
        if 0 == state.total {
            return true //dupped starting without ctor ~ likely created by this module
        }
        for _ in 0..self.cfg.racer_count {
            let info = state.clone();
            let sleep = self.cfg.sleep;
            let bananaq = state.bananaq().clone();
            thread::spawn(move || {
                thread::sleep(time::Duration::from_millis(
                    rand::thread_rng().gen_range(0..=sleep),
                ));
                unsafe {
                    push_state(&bananaq, info.id, &info.fd)
                }
            });
        }
        true
    }
    fn notify_dtor(&self, _: &StateInfo) {}
}

impl RaceUnlock {
    pub fn new(cfg: &RaceUnlockConfig) -> RaceUnlock {
        RaceUnlock {
            cfg: *cfg,
        }
    }
}
pub fn observers(
    cfg: &Option<RaceUnlockConfig>,
) -> (
    Option<Box<dyn IStateObserver>>,
    Option<Box<dyn ICallObserver>>,
) {
    if let Some(ref cfg) = *cfg {
        (Some(Box::new(RaceUnlock::new(cfg))), None)
        
    } else { (None, None) }
}
