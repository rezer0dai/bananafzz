extern crate log;
use self::log::{trace, debug, info, warn, error};

use std::{
    mem::size_of,
    collections::{BTreeMap,HashMap,BTreeSet,HashSet},
};

use core::exec::call::Call;
use core::exec::id::CallTableId;
use core::state::state::StateInfo;
use core::banana::bananaq;
use core::banana::observer::WantedMask;

use repro::PocCall;
use poc::PocData;//{PocData, INCOMPLETE};
pub use info::{BananizedFuzzyLoopConfig, PocCallHeader};

extern crate rand;
use self::rand::Rng;

type TUidLookup = BTreeMap<u64, u64>;
type TFdLookup = HashMap< Vec<u8>, Vec<u8> >;

type TUidOnce = BTreeSet<u64>;
type TFdOnce = HashSet< Vec<u8> >;

macro_rules! call_attempts {
    ($call:expr, $state:expr, $n_attempts:expr, $poc:expr) => {
        if 0 != $state.level && 0 != $poc.info.level {
            // for $state attempts does not account before created o objects, level > 0
            // as for $poc.info.leve we refuse to skip ctor in repro
            // as it will lead to future broken calls by default..
            $call.attempts($n_attempts)
        } else { 0 }
    };
}

pub struct BananizedFuzzyLoop {
    cfg: BananizedFuzzyLoopConfig,

    fid_lookup: TFdLookup,
    uid_lookup: TUidLookup,

    fid_once: TFdOnce,
    uid_once: TUidOnce,

    poc: PocData,
    poc_ind: usize,

    ctor_done: bool,
    ctor_name: String,
    call_data: Vec<u8>,

    ctors_cnt: usize,
    calls_cnt: usize,

    fuzzy_uid: u64,
    fuzzy_cnt: usize,

    level: usize,

    passed: usize,
    n_attempts: usize,

    pub wanted: Option<WantedMask>,
}

impl BananizedFuzzyLoop {
    pub fn new(config: &BananizedFuzzyLoopConfig) -> BananizedFuzzyLoop {
        //we should polute fid_lookup by empty and invalid ?
        BananizedFuzzyLoop {
            cfg: *config,

            fid_lookup : TFdLookup::new(),
            uid_lookup : TUidLookup::new(),

            fid_once : TFdOnce::new(),
            uid_once : TUidOnce::new(),
//load POC from shared memory!!
            poc : PocData::new(config.magic, config.shmem),
            poc_ind : 0,

            ctor_done : true,
            ctor_name : String::from(""),
            call_data : vec![],

            ctors_cnt: 0,
            calls_cnt: 0,

            fuzzy_uid: 0,
            fuzzy_cnt: 0,

            level: 0,

            passed: 0,
            n_attempts: 0,

            wanted: None,
        }
    }

