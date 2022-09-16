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
            // move wanted here trough arc+rwlock very likely
        }
        impl Proxy {
            fn new(lookup: Arc<RwLock<$name>>) -> Proxy {
                Proxy { lookup: lookup }
            }
            fn skip_if_wanted(&self, state: &StateInfo) -> Result<bool, WantedMask> {
                self.lookup
                    .write()
                    .map_or(Ok(false), |mut target| 
                        if target.wanted.is_some() {
                            let out = if let Some(ref mask) = target.wanted {
                                if 0 != mask.uid && state.uid() != mask.uid {
                                    Err(mask.clone())
                                } else if 0 != mask.sid && 0 == u64::from(state.id) & mask.sid {
                                    Err(mask.clone())
                                } else { Ok(true) }
                            } else { Ok(true) };
                            target.wanted = None;
                            out
                        } else { Ok(true) }
                    )
            }
        }
        impl ICallObserver for Proxy {
            fn notify(&self, state: &StateInfo, call: &mut Call) -> Result<bool, WantedMask> {
                self.skip_if_wanted(state)?;
                self.lookup
                    .write()
                    .map_or(Ok(false), |mut target| target.notify(state, call))
            }
            fn aftermath(&self, state: &StateInfo, call: &mut Call) {
                if let Ok(mut target) = self.lookup.write() {
                    let _ = target.aftermath(state, call);
                }
            }
            fn revert(&self, info: &StateInfo, call: &Call, mask: WantedMask) {
                self.lookup
                    .write()
                    .map_or((), |mut target| {
                        let _ = target.wanted.insert(mask);
                        target.revert(info, call, mask)
                    })
            }
            fn stop(&self) {
                self.lookup
                    .write()
                    .map_or((), |mut target| target.stop())
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
