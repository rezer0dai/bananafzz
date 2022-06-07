use std::backtrace::Backtrace;

use super::observer::{ICallObserver, IStateObserver};

use super::super::config::FuzzyConfig;
use super::super::exec::fd_info::Fd;
use super::super::state::id::StateTableId;
use super::queue;

use exec::call::Call;
use state::state::{IFuzzyObj, StateInfo};

use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard, Weak};

pub type FuzzyQ = RwLock<queue::FuzzyQ>;

fn read_prot<F, T>(banana: &Weak<FuzzyQ>, action: F) -> Result<T, &'static str>
where
    F: Fn(&RwLockReadGuard<queue::FuzzyQ>) -> T,
{
    if let Some(banana) = banana.upgrade() {
        if let Ok(banana) = banana.read() {
            return Ok(action(&banana));
        }
    }
    Err("[fuzzing] main bananaq droped main reference")
}

fn write_prot<F, T>(banana: &Weak<FuzzyQ>, mut action: F) -> Result<T, &'static str>
where
    F: FnMut(&mut RwLockWriteGuard<queue::FuzzyQ>) -> T,
{
    if let Some(banana) = banana.upgrade() {
        loop {
            if let Ok(mut banana) = banana.try_write() {
                return Ok(action(&mut banana));
            }
        }
    }
    Err("[fuzzing] main bananaq droped main reference")
}

pub fn ping(banana: &Weak<FuzzyQ>) -> Result<(), &'static str> {
    write_prot(banana, |_| Err("ALL GOOD"))?
}

pub fn attach_call_observer(banana: &mut Arc<FuzzyQ>, obs: Box<dyn ICallObserver>) {
    Arc::get_mut(banana)
        .unwrap()
        .write()
        .unwrap()
        .observers_call
        .push(obs)
}
pub fn attach_state_observer(banana: &mut Arc<FuzzyQ>, obs: Box<dyn IStateObserver>) {
    Arc::get_mut(banana)
        .unwrap()
        .write()
        .unwrap()
        .observers_state
        .push(obs)
}

pub fn push<'a>(
    banana: &Weak<FuzzyQ>,
    fuzzy_obj: &Box<dyn IFuzzyObj>,
) -> Result<bool, &'static str> {
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
        return Ok(banana.read().unwrap().call_aftermath_safe(info, call));
    }
    Err("[bananaq] aftermath after bananaq gone")
}

pub fn call_notify<'a>(banana: &Weak<FuzzyQ>, call: &'a mut Call) -> bool {
    log::trace!("#");
//    read_prot(banana, |banana| banana.wake_up(WantedMask::default(), 1));
    // go for this call
    //loop {
        //print!(".");
        let (cvar, uid, sid, wait_max, n_cores) = 
            if let Some(banana) = banana.upgrade() {
                let banana = banana.read().unwrap();
                let(cvar, uid, sid, n_cores, wait_max) = banana.land_line();
                // propose target from modules
                (cvar, uid, sid, wait_max, n_cores)
            } else { return false };

        if 0 != uid 
            && !read_prot(banana, |banana| banana.contains(uid)).unwrap_or(true)
        { let _ = stop(banana); }// module will be stupborn, but seems object is no longer active .. }

        //println!("@[{uid} || {sid:X} ==> tid:{:?}];", thread::current().id());
        
        if let Ok(oracle) = queue::FuzzyQ::wait_for(cvar, uid, sid, wait_max) {
            call.set_oracle(oracle)
        } else { return false }

        if let Some(banana) = banana.upgrade() {
            let banana = banana.read().unwrap();
            if !banana.active() {
                return false
            }
            match banana.call_notify(call) {
                Ok(ok) => return ok,
                Err(mask) => {
                    if 0 != mask.mid {
                        banana.wake_up(mask, n_cores)
                    }
                    if uid == mask.uid {
                        call.set_oracle(mask.cid)
                    }
                    //println!("---> {mask:?} ==> tid:{uid:?} + sid:{sid:?}];");
                }
            }
        }
    //}
    false
}
pub fn ctor_notify<'a>(info: StateInfo) -> bool {
    if let Some(banana) = info.bananaq.upgrade() {
        return banana.read().unwrap().ctor_notify(info)
    }
    false
}
pub fn get_rnd_fd(
    banana: &Weak<FuzzyQ>,
    id: StateTableId,
    size: usize,
) -> Result<Fd, &'static str> {
    read_prot(banana, |banana| banana.get_rnd_fd(id.de_horn(), size))
}
pub fn is_active(bananaq: &Weak<FuzzyQ>) -> Result<bool, &'static str> {
    read_prot(bananaq, |banana| banana.active())
}
pub fn config(bananaq: &Weak<FuzzyQ>) -> Result<FuzzyConfig, &'static str> {
    read_prot(bananaq, |banana| banana.cfg.clone())
}
pub fn stop(banana: &Weak<FuzzyQ>) -> Result<(), &'static str> {
    log::info!(
        "[bananaq] QUEUE STOP: {:?}",
        Backtrace::force_capture()
            .frames()
            .iter()
            .enumerate()
            .filter(|(i, _)| (3..=4).contains(i))
            .map(|(_, s)| format!("{:?}", s))
            .collect::<Vec<String>>() ////            .nth(3)
                                      //            .enumerate()
                                      //            .map(|(i, s)| format!("\n [{i}] <{s:?}>"))
                                      //            .collect::<Vec<String>>()
                                      //            .join("\n")
    );
    read_prot(&banana, |banana| banana.stop())
}

pub fn len(bananaq: &Weak<FuzzyQ>) -> Result<usize, &'static str> {
    read_prot(bananaq, |banana| banana.len())
}
pub fn qid(bananaq: &Weak<FuzzyQ>) -> Result<u64, &'static str> {
    read_prot(bananaq, |banana| banana.qid())
}
pub fn timestamp(bananaq: &Weak<FuzzyQ>) -> Result<u64, &'static str> {
    read_prot(bananaq, |banana| banana.timestamp())
}
