use std;

extern crate core;
use core::state::id::StateTableId;
use core::exec::fd_info::Fd;

use core::config::FZZCONFIG;

extern crate common;
use self::common::{
    ModuleCallbacks,
    TPushState,
};

//extern crate libpoclog;
//extern crate libsyzkaller;

#[derive(Clone)]
pub struct PlugCallbacks {
    callback: TPushState,
}
impl PlugCallbacks {
    pub fn new<F>(push_state: &'static F) -> PlugCallbacks
        where F: Fn(StateTableId, &Fd) + std::marker::Sync + std::marker::Send
    {
        PlugCallbacks {
            callback : push_state,
        }
    }
}
impl ModuleCallbacks for PlugCallbacks {
    fn push_state(&self) -> TPushState { self.callback }

    fn read_log(&self) -> String {
//        libpoclog::logger::Logger::log()
      String::from("")
    }
    fn log_call(&self, _cmd: String, _info: &str) {
//        libpoclog::logger::Logger::safe_log(
//            libpoclog::decorate(cmd, info))
    }
    fn stop_fuzzing(&self) {
//        libsyzkaller::SyzKaller::finish();
//        libpoclog::logger::Logger::flush();
        if FZZCONFIG.noisy {
            println!("[fuzzing] DONE");
        }
        std::process::exit(0)
    }
}
