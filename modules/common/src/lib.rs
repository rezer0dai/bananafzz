extern crate core;
use self::core::exec::fd_info::Fd;
use self::core::state::id::StateTableId;

pub type TPushState = &'static (dyn Fn(StateTableId, &Fd) + std::marker::Sync + std::marker::Send);

pub trait ModuleCallbacks: Send + Sync {
    fn push_state(&self) -> TPushState;
    fn read_log(&self) -> String;
    fn log_call(&self, cmd: String, info: &str);
    fn stop_fuzzing(&self);
}

#[macro_export]
macro_rules! callback_proxy {
    ($name:ident) => {
        use std::sync::{Arc, RwLock};
        struct Proxy {
            lookup: Arc<RwLock<$name>>,
        }
        impl Proxy {
            fn new(lookup: Arc<RwLock<$name>>) -> Proxy {
                Proxy { lookup: lookup }
            }
        }
        impl ICallObserver for Proxy {
            fn notify(&self, state: &StateInfo, call: &mut Call) -> Result<bool, WantedMask> {
                self.lookup
                    .write()
                    .map_or(Ok(false), |mut target| target.notify(state, call))
            }
            fn aftermath(&self, state: &StateInfo, call: &mut Call) {
                if let Ok(mut target) = self.lookup.write() {
                    target.aftermath(state, call);
                }
            }
            fn revert(&self, info: &StateInfo, call: &Call, mask: WantedMask) {
                self.lookup
                    .write()
                    .map_or((), |mut target| target.revert(info, call, mask))
            }
        }
        impl IStateObserver for Proxy {
            fn notify_ctor(&self, state: &StateInfo) -> bool {
                self.lookup
                    .write()
                    .map_or(false, |mut target| target.ctor(state))
            }
            fn notify_dtor(&self, state: &StateInfo) {
                self.lookup
                    .write()
                    .map_or((), |mut target| target.dtor(state))
            }
        }
    };
}