    fn resolve_uid(&mut self, uid_a: u64, uid_b: u64) -> bool {
        if self.uid_lookup.contains_key(&uid_a) {
            return uid_b == self.uid_lookup[&uid_a]
        }
        if self.uid_once.contains(&uid_b) {
            return false
        }
        self.uid_once.insert(uid_b);
        self.uid_lookup.insert(uid_a, uid_b);
        true
    }
    // for now ignore, fid it is little bit tricky for FdHolder
    // .. if fd overlaps between different objects need to handle it
    // .. bascically per call having FdHolder need to pass targeted Fd
    // .. problem with really generic ones ( like unicorn )
    // but then again, in resolve fid, if too generic we will sweep them all
    // again problem with overlaping fd cross objects..
    fn sid_prefix(_sid: u64) -> Vec<u8> {
        let mut prefix = vec![];//sid.to_le_bytes().to_vec();
        prefix.extend_from_slice(&[66u8; 4+2]);
        prefix
    }
    fn resolve_fid(&mut self, sid: u64, fid_a: &[u8], fid_b: &[u8]) -> bool {
        let mut sid_fid_a = Self::sid_prefix(sid);
        sid_fid_a.extend_from_slice(fid_a);
        let sid_fid_a = sid_fid_a;

        let mut sid_fid_b = Self::sid_prefix(sid);
        sid_fid_b.extend_from_slice(fid_b);
        let sid_fid_b = sid_fid_b;

        assert!(!sid_fid_a.iter().all(|b| 0 == *b));
        if self.fid_lookup.contains_key(&sid_fid_a) {
            return sid_fid_b.eq(&self.fid_lookup[&sid_fid_a])
        }
        if self.fid_once.contains(&sid_fid_b) {
            return false
        }
        self.fid_once.insert(sid_fid_b.clone());
        self.fid_lookup.insert(sid_fid_a, sid_fid_b);
        true
    }
    fn stop_or_force(&mut self, n_attempts: usize, add_prob: f64) -> bool {
        if !self.poc.do_gen() && self.cfg.is_strict_repro {
            return false
        }
        if 0 == n_attempts {
            return false // ctors must go trough!!
        }

        let max_n_try = self.cfg.max_allowed_wait_count_per_call as f64 * 0.8;
        let n_try = 1.0 * (n_attempts % self.cfg.max_allowed_wait_count_per_call) as f64;
        if max_n_try > n_try // seems too in-efficient to do ?
            && self.n_attempts < self.cfg.max_allowed_wait_count_per_call
            && self.passed < 10
        { // if all good, then we just need to try little more
trace!("atempts are good, try harder => {:?} /{n_attempts}", self.poc_ind);
            return false
        }

        let passed = self.passed;
        let x_attempts = self.n_attempts;

        self.passed = 0;
        self.n_attempts = 0;

        if self.poc.do_gen() // we may try to insert someting here
            && self.poc.added < self.cfg.max_inserts 
// ok to do as if poc.do_gen it is not counted to coverage feedback!!
            && rand::thread_rng().gen_bool(add_prob) 
            && !self.poc.is_last_call(1 + self.poc_ind)//this does not make sense to enable
//        { return self.poc.add_one(self.poc_ind) }
        { 
info!("@@@@@@@@@@@@@@@@@@@@@@@@ adding one more to fuzz ({:?} x {:?}) || stats => {:?}", (self.poc_ind, self.poc.added), self.poc.info.calls_count, (n_attempts, x_attempts, passed));
            return self.poc.add_one(self.poc_ind) 
        }

        let poc = PocCall::new(&self.poc.load(self.poc_ind));

        self.poc.skip(self.poc_ind);
        self.poc_ind += 1;
        self.fuzzy_cnt = 0;
warn!("$$$$$$$$$$$$$$$$$$$$$$$$ lets do skip, incomplete, [call : {}]({:?} x {:?}) || stats : [{:?}] add_prob:{:?}", poc.info.cid, (self.poc_ind, self.poc.added), self.poc.info.calls_count, (n_attempts, x_attempts, passed), add_prob);

        //unsafe { INCOMPLETE = true }
        return false
    }
    fn notify_locked_repro(&mut self, state: &StateInfo, call: &mut Call) -> bool {
trace!("nbotify ->  {:?} <{:?}>", state.uid(), self.poc.info.calls_count);
        if self.poc_ind == self.poc.max_ind() {
debug!("pocind");
            return false //last one was ctor, skip this call and start fuzzy-generate
        }
        if 0 != state.level {
            self.n_attempts += 1;
        }
        let poc = PocCall::new(&self.poc.load(self.poc_ind));

        if u64::from(state.id) != poc.info.sid {
            // should not happen if environment is predictable
            // though when we use splice or insert, we messing with this
        // aka environnment of origina POC is changed
debug!("#sid (object:{:?}; bananaq.len={:?}) stop or forcese  [{:?}/{:?}] -> <{:?}][{:?}> last_call {:?}", state.uid(), bananaq::len(&state.bananaq).unwrap(), self.poc_ind, self.poc.info.calls_count, u64::from(state.id), poc.info.sid, self.poc.is_last_call(1 + self.poc_ind));
            return self.stop_or_force(call_attempts!(call, state, self.n_attempts, poc), 0.7)
//            return false
        }

        if !state.fd.is_invalid() // need to here cuz dupped
            && !self.resolve_fid(poc.info.sid, &poc.fid, state.fd.data()) {
debug!("fid --> {:?}", (poc.fid, state.fd.data())); // seems common tbh
            return false
        }

        if !self.resolve_uid(poc.info.uid, state.uid()) {
debug!("uid : {:?} x {:?} \n\t FULL UID MAP {:?}", state.uid(), poc.info.uid, self.uid_lookup);
            return false
        }//C&C iterative approach, as we monitoring it from the begining!! 

        if state.level != poc.info.level {

info!("[bfl] object:{:X}=={} WRONG #levels {:?} <cid: {:?} ; name = {:?}> stop or force in bananaq#{:X}", poc.info.sid, poc.info.sid, (state.level, poc.info.level), poc.info.cid, state.name, bananaq::qid(&state.bananaq).unwrap());

            return self.stop_or_force(
                call_attempts!(call, state, self.n_attempts, poc), 
                if 0 != poc.info.level { 0.5 } else { 0.0 });
        }

        //seems problem hereis poc.info.cid = 0, aka WTF ??
        if CallTableId::Id(poc.info.cid) != call.id() {

debug!("#cid ({:?} vs {:?}) stop or force in bananaq#{:X}", poc.info.cid, call.id(), bananaq::qid(&state.bananaq).unwrap());

//for _ in 0..1000 { println!("cid with levels {:?}", (poc.info.level, state.level, poc.info.cid, call.id(), self.poc_ind)) }
            return self.stop_or_force(call_attempts!(call, state, self.n_attempts, poc), 0.0)//seems wanted call is dead ??
        }
        self.n_attempts = 0;
        self.passed += 1;
        if self.passed > 10 {//call.n_attempts() % 10 { // ok seems upper layer plugins holding it off

debug!("#atempts stop or force in bananaq#{:X}", bananaq::qid(&state.bananaq).unwrap());

            return self.stop_or_force(call_attempts!(call, state, self.n_attempts, poc), 1.0)//try add something
        }

        let data_load_freedom_ratio = if self.poc.do_gen() && rand::thread_rng().gen_bool(
            self.cfg.allow_data_load_freedom_ratio) 
        { 1.0 - self.cfg.data_load_freedom_ratio } else { 1. };

//println!("info : {:?} {data_load_freedom_ratio} | {:?}", call.name(), self.poc.do_gen());

        if let Err(msg) = call.load_args(&poc.dmp, &poc.mem, &Self::sid_prefix(poc.info.sid), &self.fid_lookup, data_load_freedom_ratio) {
//panic!("[libbfl] unable to load args #{}#{} :: <{msg}>", state.name, call.name());
            return false
        }

trace!("---> [fid : {:?}] : loading ARG : {}#{} with data|{:?}|", poc.fid, state.name, call.name(), poc.mem.len());
/*
        if !self.ctor_done { // OK, AFL did good job if ctor
error!("STOP2 -> failing ctor for : {} ( seems load args problem )", self.ctor_name);
            bananaq::stop(&state.bananaq).unwrap();
            return false // actually this should be an ASSERTQ!
        }
*/
        self.level = state.level;
        if state.fd.is_invalid() { // we stop all calls until we observe ctor!!
trace!("**************** we follow {}", call.name());
            self.ctor_name = format!("{} :: {}", state.name, call.name()).to_string();
            self.ctor_done = false;
        }
        true
    }
    fn aftermath_repro(&mut self, state: &StateInfo, call: &mut Call) {
trace!("APPROVED -> {:?}", state.fd.data());
// we now hard assuming all repro calls must be sucessfull beforehand!!
//assert!(call.ok() || state.level != self.level, format!("#{:?}: call {:?} FAILED!!", self.poc_ind, call.name()));
info!("#{:?}: call {:?} SUCKSES!!", self.poc_ind, call.name());
        // we need to do per AFL fuzz_one, to keep state info up to data
        // how we do it ? AFL forward us *CONST data, we transmute to *MUT ..
        // ok for InMemory fuzzing, for LibAFL Fork it will not work...
        // anyway i dont like this too much, const -> mut lel ...
        self.poc.desc_data(self.poc_ind).kin = call.kin();//one step later
        // one pitfall is that kin is object specific, need to be considered when crossover

        self.poc.runtime(self.level, state.uid(), state.fd.data());

        self.calls_cnt += 1;
        self.fuzzy_cnt = 0;//ok managed to go for repro
        self.poc_ind += 1; // poc_ind will be updated only if all observers agree == call was allowed
trace!("[bfl] approved-call {} / {} :: {:?} [ uid :: {:?}", state.name, call.name(), call.id(), state.uid());
    }
    fn verify_ctor(&mut self, state: &StateInfo) -> bool {
        let poc = PocCall::new(&self.poc.load(self.poc_ind));
        if self.resolve_fid(poc.info.sid, &poc.fid, state.fd.data()) {
            return true
        }
        
//println!("LOOKUP {:?}", self.fid_lookup);
//println!("ONCE {:?}", self.fid_once);
// could happen once ctor StateIds/StateTableId < 0x10
error!("STOP3 [SID:{:X} vs {:X}] {:?} vs {:?}", u64::from(state.id), poc.info.sid, poc.fid, state.fd.data()); // this should not happen
        bananaq::stop(&state.bananaq).unwrap();
warn!("[BFL] Overlapping fid at runtime: {:?} != {:?}\n\t=> {:?}", 
    state.fd.data(), self.fid_lookup, poc.fid);
        return false;
    }
    fn notify_ctor_locked_repro(&mut self, state: &StateInfo) -> bool {
trace!("APPROVED-CTOR -> {:?}", state.fd.data());

        if !self.verify_ctor(state) {
            return false
        }
        self.poc.runtime(self.level, state.uid(), state.fd.data());
trace!("[bfl] approved-ctor {} :: {:?}", state.name, state.uid());
        self.poc_ind += 1;
        self.ctors_cnt += 1;
        // this matching is enforced by notify_locked deny all ctors 
        // until our own recognized pass trhough here,
        // lets signal back we are done here, resume poc_ind
        self.ctor_done = true;
        true
    }

