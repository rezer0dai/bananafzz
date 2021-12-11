use std::sync::RwLock;

use super::observer::{
    ICallObserver,
    IStateObserver,
};
use exec::call::Call;
use exec::fd_info::Fd;
use state::id::StateTableId;
use state::state::IFuzzyObj;
use super::queue::FuzzyQ;

lazy_static! {
    /// sync primitive around Queue for fuzzing, singleton concept - for more better to check
    /// queue.rs instead
    static ref FUZZY_QUEUE: RwLock<FuzzyQ> = RwLock::new(FuzzyQ::new());
}

pub fn attach_call_observer(obs: Box<dyn ICallObserver>) {
    match FUZZY_QUEUE.write() {
        Ok(mut banana) => banana.observers_call.push(obs),
        Err(_) => (),
    };
}
pub fn attach_state_observer(obs: Box<dyn IStateObserver>) {
    match FUZZY_QUEUE.write() {
        Ok(mut banana) => banana.observers_state.push(obs),
        Err(_) => (),
    };
}

pub fn push(fuzzy_obj: &Box<dyn IFuzzyObj>) -> bool {
    match FUZZY_QUEUE.write() {
        Ok(mut banana) => banana.push_safe(fuzzy_obj.state().info()),
        Err(_) => false,
    }
}
pub fn pop() {
    match FUZZY_QUEUE.read() {
        Ok(banana) => banana.dtor_notify_safe(),
        Err(_) => (),
    };
    match FUZZY_QUEUE.write() {
        Ok(mut banana) => banana.pop_safe(),
        Err(e) => panic!("FuzzyQ: pop fail, syscall excepted .. no more to do here {}", e)
    };
}
pub fn update(fuzzy_obj: &Box<dyn IFuzzyObj>) {
    match FUZZY_QUEUE.write() {
        Ok(mut banana) => banana.update_safe(fuzzy_obj.state().info()),
        Err(e) => panic!("FuzzyQ: update fail, syscall excepted .. no more to do here {}", e)
    };
}

pub fn ctor_notify(fuzzy_obj: &Box<dyn IFuzzyObj>) -> bool {
    match FUZZY_QUEUE.read() {
        Ok(banana) => banana.ctor_notify_safe(fuzzy_obj.state().info()),
        Err(_) => false,
    }
}
pub fn call_notify<'a>(call: &'a Call) -> bool {
    match FUZZY_QUEUE.read() {
        Ok(banana) => banana.call_notify_safe(call),
        Err(_) => false,
    }
}

pub fn get_rnd_fd(id: StateTableId) -> Fd {
    match FUZZY_QUEUE.read() {
        Ok(banana) => banana.get_rnd_fd_safe(id),
        Err(_) => Fd::empty(),
    }
}
