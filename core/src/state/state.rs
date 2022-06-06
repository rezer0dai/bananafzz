use std::{
    thread,
    time::Duration,
    cmp::min,
    sync::Weak,
};

extern crate rand;
use rand::Rng;

use banana::bananaq;
use banana::bananaq::FuzzyQ;

use exec::call::Call;
use exec::id::CallTableId;
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
    fn fuzzy_loop(&mut self, state_idx: u16) -> Result<(), String>;
    /// will do essentialy same as fuzzy_loop, however is invoked only until state is not initialized
    fn fuzzy_init(&mut self) -> Result<(), String>;
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
    pub bananaq: Weak<FuzzyQ>,
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
    /// signaling when dtors apply
    pub finished: bool,
}

impl StateInfo {
    pub fn uid(&self) -> u64 { u64::from(thread::current().id().as_u64()) }
    pub fn bananaq(&self) -> Weak<FuzzyQ> { self.bananaq.clone() }
}

/// user mode state ( representation ) of target ( kernel object, remote object, io device, .. )
pub struct State {
    /// sharable informations with FuzzyQueue (rnd fd arg, de-dups, ..) && with modules
    info: StateInfo,
    /// if call constantly fails to pass trough plugins
    failing_delay_limit: u64,
    /// hardcoded limit per object how many iteration of fuzzing to perform on it - then close
    limit: usize,
    dead_ratio: f64,
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
    pub fn do_fuzz_one(&mut self, shared: &mut[u8]) -> Result<(), String> {
        if self.info.total > self.limit {
            log::trace!("fuzz one DONE");
            return Err(format!("[state] out of fuzzing limit total : {:?} > limit : {:?}",
                    self.info.total, self.limit))
        }
        self.info.total += 1;
        let fd = self.info.fd.clone();
        let mut dead = false;

        let mut oracle: u64 = 0;

        let bananaq = &self.info.bananaq;

        let mut i = 0;
        for _ in 0u16.. {
            i += 1;
            if !bananaq::is_active(&self.info.bananaq())? {
                break
            }

            let uid = std::thread::current().id();
            let uid = u64::from(uid.as_u64());
            log::trace!("we will choose by oracle {oracle} uid:{uid}");

            loop {
                self.ccache.1 = rand::thread_rng().gen_range(0..self.groups[self.ccache.0].len());
                if 0 == oracle {
                    break
                }
                if CallTableId::Id(oracle) == self.groups[self.ccache.0][self.ccache.1].id() {
                    break
                }
            }

            log::trace!("!! choosen by oracle {oracle} -> {:?} <{}> :: tid:{uid}", self.groups[self.ccache.0][self.ccache.1].id(), self.groups[self.ccache.0][self.ccache.1].name());

            if !self.call_view().dead(self.dead_ratio) 
                && self.groups[self.ccache.0][self.ccache.1]
                    .do_call(&bananaq, &fd.data(), shared) 
            { return Ok(()) }

            oracle = self.groups[self.ccache.0][self.ccache.1].oracle();
            if oracle != 0 {
                log::trace!("we got and oracle: {oracle} vs {:?}", self.groups[self.ccache.0][self.ccache.1].id());
            }

            // ok do some proportional way wait
            thread::sleep(Duration::from_nanos(1 + 100 * (self.call_view().n_attempts() as u64) % self.failing_delay_limit));
            /*
            if self.level() > 0 && 1 != self.groups[self.ccache.0].len() {
                thread::sleep(Duration::from_nanos(1 + 100 * ((self.call_view().n_attempts() - 1) as u64) % self.failing_delay_limit));
            } else { // otherwise just for task switch
                thread::sleep(Duration::from_nanos(1));
            }
            */

            if self.groups[self.ccache.0]
                .iter()
                .any(|ref call| !call.dead(self.dead_ratio)) { continue }
            dead = true;
        }
        self.call_dtor(shared);

        Err(format!("[state] end of fuzzing cycle for this object <{:?}::{:?}>, is dead : {:?}; fuzzing attempts for this round : {:?}, total fuzzed calls : {:?} -> {}\n\t==> w/ shared<{:?}>", self.info.name, fd.data(), dead, i, self.info.total, if self.ccache.0 < self.groups.len() && self.ccache.1 < self.groups[self.ccache.0].len() { self.call_view().name() } else { "no-call-yet" }, &shared))
    }

