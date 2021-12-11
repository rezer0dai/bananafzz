#[macro_use]
extern crate serde_derive;

extern crate rand;
use rand::Rng;

use std::{thread, time};

extern crate core;

use core::banana::observer::{ICallObserver, IStateObserver};
use core::state::state::StateInfo;

extern crate common;
use self::common::ModuleCallbacks;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct RaceUnlockConfig {
    sleep: u64,
    racer_count: usize,
}
struct RaceUnlock {
    cfg: RaceUnlockConfig,
    callbacks: Box<dyn ModuleCallbacks>,
}

impl IStateObserver for RaceUnlock {
    fn notify_ctor(&self, state: &StateInfo) -> bool {
        if 0 == state.total {
            return true //dupped starting without ctor ~ likely created by this module
        }
        for _ in 0..self.cfg.racer_count {
            let info = state.clone();
            let sleep = self.cfg.sleep;
            let push_state = self.callbacks.push_state();
            thread::spawn(move || {
                thread::sleep(time::Duration::from_millis(
                    rand::thread_rng().gen_range(0..=sleep),
                ));
                push_state(info.id, &info.fd);
            });
        }
        true
    }
    fn notify_dtor(&self, _: &StateInfo) {}
}

impl RaceUnlock {
    pub(crate) fn new(cfg: &RaceUnlockConfig, callbacks: Box<dyn ModuleCallbacks>) -> RaceUnlock {
        RaceUnlock {
            cfg: *cfg,
            callbacks: callbacks,
        }
    }
}
pub fn observers(
    cfg: &Option<RaceUnlockConfig>,
    callbacks: Box<dyn ModuleCallbacks>,
) -> (
    Option<Box<dyn IStateObserver>>,
    Option<Box<dyn ICallObserver>>,
) {
    match *cfg {
        Some(ref cfg) => (Some(Box::new(RaceUnlock::new(cfg, callbacks))), None),
        _ => (None, None),
    }
}