    pub fn notify_locked_fuzzy(&mut self, state: &StateInfo, call: &mut Call) -> bool {

        if !self.poc.do_gen() && !self.poc.is_last_call(self.poc_ind) {
error!("STOP4");
            bananaq::stop(&state.bananaq).unwrap();
            return false
        }
//lock current state.uid as fuzzing target for generated banana call to AFL
        self.fuzzy_cnt += 1;

        if self.fuzzy_cnt > 10 // seems fuzzy object have troubles
            && state.uid() != self.fuzzy_uid 
        {
            // as syncer assure us if it is not ctor, aftermath should follow right away
            // but if it is ctor, there is a time for racing with others..
            self.fuzzy_uid = 0;
            self.fuzzy_cnt = 0;
        }
        if 0 == self.fuzzy_uid {
            self.fuzzy_uid = state.uid()
        } else if state.uid() != self.fuzzy_uid {

trace!("[bfl] denied-fuzzy-insert");

            return false // waiting for ctor
        }

/*****************************/
/* DUMPING for repro and BFL */
/*****************************/
trace!("........OK WE DUMPING A CALL!!! {:?}", state.uid());
        self.call_data = PocCall::dump_call(call, state.id.into(), &state.fd, state.uid(), state.level);

        self.level = state.level;
        true
    }
    fn aftermath_fuzzy(&mut self, state: &StateInfo, call: &mut Call) {
        if state.uid() != self.fuzzy_uid
            || 0 == self.call_data.len() 
        { return bananaq::stop(&state.bananaq).unwrap() }

//allow for next fuzzy call in other state.uid / thread / object
        self.fuzzy_uid = 0;
        let call_data: Vec<u8> = self.call_data.drain(..).collect();

// TODO: OK THIS IS ON THE EDGE, allows hacker to specify failing calls
// to go to next level ( usefull sometimes when you just want to try call )
// but may hinder repro efforts, need to properly eval if THIS IS OK or NOT ?
        if !call.ok() && state.level == self.level {
trace!("FUZZY CAIL SOLY-SUCKS");
//seems garbage call, skip from BFL ~ well this is for SOLY, no good for general code cov..
            return
        }

info!("OK AFTERMATH CALL GOODIE : #{:?}: {:?}", self.poc_ind, call.name());

        assert!(self.poc.do_gen() || self.poc.is_last_call(self.poc_ind));

        self.poc.runtime(self.level, state.uid(), state.fd.data());//also fid ?
//now load it to SHMEM -> should do exit process too!!
        self.poc.push(self.poc_ind, &call_data, call.kin());
        if self.poc.share(self.cfg.pocmem) {
trace!("-------- CALL : OK SHAAARE");
            return bananaq::stop(&state.bananaq).unwrap()
        }

        if self.poc.is_last_call(1 + self.poc_ind)
            || self.poc.added > self.cfg.max_inserts
            || rand::thread_rng().gen_bool(self.poc.added as f64 / self.cfg.max_inserts as f64) 
        { return }
//if self.poc.added > 30 { println!("GOOOD ADDED LOTS > {:?}", self.poc.added); std::process::exit(0); }
        self.poc.add_one(self.poc_ind);
        self.fuzzy_cnt = 1;
    }
    pub fn notify_ctor_locked_fuzzy(&mut self, state: &StateInfo) -> bool {
        if state.uid() != self.fuzzy_uid
            || 0 == self.call_data.len() 
        { 
            bananaq::stop(&state.bananaq).unwrap();
            return false
        }

        let poc = PocCall::new(&self.call_data);
        if state.uid() != poc.info.uid {

trace!("refusing ctor");

            return false
        }

        //We need to have fd info even for ctor-call due to repro
        self.call_data[size_of::<PocCallHeader>()..][..state.fd.data().len()]
            .clone_from_slice(state.fd.data());
        self.poc.runtime(self.level, state.uid(), state.fd.data());//also fid ?

//allow for next fuzzy call in other state.uid / thread / object
        self.fuzzy_uid = 0;
        self.poc.push(self.poc_ind, &self.call_data, 0);//we avoid crossover over ctors ?
        if self.poc.share(self.cfg.pocmem) {
trace!("-------- CTOR : OK SHAAARE");
            bananaq::stop(&state.bananaq).unwrap()
        }
        self.call_data.clear();

        if self.poc.is_last_call(1 + self.poc_ind)
            || self.poc.added > self.cfg.max_inserts
            || rand::thread_rng().gen_bool(1f64 / (1 + self.cfg.max_inserts - self.poc.added) as f64) 
        { return true }
        self.poc.add_one(self.poc_ind);
        self.fuzzy_cnt = 1;

        true
    }

