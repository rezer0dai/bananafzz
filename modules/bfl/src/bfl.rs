use std::{
    mem::size_of,
    collections::{BTreeMap,HashMap,BTreeSet,HashSet},
};

use core::exec::call::Call;
use core::exec::id::CallTableId;
use core::state::state::StateInfo;
use core::banana::bananaq;

use repro::PocCall;
use poc::PocData;
pub use info::{BananizedFuzzyLoopConfig, PocCallHeader};

extern crate rand;
use self::rand::Rng;

type TUidLookup = BTreeMap<u64, u64>;
type TFdLookup = HashMap< Vec<u8>, Vec<u8> >;

type TUidOnce = BTreeSet<u64>;
type TFdOnce = HashSet< Vec<u8> >;

pub struct BananizedFuzzyLoop {
    cfg: BananizedFuzzyLoopConfig,

    fid_lookup: TFdLookup,
    uid_lookup: TUidLookup,

    fid_once: TFdOnce,
    uid_once: TUidOnce,

    poc: PocData,
    poc_ind: usize,

    ctor_done: bool,
    call_data: Vec<u8>,

    ctors_cnt: usize,
    calls_cnt: usize,

    fuzzy_uid: u64,
    fuzzy_cnt: usize,

    level: usize,

    passed: usize,
    n_attempts: usize,
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
            call_data : vec![],

            ctors_cnt: 0,
            calls_cnt: 0,

            fuzzy_uid: 0,
            fuzzy_cnt: 0,

            level: 0,

