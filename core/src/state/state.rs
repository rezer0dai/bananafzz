use std::thread;
use std::cmp::min;

extern crate rand;
use rand::Rng;

use banana::bananaq;

use config::FZZCONFIG;

use exec::call::Call;
use exec::fd_info::Fd;
use super::id::StateTableId;

pub trait IFdState {
    fn invalid(&self) -> bool;
    /// once we want to share FD between processes / threads, we should be able to check if those are
    /// valid ( alive )
    ///
    /// - can be default = true, but better manualy craft per state
    ///     - you can enhance effectivity of fuzzing this way
    fn is_online(&mut self) -> bool;
}

/// expose interface for fuzzing framework - to fuzz & log & debug!
pub trait IFuzzyObj : Send + Sync + IFdState {//quite pitty that i can not do Box<IFuzzyObj + IFdState>...
    /// lets do one fuzzing loop over specific State
    ///
    /// - need to invoke do_fuzz_one -> will select and invoke (sys)call
    /// - need to invoke do_fuzz_update -> change state of current State ( allow to fuzz other layers)
    /// - in between place your fuzzy knowledge to boost fuzzing ( optionaly )
    /// # Example :
    /// ```
    /// impl IFuzzyObj for TestState {
    ///     fn fuzzy_loop(&mut self) -> bool {
    ///         while !self.state.do_fuzz_one() {
    ///         }
    ///
    ///         { let data: &test_struct = self.state.call_mut().args(0).read_unsafe(); }
    ///
    ///         println!(">>{}", self.state.call_view().serialize(self.info.fd.data()));
    ///         self.state.do_fuzz_update()
    ///     }
    /// }
    /// ```
    fn fuzzy_loop(&mut self) -> bool;
    /// will do essentialy same as fuzzy_loop, however is invoked only until state is not initialized
    fn fuzzy_init(&mut self) -> bool;
    /// we want forward information about current state + calls to callbacks
    ///
    /// - only immutable
    /// - mutable state *must* be only at queue worker who actually do fuzzing
    /// - therefore only one mutable point, rest are imutable ones!
    /// # Example :
    /// ```
    /// impl IFuzzyObj for TestState {
    ///     fn state(&self) -> &State {
    ///         &self.state
    ///     }
    /// }
    /// ```
    fn state(&self) -> &State;
}

/// sharable informations with FuzzyQueue (rnd fd arg, de-dups, ..) && with modules; cheap to copy
#[derive(Clone)]
pub struct StateInfo {
    pub name: String,//maybe too expensive to share, and can be skipped ?
    /// id is specific per fuzzing target ( vmwp, vmbus, packets, w32k, ntos, alpc, io, .. )
    pub id: StateTableId,
    /// runtime id of state! unique identifiable because of POC and building connections between state and for races
    pub fd: Fd,
    /// num of total fuzzing iterations performed
    pub total: usize,
    /// num of sucessfull ( syscall return OK value ) fuzzing iterations performed; debug reasons
    pub sucess: usize,
    /// level in ccache -> knowledge based fuzzing
    pub level: usize,
}

impl StateInfo {
    pub fn uid(&self) -> u64 { u64::from(thread::current().id().as_u64()) }
}

/// user mode state ( representation ) of target ( kernel object, remote object, io device, .. )
pub struct State {
    /// sharable informations with FuzzyQueue (rnd fd arg, de-dups, ..) && with modules
    info: StateInfo,
    /// hardcoded limit per object how many iteration of fuzzing to perform on it - then close
    limit: usize,
    /// how many times to stay at level if call is declined to call ( observer or fail )
    n_failed_notify_allowed: usize,
    /// one of the main features of state, we can here calibrate behaviour which (set of)function should be called when
    slopes: Vec<[isize; 2]>,
    /// we need to preserve state, which level and which call is fuzzed
    ///
    /// - after do_fuzz_one it implies that self.call_view() will return currently fuzzed object
    ///     - on this object was just performed syscall!
    /// - after do_fuzz_update, implies :
    ///     - self.level() points to current level ( updated by slopes )
    ///     - self.call_view() now is invalid!
    ccache: (usize, usize),
    /// 2 dimensional vector of target syscalls / apicalls / operations
    ///
    /// - grouped by researcher knowledge - he can make it SOTA fuzzer for target or limit it way too much and fail
    ///     - no more calls in group than 0x200! - (just empirical numero)
    /// - by def is this classic groups :
    ///     group1 : ctors => calls creating particular object in kernel
    ///     group2 : workers => calls working over this object
    ///     - can be grouped also with getters, checkers ( as those seems little impact, so waste of time to fuzz it with same prio as setters ? )
    ///     - then grouping is here to overcome complicated set-state scenarios, to help fuzzer in
    ///     blackbox settings!
    ///     - note that dtors are *not* in group!!
    groups: Vec< Vec<Call> >,
    /// at the end of fuzzing ( do_fuzz_update will say NO MORE ) is invoked dtor callback, should close resource in kernel / target
    ///
    /// - close, CloseHandle, DeleteDC, DeleteObject, ZwFreeVirtualMemory, ...
    /// - i experienced that state has always only one dtor - or can be dtored by only one call / packet
    ///     - if in future will get knowledge about breaking this rule, then vec![dtors] should be provided!
    dtor: Call,
}