    /// need to be called after do_fuzz_one, to change level based on slopes!
    ///
    /// - call once do_fuzz_one will return true
    /// - this basically wraps do_fuzz_update_impl, that it checks for end of fuzzing
    ///     - and then it performs closing of this State by calling dtor syscall!
    pub fn do_fuzz_update(&mut self, shared: &mut[u8]) -> Result<(), String> {
        thread::yield_now();

        //maybe better weight it against #calls in current group
        //this way we trust a lot that every call is implemented correctly == good way
        //it is quick death then, like object is no longer online
        //however, if some syscall poorely implemented this will kill fuzzing for whole fuzzy object most
        // likely ...
        let e = match self.do_fuzz_update_impl() {
            Ok(_) => return Ok(()),
            Err(e) => e,
        };
        self.call_dtor(shared);

        Err(format!("[state] failing fuzzning cycle : update failed for fuzzed object <{:?}::{:?}> total fuzzed calls : {:?}, with details <{e}>", self.info.name, self.info.fd.data(), self.info.total))
    }
    fn call_dtor(&mut self, shared: &mut[u8]) {
        thread::yield_now();

        if self.info.fd.is_invalid() {
            return
        }
        self.info.level = usize::MAX;
        self.info.finished = true;

        if let Ok(_) = bananaq::update(&self.info) {
            self.dtor.do_call(&self.info.bananaq, self.info.fd.data(), shared);
        }
    }
    /// update slopes - state of current State, that we can proceed to fuzz next layer of syscalls
    fn do_fuzz_update_impl(&mut self) -> Result<(), String> {
        assert!(usize::MAX != self.info.level, "DTOR CALLED UPDATE CALLED TOO!!");

        let mut call = &mut self.groups[self.ccache.0][self.ccache.1];
        log::trace!("call <{}> success?({}) did go trough for <{}>", call.name(), call.ok(), self.info.name);

        let ok = if call.ok() { 1 } else { 0 };

        self.ccache.0 = (self.ccache.0 as isize + self.slopes[self.ccache.0][ok]) as usize;
        self.ccache.1 = !0;//invalidate!! - now self.{c/m}call() is pretty much invalid!

        let level = self.info.level;
        self.info.sucess += ok;
        self.info.level = self.ccache.0;

        if 0 != level {//write locked callback
            bananaq::call_aftermath(&mut self.info, &mut call)?;
        } // for ctors we have notify_ctor at core/banana/looper.rs

        if self.info.total > self.limit {
            return Err(format!("fuzzed over limit : total:{:?} vs limit:{:?}",
                    self.info.total, self.limit))
        }

        if !call.dead(self.dead_ratio) {
            return Ok(())
        }

        let dead_calls = self.groups[self.ccache.0]
            .iter()
            .filter(|&call| call.dead(self.dead_ratio))
            .map(|call| format!("<{}>", call.name()))
            .collect::<Vec<String>>();

        if dead_calls.len() * 2 < self.groups[self.ccache.0].len() {
            return Ok(())
        }

        Err(format!("more thatn 1/2 of group #{:?} call are dead; with current calls {:?}",
                self.ccache.0, dead_calls.join("")))
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
        bananaq: Weak<FuzzyQ>,
        name: &'static str,
        id: StateTableId,
        fd_size: usize,
        limit: usize,
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

        let fzzconfig = bananaq::config(&bananaq).unwrap();

        State {
            info : StateInfo {
                bananaq : bananaq,
                name : String::from(name),
                total : 0,
                sucess : 0,
                fd : Fd::empty(fd_size),
                id : id,
                level : 0,
                finished : false,
            },
            failing_delay_limit : fzzconfig.failing_delay_limit,
            dead_ratio : fzzconfig.dead_call,
            limit : min(fzzconfig.new_limit, limit),
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
        bananaq: Weak<FuzzyQ>,
        name: &'static str,
        id: StateTableId,
        fd: &Fd,
        limit: usize,
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

        let fzzconfig = bananaq::config(&bananaq).unwrap();

        let level = slopes[0][1] as usize;

        State {
            info : StateInfo {
                bananaq : bananaq,
                name : String::from(name),
                total : 0,
                sucess : 0,
                fd : fd.clone(),
                id : id,
                level : level,
                finished : false,
            },
            failing_delay_limit : fzzconfig.failing_delay_limit,
            dead_ratio : fzzconfig.dead_call,
            limit : min(fzzconfig.dup_limit, limit),
            slopes : slopes,
            groups : groups,
            dtor: dtor,
            ccache : (level, !0),
        }
    }
}