            passed: 0,
            n_attempts: 0,
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
    fn resolve_fid(&mut self, fid_a: &[u8], fid_b: &[u8]) -> bool {
        assert!(!fid_a.iter().all(|b| 0 == *b));
        if self.fid_lookup.contains_key(fid_a) {
            return fid_b.eq(&self.fid_lookup[fid_a])
        }
        if self.fid_once.contains(fid_b) {
            return false
        }
        self.fid_once.insert(fid_b.to_vec());
        self.fid_lookup.insert(fid_a.to_vec(), fid_b.to_vec());
        true
    }
    fn stop_or_force(&mut self, n_attempts: usize, add_prob: f64) -> bool {
        if !self.poc.do_gen() && self.cfg.is_strict_repro {
            return false
        }

        let max_n_try = self.cfg.max_allowed_wait_count_per_call as f64 * 0.8;
        let n_try = 1.0 * (n_attempts % self.cfg.max_allowed_wait_count_per_call) as f64;
        if max_n_try > n_try // seems too in-efficient to do ?
            && self.n_attempts < 50
            && self.passed < 10
        { // if all good, then we just need to try little more
if self.cfg.debug { println!("atempts are good, try harder => {:?} /{n_attempts}", self.poc_ind); }
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
if self.cfg.debug { println!("@@@@@@@@@@@@@@@@@@@@@@@@ adding one more to fuzz ({:?} x {:?}) || stats => {:?}", (self.poc_ind, self.poc.added), self.poc.info.calls_count, (n_attempts, x_attempts, passed)); }
            return self.poc.add_one(self.poc_ind) 
        }

        self.poc.skip(self.poc_ind);
        self.poc_ind += 1;
        self.fuzzy_cnt = 0;
if self.cfg.debug { println!("$$$$$$$$$$$$$$$$$$$$$$$$ lets do skip ({:?} x {:?}) || stats : [{:?}] add_prob:{:?}", (self.poc_ind, self.poc.added), self.poc.info.calls_count, (n_attempts, x_attempts, passed), add_prob); }
        return false
    }
    fn notify_locked_repro(&mut self, state: &StateInfo, call: &mut Call) -> bool {
if self.cfg.debug { println!("nbotify ->  {:?} <{:?}>", state.uid(), self.poc.info.calls_count) }
        if self.poc_ind == self.poc.max_ind() {
if self.cfg.debug { println!("pocind") }
            return false //last one was ctor, skip this call and start fuzzy-generate
        }
        self.n_attempts += 1;
        let poc = PocCall::new(&self.poc.load(self.poc_ind));

        if u64::from(state.id) != poc.info.sid {
            // should not happen if environment is predictable
            // though when we use splice or insert, we messing with this
        // aka environnment of origina POC is changed
if self.cfg.debug { println!("#sid (object:{:?}; bananaq.len={:?}) stop or forcese  [{:?}/{:?}] -> <{:?}][{:?}> last_call {:?}", state.uid(), bananaq::len(&state.bananaq).unwrap(), self.poc_ind, self.poc.info.calls_count, u64::from(state.id), poc.info.sid, self.poc.is_last_call(1 + self.poc_ind)) }
            return self.stop_or_force(call.n_attempts(), 0.7)
//            return false
        }

        if !state.fd.is_invalid() // need to here cuz dupped
            && !self.resolve_fid(&poc.fid, state.fd.data()) {
if self.cfg.debug { println!("fid") }
            return false
        }

        if !self.resolve_uid(poc.info.uid, state.uid()) {
if self.cfg.debug { println!("uid : {:?} x {:?} \n\t FULL UID MAP {:?}", state.uid(), poc.info.uid, self.uid_lookup) }
            return false
        }//C&C iterative approach, as we monitoring it from the begining!! 

        if state.level != poc.info.level {

if self.cfg.debug { println!("#levels {:?} stop or force in bananaq#{:X}", (state.level, poc.info.level), bananaq::qid(&state.bananaq).unwrap()) }

            return self.stop_or_force(
                call.n_attempts(), 
                if 0 != poc.info.level { 0.5 } else { 0.0 });
        }
        

        //seems problem hereis poc.info.cid = 0, aka WTF ??
        if CallTableId::Id(poc.info.cid) != call.id() {

if self.cfg.debug { println!("#cid stop or force in bananaq#{:X}", bananaq::qid(&state.bananaq).unwrap()) }

//for _ in 0..1000 { println!("cid with levels {:?}", (poc.info.level, state.level, poc.info.cid, call.id(), self.poc_ind)) }
            return self.stop_or_force(call.n_attempts(), 0.0)//seems wanted call is dead ??
        }
        self.n_attempts = 0;
        self.passed += 1;
        if self.passed > 10 {//call.n_attempts() % 10 { // ok seems upper layer plugins holding it off

if self.cfg.debug { println!("#atempts stop or force in bananaq#{:X}", bananaq::qid(&state.bananaq).unwrap()) }

            return self.stop_or_force(call.n_attempts(), 1.0)//try add something
        }

        if self.ctor_done { // OK, AFL did good job if ctor
/****************************/
/* DOING REPRO on this call */
/****************************/
            call.load_args(&poc.dmp, &poc.mem, &self.fid_lookup);
        } else {//AFL screwed ctor, we want to abandon fuzzing
if self.cfg.debug { println!("STOP2") }
            bananaq::stop(&state.bananaq).unwrap();
            return false
            // another option, is let it bananafuzzer fix it
            // do few more iterations until ctor is OK
            // but then it will meddle with AFL and its statistics
            // mainly it will connect new code cov with screwed ctor
            // not with fixed ctor, which will addup until 
            // pairing code cov - poc will be out of sync too much
        }
        self.level = state.level;
        if state.fd.is_invalid() { // we stop all calls until we observe ctor!!
            self.ctor_done = false;
        }
        true
    }
    fn aftermath_repro(&mut self, state: &StateInfo, call: &mut Call) {
//println!("APPROVED");
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
    }
    fn verify_ctor(&mut self, state: &StateInfo) -> bool {
        let poc = PocCall::new(&self.poc.load(self.poc_ind));
        if self.resolve_fid(&poc.fid, state.fd.data()) {
            return true
        }
// could happen once ctor StateIds/StateTableId < 0x10
if self.cfg.debug { loop { println!("STOP3") } } // this should not happen
        bananaq::stop(&state.bananaq).unwrap();
        println!("[BFL] Overlapping fid at runtime: {:?} != {:?}\n\t=> {:?}", 
            state.fd.data(), self.fid_lookup[&poc.fid], poc.fid);
        return false;
    }
    fn notify_ctor_locked_repro(&mut self, state: &StateInfo) -> bool {
        if 0 == state.total {
            return true // dupped
        }

        if !self.verify_ctor(state) {
            return false
        }

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
if self.cfg.debug { println!("STOP4") }
            bananaq::stop(&state.bananaq).unwrap();
            return false
        }
//lock current state.uid as fuzzing target for generated banana call to AFL
        self.fuzzy_cnt += 1;