impl State {
    pub fn name(&self) -> &str { &self.info.name }
    pub fn id(&self) -> StateTableId { self.info.id.clone() }

    pub fn info(&self) -> StateInfo { self.info.clone() }

    pub fn fd(&self) -> &Fd { &self.info.fd }
    pub fn level(&self) -> usize { self.ccache.0 }

    pub fn call_view(&self) -> &Call { &self.groups[self.ccache.0][self.ccache.1] }
    pub fn total(&self) -> usize { self.info.total }
    pub fn sucess(&self) -> usize { self.info.sucess }

    /// usually we dont know fd at creation of object, therefore after first OK syscalls should be initialized by IFuzzyObj holder
    ///
    /// - call per fuzzy obj / state in fuzzy_init exposed by trait owned by holder of this State!
    pub fn init(&mut self, fd: &Fd) { self.info.fd.init(&fd) }

    /// mainly because dummy + negate trick, other cases should be reconsidered because PoC
    /// generation legacy, you do something too fancy you are likely not able to repro in poc ...
    pub fn call_mut(&mut self) -> &mut Call { &mut self.groups[self.ccache.0][self.ccache.1] }

    /// select random one from current level
    ///
    /// - maybe intercorporate some ranking of OK syscalls - aka prefer some syscalls at current level
    ///     - heatmaps, reinforcement learning, other algorithm
    ///     - currently is opt-ed to do it via modules, but it is perf overkill
    ///     - therefore if once decide for particular algorithm should be implemented here and
    ///     switched by config
    /// - also some blacklisting of too often rejected call - very OK by modules
    /// - and maybe tracking uniformity of selection globaly per State - forbid prefering one syscall by thread_rng ?
    ///     - in general this can be crucial part of fuzzer which i neglected to tinker with yet..
    pub fn do_fuzz_one(&mut self, shared: &mut[u8]) -> bool {
        if self.info.total > self.limit {
            return false
        }
        self.info.total += 1;
        let fd = self.info.fd.clone();
        for _ in 0..self.n_failed_notify_allowed {
            self.ccache.1 = rand::thread_rng().gen_range(0..self.groups[self.ccache.0].len());
            if !self.call_view().dead()
                && self.call_mut().do_call(&fd.data(), shared) 
            { return true }

//            assert!(!self.call_view().dead(), "CALL VIEW DEAD");

            if self.groups[self.ccache.0]
                .iter()
                .all(|ref call| call.dead()) { break }
        }
        self.call_dtor(shared);
        false
    }

    /// need to be called after do_fuzz_one, to change level based on slopes!
    ///
    /// - call once do_fuzz_one will return true
    /// - this basically wraps do_fuzz_update_impl, that it checks for end of fuzzing
    ///     - and then it performs closing of this State by calling dtor syscall!
    pub fn do_fuzz_update(&mut self, shared: &mut[u8]) -> bool {
        //maybe better weight it against #calls in current group
        //this way we trust a lot that every call is implemented correctly == good way
        //it is quick death then, like object is no longer online
        //however, if some syscall poorely implemented this will kill fuzzing for whole fuzzy object most
        // likely ...
        if self.do_fuzz_update_impl() {
            return true
        }
        self.call_dtor(shared);
        false
    }
    fn call_dtor(&mut self, shared: &mut[u8]) {
        if self.info.fd.is_invalid() {
            return
        }
        self.dtor.do_call(self.info.fd.data(), shared);
    }
    /// update slopes - state of current State, that we can proceed to fuzz next layer of syscalls
    fn do_fuzz_update_impl(&mut self) -> bool {
        let mut call = &mut self.groups[self.ccache.0][self.ccache.1];
        let ok = if call.ok() { 1 } else { 0 };

        self.ccache.0 = (self.ccache.0 as isize + self.slopes[self.ccache.0][ok]) as usize;
        self.ccache.1 = !0;//invalidate!! - now self.{c/m}call() is pretty much invalid!

        let level = self.info.level;
        self.info.sucess += ok;
        self.info.level = self.ccache.0;

        if 0 != level {//write locked callback
            bananaq::call_aftermath(&self.info, &mut call);
        } // for ctors we have notify_ctor at core/banana/looper.rs

        if self.info.total > self.limit {
            return false
        }
        if call.dead() && self.groups[self.ccache.0]
            .iter()
            .filter(|&call| call.dead())
            .count() * 2 > self.groups[self.ccache.0].len()
        {
            return false
        }

        true
    }

