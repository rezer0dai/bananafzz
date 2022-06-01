use std::{collections::HashMap, rc::Rc, sync::{Arc, Condvar, Mutex, RwLock}, thread};
use log::{debug, trace};

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
    active: Rc<RwLock<bool>>,

    wanted: Arc<(Mutex<WantedMask>, Condvar)>,

    states: HashMap<u64, StateInfo>,
    pub(crate) observers_state: Vec<Box<dyn IStateObserver>>,
    pub(crate) observers_call: Vec<Box<dyn ICallObserver>>,
}

unsafe impl Send for FuzzyQ {}
unsafe impl Sync for FuzzyQ {}

impl FuzzyQ {
    pub fn new(config: FuzzyConfig) -> FuzzyQ {
        FuzzyQ {
            cfg: config,

            qid: rand::thread_rng().gen(),
            active: Rc::new(RwLock::new(true)),

            wanted: Arc::new((Mutex::new(WantedMask::default()), Condvar::new())),

            states: HashMap::new(),
            observers_state: Vec::new(),
            observers_call: Vec::new(),
        }
    }

    pub(crate) fn wait_for(
        wanted: Arc<(Mutex<WantedMask>, Condvar)>,
        uid: u64,
        sid: u64,
        wait_max: u64
        ) -> Result<u64, ()> 
    {
        let (ref lock, ref cvar) = &*wanted;
        //println!("----> wait for up {} vs {}", info.uid, uid);
        let mut info = cvar.wait_timeout_while(
                lock.lock().or_else(|_| Err(()))?, 
                // .. maybe even nanos .. ?`
                std::time::Duration::from_millis(wait_max),
                |mask| { 
                    // no specific uid and matching mask sid or sid is no important
                    !((0 == mask.uid && (0 == mask.sid || 0 != sid & mask.sid))
                        // or we are interested in specific uid!!
                        || uid == mask.uid)
                }
                ).or_else(|_| Err(()))?.0;

        //println!("----> waked up {} vs {}", info.uid, uid);
        info.uid = uid;
        Ok(info.cid)
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
        ) -> Result<(), ()>
    {
        let (ref lock, ref cvar) = &*self.wanted;
        if let Ok(mut info) = lock.lock() {
            *info = mask
        } else { return Err(()) }

        if 0 == n_cores {
            cvar.notify_all();
        }

        for _ in 0..n_cores { // number of cores for fuzzing
            cvar.notify_one(); // resume selection
        }
        Ok(())
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
        *self.active.write().unwrap() = false;
        // ok lets other notify to quit
        let _ = self.wake_up(WantedMask::default(), 0);
    }

    /// certain calls want to intercorporate foreign state
    ///
    /// therefore we choose randomly from our queue ( based on criteria of caller )
    pub fn get_rnd_fd(&self, id: StateTableId, size: usize) -> Fd {
        assert!(
            0 != self.states.len(),
            "[bananafzz] get_rnd_safe queried while no state in queue, possible ?"
        );
        match self
            .states
            .iter()
            // OK we will filter only full covered IDs ( avoid partially & )
            .filter(|info| info.1.id.do_match(&id)
            // unicorns are special, we allo non-partial finds
                || (info.1.id.is_unicorn() && id.is_unicorn()))
            .filter(|info| !info.1.fd.is_invalid())
            //.inspect(|info| println!(".............{:?} -> {:X}", id, info.1.fd))
            .collect::<Vec<_>>()
            .choose(&mut rand::thread_rng())
        {
            Some(info) => info.1.fd.clone(),
            None => Fd::empty(size),
        }
    }

    /// call callback
    pub fn call_notify<'a>(&self, call: &'a mut Call) -> Result<bool, WantedMask> {
        if !self.active() {
            return Ok(false)
        }

        // add some competition to current thread
        if !self.wake_up(WantedMask::default(), 1).is_ok() {
            return Ok(false)
        }

        let uid = thread::current().id();
        let uid = u64::from(uid.as_u64());
        let info = &self.states[&uid];
        for obs in self.observers_call.iter() {
            if !obs.notify(info, call)? {
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
        let _ = self.wake_up(WantedMask::default(), 1);
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
debug!("QUEUE is FULL, denying entry of {:?}", fuzzy_info.id);
            return false; //0 != same_kind
        }
        let same_kind = self
            .states
            .iter()
            // filter out unicorns
            .filter(|&(_, ref state)| !(state.id.clone() & StateTableId::Id(1)))
            // count just same kind
            .filter(|&(_, ref state)| (state.id.clone() & fuzzy_info.id.clone()))
            .count();

        // forcing at least 1 object of its kind in queue is not necessary what we want, limit config expresivness

        //here we want to FOLD and check siblings count ( ratio-% per object!! )
        //pay attention that we are interested only in activated ones ?
        //or maybe double activation, and final activation after intialization ?
        //
        //ok seems strict check on all siblings is preferable!!

        // well rust, overflows are handled, kind of overkill geting here overlow checks - implmenting fuzzer not OS
        if StateTableId::Id(1) & fuzzy_info.id.clone() { // unicorn
            if self.cfg.unicorn_kin_limit < self
                .states
                .iter()
                .filter(|&(_, ref state)| (state.id.clone() & StateTableId::Id(1)))
                .filter(|&(_, ref state)| (state.id.clone() & fuzzy_info.id.clone()))
                .count()
            { return false }
        } else if same_kind * self.cfg.ratio > self.cfg.max_queue_size * 1 {
trace!("QUEUE is overpopulated of same kind, denying entry of {:?}", fuzzy_info.id);
            return false;
        }

        let uid = thread::current().id();
        let uid = u64::from(uid.as_u64());
        if self.states.contains_key(&uid) {
            panic!(
                "trying to insert from same thread twice++ -> {}",
                fuzzy_info.name
            );
        }

        self.states.insert(uid, fuzzy_info);
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