        if 2 >= self.fuzzy_cnt % 20 {//3 is maybe too low time for ctor to appear ?
            self.fuzzy_uid = 0
        }
        if 0 == self.fuzzy_uid {
            self.fuzzy_uid = state.uid()
        } else if state.uid() != self.fuzzy_uid {

if self.cfg.debug { println!("[bfl] denied-fuzzy-insert") }

            return false // waiting for ctor
        }

/*****************************/
/* DUMPING for repro and BFL */
/*****************************/
        self.call_data = PocCall::dump_call(call, state.id.into(), &state.fd, state.uid(), state.level);

        self.level = state.level;
        true
    }
    fn aftermath_fuzzy(&mut self, state: &StateInfo, call: &mut Call) {
        if state.uid() != self.fuzzy_uid
            || 0 == self.call_data.len() 
        { return bananaq::stop(&state.bananaq).unwrap() }

        assert!(self.poc.do_gen() || self.poc.is_last_call(self.poc_ind));

        self.poc.runtime(self.level, state.uid(), state.fd.data());//also fid ?
//allow for next fuzzy call in other state.uid / thread / object
        self.fuzzy_uid = 0;
//now load it to SHMEM -> should do exit process too!!
        self.poc.push(self.poc_ind, &self.call_data, call.kin());
        if self.poc.share(self.cfg.pocmem) {
            return bananaq::stop(&state.bananaq).unwrap()
        }

        self.call_data.clear();
//if self.stop_fuzzing { println!("LEGIT : STOPY"); }
        if self.poc.is_last_call(1 + self.poc_ind)
            || self.poc.added > self.cfg.max_inserts
            || !rand::thread_rng().gen_bool(1.0 - self.poc.added as f64 / self.cfg.max_inserts as f64) 
        { return }
                
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

if self.cfg.debug { println!("refusing ctor") }

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
            bananaq::stop(&state.bananaq).unwrap()
        }
        self.call_data.clear();
        true
    }

    pub fn notify_locked(&mut self, state: &StateInfo, call: &mut Call) -> bool {
        if call.id().is_default() {
            assert!(0 != state.level);
            return true
        }
        if !bananaq::is_active(&state.bananaq).unwrap() {
            return false
        }
//println!("locked notify all");
        if self.poc_ind > self.poc.max_ind() {

if self.cfg.debug { println!("STOP5 {:?} / {:?}", (self.poc_ind, self.poc.max_ind()), self.poc.info.calls_count) }

            bananaq::stop(&state.bananaq).unwrap();
            return false
        }
        if self.poc_ind < self.poc.max_ind() {
            self.notify_locked_repro(state, call)
        } else {
            self.notify_locked_fuzzy(state, call)
        }
    }
    pub fn notify_ctor_locked(&mut self, state: &StateInfo) -> bool {

if self.cfg.debug { println!("NEW OBJECT!!! {:?} + {:?}", state.id, state.fd.data()) }

        if 0 == state.total {
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
    pub fn aftermath_locked(&mut self, state: &StateInfo, call: &mut Call) {
        if call.id().is_default() {
            return // defaults, probably doing it up 
        }
        self.passed = 0;

if self.cfg.debug { println!("AFTERMATH OK") }

        if self.poc_ind < self.poc.max_ind() {
            self.aftermath_repro(state, call)
        } else {
            self.aftermath_fuzzy(state, call)
        }

        if !self.ctor_done {
            return bananaq::stop(&state.bananaq).unwrap()
        }
    }
}