    /// TODO :
    ///
    /// here is how we will build our state by default :
    ///
    /// # Example
    /// ```
    /// struct TestState {
    ///     state: State,
    /// }
    ///
    /// impl TestState {
    ///     pub fn new() -> TestState {
    ///         TestState {
    ///             state : State::new(
    ///                 "test-state",
    ///                 StateTableId::Id(2),
    ///                 40,
    ///                 vec![[1, 0], [-1, -1]],
    ///                 vec![
    ///                     vec![
    ///                         Call::test_call()
    ///                     ],
    ///                     vec![
    ///                         Call::test_call(),
    ///                         Call::test_call()
    ///                     ]],
    ///                     Call::test_call())
    ///         }
    ///     }
    /// }
    /// ```
    /// later we want to manualy play with fuzzing at IFuzzyObj::fuzzy_loop
    ///
    /// - that will be our main point how to improve fuzzing by knowledge
    /// - basically we can set pre-callback in modules, and post-callback there
    /// - have unique state informations gathered
    /// - create arbitrary ( but connected ) objects at certain points of fuzzing
    /// - setting breakpoints, and optionally lead fuzzing
    ///     - leading of fuzzing maybe needed update bit core of this :
    ///     - ability to set self.ccache.1 and self.ccache.0 by will - for now i dont see a point
    ///     - ability to update self.call_view()->args()[..]->load_unsafe() data - also for now seems more over-engineering
    ///         - prefered dont touch arguments, and leave that for their generators
    ///         - by konwledge based approach do all work before+afer syscall is invoked, more
    ///         logic level leading
    pub fn new(
        name: &'static str,
        id: StateTableId,
        fd_size: usize,
        limit: usize,
        n_failed_notify_allowed: usize,
        slopes: Vec<[isize; 2]>,
        groups: Vec< Vec<Call> >,
        dtor: Call
        ) -> State
    {
        assert!(slopes.len() == groups.len());
        assert!(groups.iter().all(|ref group| group.len() < 0x200));

        if slopes.len() != groups.len() {
            panic!("slopes vs groups len problem at : {} => {} vs {}", name, slopes.len(), groups.len());
        }
        if !groups.iter().all(|ref group| group.len() < 0x200) {
            panic!("one of the group for {} is oversized!", name);
        }

        State {
            info : StateInfo {
                name : String::from(name),
                total : 0,
                sucess : 0,
                fd : Fd::empty(fd_size),
                id : id,
                level : 0,
            },
            limit : min(FZZCONFIG.new_limit, limit),
            n_failed_notify_allowed: n_failed_notify_allowed,
            slopes : slopes,
            groups : groups,
            dtor: dtor,
            ccache : (0, !0),
        }
    }
    /// apply as for new, but here we create already existing object :
    ///
    /// - introduce race conditions
    /// - some calls return more than one FD, so one of them we need create State by this way
    ///     - meaning : some calls create multiple states in target space / kernel / ..
    ///     - not necessary FD, sometimes can mean other than file descriptor only, but any runtime unique ID
    ///         - id of allocation, id of font in global table, crc32(string), runtime memory pointer, of some name uuid ? ...
    pub fn duped(
        name: &'static str,
        id: StateTableId,
        fd: &Fd,
        limit: usize,
        n_failed_notify_allowed: usize,
        slopes: Vec<[isize; 2]>,
        groups: Vec< Vec<Call> >,
        dtor: Call
        ) -> State
    {
        assert!(slopes.len() == groups.len());
        assert!(groups.iter().all(|ref group| group.len() < 0x200));

        if slopes.len() != groups.len() {
            panic!("slopes vs groups len problem at : {} => {} vs {}", name, slopes.len(), groups.len());
        }
        if !groups.iter().all(|ref group| group.len() < 0x200) {
            panic!("one of the group for {} is oversized!", name);
        }

        let level = slopes[0][1] as usize;

        State {
            info : StateInfo {
                name : String::from(name),
                total : 0,
                sucess : 0,
                fd : fd.clone(),
                id : id,
                level : level,
            },
            limit : min(FZZCONFIG.dup_limit, limit),
            n_failed_notify_allowed: n_failed_notify_allowed,
            slopes : slopes,
            groups : groups,
            dtor: dtor,
            ccache : (level, !0),
        }
    }
}
