#![feature(integer_atomics)]

#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate core;

use core::exec::call::Call;
use core::exec::id::CallTableId;
use core::banana::observer::{ICallObserver, IStateObserver};
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

use std::collections::HashMap;


impl ICallObserver for Debug {
    fn notify(&self, state: &StateInfo, call: &mut Call) -> bool {
        if !self.cfg.noisy {
            return true;
        }
        if !self.cfg.ctors_included && state.fd.is_invalid() {
            return true;
        }
        if 0 != self.cfg.mask && !(CallTableId::Id(self.cfg.mask) & call.id()) {
            return true;
        }
        if self.cfg.only_successfull && !call.ok() {
            return true;//as this is pre-callback call.ok() will get us print second time call is hit after it suceed
            //also it will never print ctors in only_successfull mode...
            //debug modle is mostly to print out if all calls are called with frequency as expected
            //and if they are sucessfull more or less, for better analysis need separate analyze module
        }
println!("[A] TEMPORARY debug out args {:?} : {:?} [fd:{:?}] ", call.name(), call.dump_args(), state.fd);
//call.load_args( &call.dump_args().iter().map(|b| b + 1).collect::<Vec<u8>>() );
//println!("[B] TEMPORARY debug out args {:?} : {:?} [fd:{:?}] ", call.name(), call.dump_args(), state.fd);
//call.load_args( &call.dump_args().iter().map(|b| b - 1).collect::<Vec<u8>>() );

let fd_lookup = HashMap::new();
call.load_args( 
    &call.dump_args().iter().map(|b| *b).collect::<Vec<u8>>(),
    &call.dump_mem(),
    &fd_lookup
    );

        println!("[d]call : {:?} <{:?}> [fd:{:?} | {:?}]", call.name(), state.name, state.fd, call.success());
        true
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
