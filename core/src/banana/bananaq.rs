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
    if let Ok(mut banana) = FUZZY_QUEUE.write() {
        banana.observers_call.push(obs)
    }
}
pub fn attach_state_observer(obs: Box<dyn IStateObserver>) {
    if let Ok(mut banana) = FUZZY_QUEUE.write() {
        banana.observers_state.push(obs)
    }
}
pub fn detach_observers() {
    if let Ok(mut banana) = FUZZY_QUEUE.write() {
        banana.observers_call.clear();
        banana.observers_state.clear();
    }
}

pub fn empty() -> bool {
    FUZZY_QUEUE.read().unwrap().empty()
}
pub fn push(fuzzy_obj: &Box<dyn IFuzzyObj>) -> bool {
    if let Ok(mut banana) = FUZZY_QUEUE.write() {
        banana.push_safe(fuzzy_obj.state().info())
    } else { false }
}
pub fn pop() {
    if let Ok(banana) = FUZZY_QUEUE.read() {
        banana.dtor_notify_safe()
    }
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
    if let Ok(banana) = FUZZY_QUEUE.read() {
        banana.ctor_notify_safe(fuzzy_obj.state().info())
    } else { false }
}
pub fn call_notify<'a>(call: &'a mut Call) -> bool {
    if let Ok(banana) = FUZZY_QUEUE.read() {
        banana.call_notify_safe(call)
    } else { false }
}

pub fn call_aftermath<'a>(call: &'a mut Call) {
    if let Ok(banana) = FUZZY_QUEUE.read() {
        banana.call_aftermath_safe(call)
    }
}

pub fn get_rnd_fd(id: StateTableId) -> Fd {
    if let Ok(banana) = FUZZY_QUEUE.read() {
        banana.get_rnd_fd_safe(id)
    } else { Fd::empty() }
}
