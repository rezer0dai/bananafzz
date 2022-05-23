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
        struct Proxy {
            lookup: Rc<RwLock<$name>>,
        }
        impl Proxy {
            fn new(lookup: Rc<RwLock<$name>>) -> Proxy {
                Proxy { lookup: lookup }
            }
        }
        impl ICallObserver for Proxy {
            fn notify(&self, state: &StateInfo, call: &mut Call) -> bool {
                self.lookup
                    .write()
                    .map_or(false, |mut target| target.notify(state, call))
            }
            fn aftermath(&self, state: &StateInfo, call: &mut Call) {
                if let Ok(mut target) = self.lookup.write() {
                    target.aftermath(state, call);
                }
            }
        }
        impl IStateObserver for Proxy {
            fn notify_ctor(&self, state: &StateInfo) -> bool {
                self.lookup
                    .write()
                    .map_or(false, |mut target| target.ctor(state))
            }
        }
    };
}
