use std::{collections::HashMap, rc::Rc, sync::{Arc, Condvar, Mutex, RwLock}, thread};
use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
};
use log::{debug, trace, warn};

extern crate rand;
use rand::{seq::SliceRandom, Rng};

use super::observer::{ICallObserver, IStateObserver};
use exec::call::Call;
use exec::fd_info::Fd;
use state::id::StateTableId;
use state::state::StateInfo;

use config::FuzzyConfig;

pub use super::observer::WantedMask;

/// central structure(queue) for fuzzing - internal fuzzer manager/banana
///
/// - register all states and all call invocations
/// - firing callbacks to observers
/// - checking duplicates
/// - can be enforced single thread - ability to fuzzing trough config
/// - must be sync-ed
pub struct FuzzyQ {
    /// basic information per state
    ///
    /// - duplicate resolving
    /// - callback forwarding
    pub(crate) cfg: FuzzyConfig,

    qid: u64,
    timestamp: AtomicU64,

    started: AtomicBool,
    active: Rc<RwLock<bool>>,

    wanted: Arc<(Mutex<WantedMask>, Condvar)>,

    states: HashMap<u64, StateInfo>,
    pub(crate) observers_state: Vec<Box<dyn IStateObserver>>,
    pub(crate) observers_call: Vec<Box<dyn ICallObserver>>,
}

unsafe impl Send for FuzzyQ {}
unsafe impl Sync for FuzzyQ {}

impl FuzzyQ {
    pub fn new(config: FuzzyConfig, start: bool) -> FuzzyQ {
        FuzzyQ {
            cfg: config,

            qid: rand::thread_rng().gen(),
            active: Rc::new(RwLock::new(true)),

            wanted: Arc::new((Mutex::new(WantedMask::default()), Condvar::new())),
            timestamp: AtomicU64::new(clock_ticks::precise_time_ms()),
            started: AtomicBool::new(start),

            states: HashMap::new(),
            observers_state: Vec::new(),
            observers_call: Vec::new(),
        }
    }

    pub fn start(&mut self) {
        self.started.store(true, Ordering::SeqCst)
    }
    pub fn started(&self) -> bool {
        self.started.load(Ordering::SeqCst)
    }

    pub(crate) fn wait_for(
        wanted: Arc<(Mutex<WantedMask>, Condvar)>,
        uid: u64,
        sid: u64,
        wait_max: u64
        ) -> Result<u64, ()> 
    {
//        std::thread::sleep(// avoid retaking thread just by sheer opportunity
//            std::time::Duration::from_millis(wait_max));

        std::thread::yield_now();

        let (ref lock, ref cvar) = &*wanted;
        //println!("----> wait for up {} vs {}", info.uid, uid);
        let mut info = cvar.wait_timeout_while(
                lock.lock().or_else(|_| Err(()))?, 
                // .. maybe even nanos .. ?`
                std::time::Duration::from_millis(wait_max),
                |mask| { 
                    !((0 == mask.uid && 0 != sid & mask.sid) || uid == mask.uid)
                }
                ).or_else(|_| {
                    warn!("[queue#wait_for] timeout for uid:{uid}; sid:{sid}"); 
                    Err(())
        })?.0;

        trace!("----> [{}] waked up {} vs {} || sid : {} + cid : {} ", info.mid, info.uid, uid, info.sid, info.cid);
        let cid = if uid == info.uid {
            info.cid
        } else { 0 };
        if (uid == info.uid || 0 == info.uid) && sid == info.sid {
            info.sid = 0
        }
        if uid == info.uid {
            info.uid = 0
        }
        Ok(cid)
    }

    pub(crate) fn land_line(&self) -> (Arc<(Mutex<WantedMask>, Condvar)>, u64, u64, u64, u64) {
        let uid = thread::current().id();
        let uid = u64::from(uid.as_u64());
        let sid: u64 = if let Some(info) = self.states.get(&uid) {
            info.id.into()
        } else { 0 };
        (Arc::clone(&self.wanted), uid, sid, self.cfg.n_cores, self.cfg.wait_max)
    }