    pub fn notify(&mut self, state: &StateInfo, call: &mut Call) -> Result<bool, WantedMask> {
        if call.id().is_default() {
            assert!(0 != state.level);
            return Ok(true)
        }
        if !bananaq::is_active(&state.bananaq).unwrap() {
            return Ok(false)
        }
        if self.poc_ind > self.poc.max_ind() {

error!("STOP5 {:?} / {:?}", (self.poc_ind, self.poc.max_ind()), self.poc.info.calls_count);

            bananaq::stop(&state.bananaq).unwrap();
            return Ok(false)
        }
        let approved = if self.poc_ind < self.poc.max_ind() {
            self.notify_locked_repro(state, call)
        } else {
            self.notify_locked_fuzzy(state, call)
        };

        if approved {
            return Ok(true)
        }

        if self.poc_ind >= self.poc.max_ind() {
            // seems we do fuzzy next, lets say allow all!
            // with all 0, means we will allow anything
            return Err(WantedMask{mid:42, ..WantedMask::default()})
        }

        let poc = PocCall::new(&self.poc.load(self.poc_ind));
        Err(WantedMask{
            mid: 42,
            uid: if let Some(uid) = self.uid_lookup.get(&poc.info.uid) {
                *uid
            } else { 0 },
            sid: poc.info.sid,
            cid: poc.info.cid,
        })
    }
    pub fn ctor(&mut self, state: &StateInfo) -> bool {
trace!("NEW OBJECT!!! {:?} + {:?}", state.id, state.fd.data());

        if 0 == state.total {

            let _ = self.resolve_fid(
                66,
                state.fd.data(),
                state.fd.data());

            return true // racers always to be approved
        }
        self.passed = 0;
// here we should assert that call was not default...
        if self.poc_ind < self.poc.max_ind() {
            self.notify_ctor_locked_repro(state)
        } else {
            self.notify_ctor_locked_fuzzy(state)
        }
    }
    pub fn aftermath(&mut self, state: &StateInfo, call: &mut Call) {
        if call.id().is_default() {
            return // defaults, probably doing it up 
        }
        self.passed = 0;

        if self.poc_ind < self.poc.max_ind() {
trace!("AFTERMATH REPRO");
            self.aftermath_repro(state, call)
        } else {
trace!("AFTERMATH FUZZY");
            self.aftermath_fuzzy(state, call)
        }

        self.ctor_done = true;
/*
        if !self.ctor_done {
            return bananaq::stop(&state.bananaq).unwrap()
        }
*/
    }

    pub fn stop(&mut self) {
        while !self.poc.share(self.cfg.pocmem) {
            ;
        }
    }

    pub fn dtor(&mut self, _state: &StateInfo) { }
    pub fn revert(&mut self, _info: &StateInfo, _call: &Call, _mask: WantedMask) { }
}
