use std::backtrace::Backtrace;

use super::observer::{
    ICallObserver,
    IStateObserver,
};

use super::queue;
use super::super::state::id::StateTableId;
use super::super::exec::fd_info::Fd;
use super::super::config::FuzzyConfig;

use exec::call::Call;
use state::state::{IFuzzyObj, StateInfo};

use std::sync::{Arc, Weak, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub type FuzzyQ = RwLock<queue::FuzzyQ>;

fn read_prot<F, T>(banana: &Weak<FuzzyQ>, action: F) -> Result<T, &'static str>
where F: Fn(&RwLockReadGuard<queue::FuzzyQ>) -> T
{
    if let Some(banana) = banana.upgrade() {
        if let Ok(banana) = banana.read() {
            return Ok(action(&banana))
        }
    }
    Err("[fuzzing] main bananaq droped main reference")
}

fn write_prot<F, T>(banana: &Weak<FuzzyQ>, mut action: F) -> Result<T, &'static str>
where F: FnMut(&mut RwLockWriteGuard<queue::FuzzyQ>) -> T
{
    if let Some(banana) = banana.upgrade() {
        if let Ok(mut banana) = banana.write() {
            return Ok(action(&mut banana))
        }
    }
    Err("[fuzzing] main bananaq droped main reference")
}

pub fn ping(banana: &Weak<FuzzyQ>) -> Result<(), &'static str> {
    write_prot(banana, |_| Err("ALL GOOD"))?
}

pub fn attach_call_observer(banana: &mut Arc<FuzzyQ>, obs: Box<dyn ICallObserver>) {
    Arc::get_mut(banana).unwrap().write().unwrap().observers_call.push(obs)
}
pub fn attach_state_observer(banana: &mut Arc<FuzzyQ>, obs: Box<dyn IStateObserver>) {
    Arc::get_mut(banana).unwrap().write().unwrap().observers_state.push(obs)
}

pub fn push<'a>(banana: &Weak<FuzzyQ>, fuzzy_obj: &Box<dyn IFuzzyObj>) -> Result<bool, &'static str> {
    write_prot(banana, |banana| banana.push_safe(fuzzy_obj.state().info()))
}

pub fn dtor<'a>(banana: &Weak<FuzzyQ>) -> Result<(), &'static str> {
    read_prot(banana, |banana| banana.dtor_notify())?;
    write_prot(banana, |banana| banana.pop_safe())
}
pub fn update(info: &StateInfo) -> Result<(), &'static str> {
    write_prot(&info.bananaq, |banana| banana.update_safe(info))
}

pub fn call_aftermath<'a>(info: &mut StateInfo, call: &'a mut Call) -> Result<(), &'static str> {
    update(info)?;
    if let Some(banana) = info.bananaq.upgrade() {
        return Ok(banana.read().unwrap().call_aftermath_safe(info, call))
    }
    Err("[bananaq] aftermath after bananaq gone")
}

pub fn call_notify<'a>(banana: &Weak<FuzzyQ>, call: &'a mut Call) -> bool {
    if let Some(banana) = banana.upgrade() {
        return banana.read().unwrap().call_notify(call)
    }
    false
}
pub fn ctor_notify<'a>(info: StateInfo) -> bool {
    if let Some(banana) = info.bananaq.upgrade() {
        return banana.read().unwrap().ctor_notify(info)
    }
    false
}
pub fn get_rnd_fd(banana: &Weak<FuzzyQ>, id: StateTableId) -> Result<Fd, &'static str> {
    read_prot(banana, |banana| banana.get_rnd_fd(id))
}
pub fn is_active(bananaq: &Weak<FuzzyQ>) -> Result<bool, &'static str> {
    read_prot(bananaq, |banana| banana.active())
}
pub fn config(bananaq: &Weak<FuzzyQ>) -> Result<FuzzyConfig, &'static str> {
    read_prot(bananaq, |banana| banana.cfg.clone())
}
pub fn stop(banana: &Weak<FuzzyQ>) -> Result<(), &'static str> {
    if config(banana)?.noisy {
        println!("[bananaq] QUEUE STOP: {:?}", Backtrace::force_capture()
            .frames()
            .iter()
            .enumerate()
            .filter(|(i, _)| (3..=4).contains(i))
            .map(|(_, s)| format!("{:?}", s))
            .collect::<Vec<String>>()
////            .nth(3)
//            .enumerate()
//            .map(|(i, s)| format!("\n [{i}] <{s:?}>"))
//            .collect::<Vec<String>>()
//            .join("\n")
            );
    }
    read_prot(&banana, |banana| banana.stop())
}

pub fn len(bananaq: &Weak<FuzzyQ>) -> Result<usize, &'static str> {
    read_prot(bananaq, |banana| banana.len())
}
pub fn qid(bananaq: &Weak<FuzzyQ>) -> Result<u64, &'static str> {
    read_prot(bananaq, |banana| banana.qid())
}
