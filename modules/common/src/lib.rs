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