    pub(crate) fn wake_up(
        &self,
        mask: WantedMask,
        n_cores: u64,
        )
    {
        let (ref lock, ref cvar) = &*self.wanted;
        if let Ok(mut info) = lock.lock() {
            *info = mask
        }

        if 0 == n_cores {
            cvar.notify_all();
        }

        for _ in 0..n_cores { // number of cores for fuzzing
            cvar.notify_one(); // resume selection
        }
    }
    pub(crate) fn timestamp(&self) -> u64 {
        self.timestamp.load(Ordering::SeqCst)
    }
    pub(crate) fn qid(&self) -> u64 {
        self.qid
    }
    pub(crate) fn len(&self) -> usize {
        self.states.len()
    }
    pub(crate) fn active(&self) -> bool {
        *self.active.read().unwrap()
    }
    pub(crate) fn stop(&self) {
        log::info!("stop with time : {:?}", self.timestamp() - clock_ticks::precise_time_ms());
        *self.active.write().unwrap() = false;
        // ok lets other notify to quit
        //self.wake_up(WantedMask::default(), 0);
    }
    pub(crate) fn finish(&self) {
        for obs in &self.observers_call {
            obs.stop()
        }
    }

    /// certain calls want to intercorporate foreign state
    ///
    /// therefore we choose randomly from our queue ( based on criteria of caller ) 
    pub fn get_rnd_fd(&self, id: StateTableId) -> Option<(Fd, StateTableId)> {
        assert!(
            0 != self.states.len(),
            "[bananafzz] get_rnd_safe queried while no state in queue, possible ?"
        );
        self.states
            .iter()
            // OK we will filter only full covered IDs ( avoid partially & )
            .filter(|info| id.do_match(&info.1.id)//.do_match(&id)
            // unicorns are special, we allo non-partial finds
                || (info.1.id.is_unicorn() && id.is_unicorn()))
            .filter(|info| !info.1.fd.is_invalid())
            //.inspect(|info| println!(".............{:?} -> {:X}", id, info.1.fd))
            .collect::<Vec<_>>()
            .choose(&mut rand::thread_rng())
            .map(|info| (info.1.fd.clone(), info.1.id))
    }

