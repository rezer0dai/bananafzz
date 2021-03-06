use std::{
    thread,
    rc::Rc,
    sync::RwLock,
    collections::HashMap,
};

extern crate rand;
use rand::{Rng, seq::SliceRandom};

use super::observer::{
    ICallObserver,
    IStateObserver,
};
use exec::call::Call;
use exec::fd_info::Fd;
use state::id::StateTableId;
use state::state::StateInfo;

use config::FuzzyConfig;

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

    states: HashMap< thread::ThreadId, StateInfo >,
    pub(crate) observers_state: Vec< Box<dyn IStateObserver> >,
    pub(crate) observers_call: Vec< Box<dyn ICallObserver> >,
}

unsafe impl Send for FuzzyQ {}
unsafe impl Sync for FuzzyQ {}

impl FuzzyQ {
    pub fn new(config: FuzzyConfig) -> FuzzyQ {
        FuzzyQ {
            cfg : config,

            qid : rand::thread_rng().gen(),
            active : Rc::new(RwLock::new(true)),

            states : HashMap::new(),
            observers_state : Vec::new(),
            observers_call : Vec::new(),
        }
    }

    pub(crate) fn qid(&self) -> u64 { self.qid }
    pub(crate) fn len(&self) -> usize { self.states.len() }
    pub(crate) fn active(&self) -> bool { *self.active.read().unwrap() }
    pub(crate) fn stop(&self) { *self.active.write().unwrap() = false; }

    /// certain calls want to intercorporate foreign state
    ///
    /// therefore we choose randomly from our queue ( based on criteria of caller )
    pub fn get_rnd_fd(&self, id: StateTableId) -> Fd {
        assert!(0 != self.states.len(), "[bananafzz] get_rnd_safe queried while no state in queue, possible ?");
        let size = self.states.iter().next().unwrap().1.fd.data().len();
        match self.states
            .iter()
            //.filter(|info| id == info.1.id)
            .filter(|info| id.clone() & info.1.id.clone())
            .filter(|info| !info.1.fd.is_invalid())
//            .inspect(|info| println!(".............{:?} -> {:X}", id, info.1.fd))
            .collect::<Vec<_>>()
            .choose(&mut rand::thread_rng())
        {
            Some(info) => info.1.fd.clone(),
            None => Fd::empty(size),
        }
    }

    /// call callback
    pub fn call_notify<'a>(&self, call: &'a mut Call) -> bool {
        if !self.active() {
            return false
        }
        let info = &self.states[&thread::current().id()];
        self.observers_call
            .iter()
            .all(|obs| obs.notify(info, call))
    }
    pub fn call_aftermath_safe<'a>(&self, info: &StateInfo, call: &'a mut Call) {
        if !self.active() {
            return
        }
        for obs in self.observers_call.iter() { 
            obs.aftermath(info, call) 
        }
    }
    /// state destruction callback
    pub fn dtor_notify(&self) {
        if !self.active() {
            return
        }
        let info = &self.states[&thread::current().id()];
        for obs in self.observers_state.iter() {
            obs.notify_dtor(info);
        }
    }
    /// state creation callback
    ///
    /// - checking dups ( same state already in queue - limit from config -> how many to allow )
    pub fn ctor_notify(&self, info: StateInfo) -> bool {
        if !self.active() {
            return false
        }
        let dups = self.states
            .iter()
            .filter(|&(_, ref state)| (state.fd.equals(&info.fd) && (state.id.clone() & info.id.clone())))
            .count();
        if dups > self.cfg.max_racers_count {
            return false
        }
        self.observers_state
            .iter()
            .all(|obs| obs.notify_ctor(&info))
    }

    /// we fuzzing only one state in one thread!
    pub fn push_safe(&mut self, fuzzy_info: StateInfo) -> bool {
        if !self.active() {
            return false
        }
        let same_kind = self.states
            .iter()
            .filter(|&(_, ref state)| (state.id.clone() & fuzzy_info.id.clone()))
            .count();

        // forcing at least 1 object of its kind in queue is not necessary what we want, limit config expresivness
        if self.states.len() > self.cfg.max_queue_size {
            return false//0 != same_kind
        }

        //here we want to FOLD and check siblings count ( ratio-% per object!! )
        //pay attention that we are interested only in activated ones ?
        //or maybe double activation, and final activation after intialization ?
        //
        //ok seems strict check on all siblings is preferable!!
       
        // well rust, overflows are handled, kind of overkill geting here overlow checks - implmenting fuzzer not OS
        if same_kind * self.cfg.ratio > self.cfg.max_queue_size * 1 {
            return false
        }

        if self.states.contains_key(&thread::current().id()) {
            panic!("trying to insert from same thread twice++ -> {}", fuzzy_info.name);
        }

        self.states.insert(thread::current().id(), fuzzy_info);
        true
    }
    pub fn pop_safe(&mut self) {
        if !self.states.contains_key(&thread::current().id()) {
            panic!("trying to pop from same thread twice++ or from different thread at all");
        }
        self.states.remove(&thread::current().id());
    }
    pub fn update_safe(&mut self, fuzzy_info: &StateInfo) {
        if !self.active() {
            return
        }
        // here we maybe want to double check how many same "fd" are in queue, and limit it by config
        // but i did not encounter issue with this, so i am letting this pass void
        assert!(self.states.contains_key(&thread::current().id()));
        if let Some(info) = self.states.get_mut(&thread::current().id()) {
            *info = fuzzy_info.clone();
        }
    }
    pub fn empty(&self) -> bool {
        0 == self.states.len()
    }
}
