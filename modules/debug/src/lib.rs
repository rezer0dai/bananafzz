#![feature(integer_atomics)]

#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate core;

use core::exec::call::Call;
use core::exec::id::CallTableId;
use core::banana::observer::{ICallObserver, IStateObserver, WantedMask};
use core::state::state::StateInfo;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct DebugConfig {
    noisy: bool,
    ctors_included: bool,
    mask: u64,
    only_successfull: bool,
}
struct Debug {
    cfg: DebugConfig,
}

//use std::collections::HashMap;

impl ICallObserver for Debug {
    fn notify(&self, state: &StateInfo, call: &mut Call) -> Result<bool, WantedMask> {
        if !self.cfg.noisy {
            return Ok(true)
        }
        if !self.cfg.ctors_included && state.fd.is_invalid() {
            return Ok(true)
        }
        if 0 != self.cfg.mask && !(CallTableId::Id(self.cfg.mask) & call.id()) {
            return Ok(true)
        }
        if self.cfg.only_successfull && !call.ok() {
            return Ok(true) //as this is pre-callback call.ok() will get us print second time call is hit after it suceed
        }
        println!("[d]call : {:?} <{:?}> [fd:{:?} | {:?}]", call.name(), state.name, state.fd, call.success());
        return Ok(true)
    }
}

impl Debug {
    pub(crate) fn new(cfg: &DebugConfig) -> Debug {
        Debug { cfg: *cfg }
    }
}

pub fn observers(
    cfg: &Option<DebugConfig>,
) -> (
    Option<Box<dyn IStateObserver>>,
    Option<Box<dyn ICallObserver>>,
) {
    match *cfg {
        Some(ref cfg) => (None, Some(Box::new(Debug::new(&cfg)))),
        _ => (None, None),
    }
}