    /// call callback
    pub fn call_notify<'a>(&self, call: &'a mut Call) -> Result<bool, WantedMask> {
/*
        if !self.active() {
            return Ok(false)
        }
        // add some competition to current thread
        if !self.wake_up(WantedMask::default(), 1).is_ok() {
            return Ok(false)
        }
*/
        self.timestamp.store(clock_ticks::precise_time_ms(), Ordering::SeqCst);

        let uid = thread::current().id();
        let uid = u64::from(uid.as_u64());
        let info = &self.states[&uid];

        match self.call_notify_exec(call, uid) {
            Ok(res) => {
                log::trace!("******* wakeup --> {}", self.states.len());
                //self.wake_up(WantedMask::default(), 1);
                Ok(res)
            }
            Err((n, mask)) => {
                log::trace!("[{n}] reverting base on {mask:?}, sid:{:?}", info.id);
                for obs in self.observers_call.iter().take(n) {
                    obs.revert(info, call, mask)
                }
                Err(mask)
            }
        }
    }
    fn call_notify_exec<'a>(
        &self, 
        call: &'a mut Call,
        uid: u64
        ) -> Result<bool, (usize, WantedMask)> 
    {
        let info = &self.states[&uid];
        for (i, obs) in self.observers_call.iter().enumerate() {
            if !obs.notify(info, call).or_else(|info| Err((i, info)))? {
                return Ok(false)
            }
        }
        Ok(true)
    }
    pub fn call_aftermath_safe<'a>(&self, info: &StateInfo, call: &'a mut Call) {
        if !self.active() {
            return;
        }
        for obs in self.observers_call.iter() {
            obs.aftermath(info, call)
        }
    }
    /// state destruction callback
    pub fn dtor_notify(&self) {
        if !self.active() {
            return;
        }
        let uid = thread::current().id();
        let uid = u64::from(uid.as_u64());

        let info = &self.states[&uid];
        for obs in self.observers_state.iter() {
            obs.notify_dtor(info);
        }
debug!("dtor tid:{uid} :: {}", info.name);
        //self.wake_up(WantedMask::default(), 0);
    }
    /// state creation callback
    ///
    /// - checking dups ( same state already in queue - limit from config -> how many to allow )
    pub fn ctor_notify(&self, info: StateInfo) -> bool {
        if !self.active() {
            return false;
        }
        let dups = self
            .states
            .iter()
            .filter(|&(_, ref state)| {
                state.fd.equals(&info.fd) && (state.id.clone() & info.id.clone())
            })
            .count();
        if dups > self.cfg.max_racers_count {
warn!("racers shuted down {:X}", u64::from(info.id));
            return false;
        }
        self.observers_state
            .iter()
            .all(|obs| obs.notify_ctor(&info))
    }

    /// we fuzzing only one state in one thread!
    pub fn push_safe(&mut self, fuzzy_info: StateInfo) -> bool {
        if !self.active() {
            return false;
        }
        if self.states.len() > self.cfg.max_queue_size {
debug!("QUEUE is FULL, denying entry of {:X} -- {}", u64::from(fuzzy_info.id), fuzzy_info.name);
            return false; //0 != same_kind
        }

        let uid = thread::current().id();
        let uid = u64::from(uid.as_u64());

        if !self.allow_pass(&fuzzy_info, uid) {
            return false
        }

        if self.states.contains_key(&uid) {
            panic!(
                "trying to insert from same thread twice++ -> {}",
                fuzzy_info.name
            );
        }

        self.states.insert(uid, fuzzy_info);
        true
    }
    fn allow_pass(&self, fuzzy_info: &StateInfo, uid: u64) -> bool {
        if 0 != fuzzy_info.level { // likely allows all racers and hard copies ( dormant ones )
            return true
        }

        let total_unicorns = self
            .states
            .iter()
            .filter(|&(_, ref state)| 0 != state.level) // only initialized ones
            // just unicorns total number!!
            .filter(|&(_, ref state)| state.id.is_unicorn())
            //.filter(|&(_, ref state)| (state.id.de_horn().clone() & fuzzy_info.id.de_horn().clone()))
            .count();

        // well rust, overflows are handled, kind of overkill geting here overlow checks - implmenting fuzzer not OS
        if fuzzy_info.id.is_unicorn() 
            && total_unicorns > self.cfg.unicorn_kin_limit 
        {
warn!("UNICORN DENY: {}", fuzzy_info.name);
            return false 
        }

        let same_kind = self
            .states
            .iter()
            // filter out racers & hardcopies - we are interested only at uniques
            //.filter(|&(_, ref state)| state.id.is_unicorn())
            //.filter(|&(_, ref state)| 0 != state.level) // only initialized ones
            // count just same kind
            // state.do_match(fuzzy) will does not count equally ( sub-fd will be pushed )
            //.filter(|&(_, ref state)| state.id.do_match(&fuzzy_info.id))
            // fuzzy.do_match(state) will DOES count equally ( sub-fd will NOT be pushed )
            //.filter(|&(_, ref state)| fuzzy_info.id.do_match(&state.id))
            .filter(|&(_, ref state)| state.id.core_flags() & fuzzy_info.id.core_flags())
            .count();

        if same_kind * self.cfg.ratio > self.cfg.max_queue_size {
warn!("QUEUE is overpopulated of same kind, [{}]({uid}) denying entry of {:X}/{:X} [same#{same_kind}", fuzzy_info.name, u64::from(fuzzy_info.id), u64::from(fuzzy_info.id.core_flags()));
            return false
        }

debug!("[{}]::({uid}) allowing entry of {:X}/{:X} [same#{same_kind}", fuzzy_info.name, u64::from(fuzzy_info.id), u64::from(fuzzy_info.id.core_flags()));

        true
    }
    pub fn pop_safe(&mut self) {
        if !self.active() {
            return;
        }
        let uid = thread::current().id();
        let uid = u64::from(uid.as_u64());
        if !self.states.contains_key(&uid) {
            panic!("trying to pop from same thread twice++ or from different thread at all");
        }
        self.states.remove(&uid);
    }
    pub fn update_safe(&mut self, fuzzy_info: &StateInfo) {
        if !self.active() {
            return;
        }
        // here we maybe want to double check how many same "fd" are in queue, and limit it by config
        // but i did not encounter issue with this, so i am letting this pass void
        let uid = thread::current().id();
        let uid = u64::from(uid.as_u64());
        assert!(self.states.contains_key(&uid));
        if let Some(info) = self.states.get_mut(&uid) {
            *info = fuzzy_info.clone();
        }
    }
    pub fn empty(&self) -> bool {
        0 == self.states.len()
    }
    pub fn contains(&self, uid: u64) -> bool {
        self.states.contains_key(&uid)
    }
}
