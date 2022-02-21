use std::{
    mem::size_of,
    collections::{BTreeMap,HashMap},
};

use core::exec::call::Call;
use core::exec::id::CallTableId;
use core::exec::fd_info::Fd;
use core::state::state::StateInfo;

use repro::PocCall;
use poc::PocData;
pub use info::{BananizedFuzzyLoopConfig, PocCallHeader};

type TUidLookup = BTreeMap<u64, u64>;
type TFdLookup = HashMap< Vec<u8>, Vec<u8> >;

pub struct BananizedFuzzyLoop {
    cfg: BananizedFuzzyLoopConfig,
    stop_fuzzing: bool,
    uid_lookup: TUidLookup,
    fid_lookup: TFdLookup,

    poc: PocData,
    poc_ind: usize,

    ctor_done: bool,
    call_data: Vec<u8>,

    ctors_cnt: usize,
    calls_cnt: usize,

    fuzzy_uid: u64,
    fuzzy_cnt: usize,

    kin: usize,
}

impl BananizedFuzzyLoop {
    pub fn new(config: &BananizedFuzzyLoopConfig) -> BananizedFuzzyLoop {
        //we should polute fid_lookup by empty and invalid ?
        BananizedFuzzyLoop {
            cfg: *config,
            stop_fuzzing: false,

            uid_lookup : TUidLookup::new(),
            fid_lookup : TFdLookup::new(),
//load POC from shared memory!!
            poc : PocData::new(config.magic, config.shmem),
            poc_ind : 0,

            ctor_done : true,
            call_data : vec![],

            ctors_cnt: 0,
            calls_cnt: 0,

            fuzzy_uid: 0,
            fuzzy_cnt: 0,

            kin: 0,
        }
    }

    fn resolve_uid(&mut self, uid_a: u64, uid_b: u64) -> bool {
        if !self.uid_lookup.contains_key(&uid_a) {
            self.uid_lookup.insert(uid_a, uid_b);
        }
        uid_b == self.uid_lookup[&uid_a]
    }
    fn resolve_fid(&mut self, fid_a: &[u8], fid_b: &[u8]) -> bool {
        if !self.fid_lookup.contains_key(fid_a) {
            self.fid_lookup.insert(fid_a.to_vec(), fid_b.to_vec());
        }
        fid_b.eq(&self.fid_lookup[fid_a])
    }
    fn notify_locked_repro(&mut self, state: &StateInfo, call: &mut Call) -> bool {
//println!("[BFL] repro call ==> {:?}", self.poc_ind);
        let poc_ind = if self.ctor_done {
            self.poc_ind + 1
        } else { self.poc_ind };

//println!("[BFL] repro call ==> stats {:?} vs {:?}", poc_ind, self.poc.max_ind());
        if self.poc_ind == self.poc.max_ind() {
            return false //last one was ctor, skip this call and start fuzzy-generate
        }

        let poc = PocCall::new(&self.poc.load(self.poc_ind));
//println!("[BFL] repro call ==> INFO {:?} vs {:?}", CallTableId::Id(poc.info.cid), call.id());
        if CallTableId::Id(poc.info.cid) != call.id() {
            return false
        }
//println!("[BFL] repro call ==> stage 2");
        
        if !self.resolve_uid(poc.info.uid, state.uid) {
            return false
        }//C&C iterative approach, as we monitoring it from the begining!! 
//println!("[BFL] repro call ==> stage 3");

        if self.ctor_done { // OK, AFL did good job if ctor
/****************************/
/* DOING REPRO on this call */
/****************************/
//println!("[BFL] poc.dmp size : {:X}", poc.dmp.len());
            call.load_args(&poc.dmp, &poc.mem, &self.fid_lookup);
        } else {//AFL screwed ctor, we want to abandon fuzzing
//println!("[BFL] STOP FUZZING screw");
            self.stop_fuzzing = true
            // another option, is let it bananafuzzer fix it
            // do few more iterations until ctor is OK
            // but then it will meddle with AFL and its statistics
            // mainly it will connect new code cov with screwed ctor
            // not with fixed ctor, which will addup until 
            // pairing code cov - poc will be out of sync too much
        }

        // we need to do per AFL fuzz_one, to keep state info up to data
        // how we do it ? AFL forward us *CONST data, we transmute to *MUT ..
        // ok for InMemory fuzzing, for LibAFL Fork it will not work...
        // anyway i dont like this too much, const -> mut lel ...
        self.poc.desc_data(self.poc_ind).kin = self.kin;//one step later
        // one pitfall is that kin is object specific, need to be considered when crossover

        if state.fd.is_invalid() { // we stop all calls until we observe ctor!!
            self.ctor_done = false;
        } else { // normal calls ( not ctors ) go trough
            self.calls_cnt += 1;
            self.poc_ind += 1;//= poc_ind;
        }
//println!("[BFL] repro call OK");
        true
    }
    pub fn aftermath_locked(&mut self, _state: &StateInfo, call: &mut Call) {
        self.kin = call.kin();
    }
    fn notify_ctor_locked_repro(&mut self, state: &StateInfo) -> bool {
//println!("[BFL] repro ctor ==> {:?}", self.poc_ind);
        let poc = PocCall::new(&self.poc.load(self.poc_ind));
        if !self.resolve_fid(&poc.fid, state.fd.data()) {
            println!("[BFL] Overlapping fid at runtime: {:?} != {:?}", 
                state.fd.data(), self.fid_lookup[&poc.fid]);
        }

        self.poc_ind += 1;
        self.ctors_cnt += 1;
        // this matching is enforced by notify_locked deny all ctors 
        // until our own recognized pass trhough here,
        // lets signal back we are done here, resume poc_ind
        self.ctor_done = true;
        true
    }

    fn broken_fuzzy_ratio(&self, fd: &Fd) -> bool {
        if self.ctors_cnt + self.calls_cnt < self.cfg.warmup_cnt {
            return false
        }
        if fd.is_invalid() && self.ctors_cnt * self.cfg.ctor_max_ratio > self.calls_cnt {
            return false
        }
        if !fd.is_invalid() && self.ctors_cnt * self.cfg.ctor_min_ratio < self.calls_cnt {
            return false
        }
        true
    }

    pub fn notify_locked_fuzzy(&mut self, state: &StateInfo, call: &mut Call) -> bool {
//println!("[BFL] fuzzy call");
//        if self.broken_fuzzy_ratio(&state.fd) {
//            return false
//        }
//println!("[BFL] ratio OK");

//lock current state.uid as fuzzing target for generated banana call to AFL
        self.fuzzy_cnt += 1;
        if self.fuzzy_cnt > 100 {
            self.stop_fuzzing = true;
            panic!("[BFL] seems banana ctor impossible to finish");
        }
        if 0 == self.fuzzy_uid {
            self.fuzzy_uid = state.uid
        } else if state.uid != self.fuzzy_uid {
            return false // waiting for ctor
        }
//println!("[BFL] UID MATCH");

/*****************************/
/* DUMPING for repro and BFL */
/*****************************/
        self.call_data = PocCall::dump_call(call, &state.fd, state.uid);

//println!("[BFL] fuzzy allowed call ==> {:?}", self.call_data);

        if state.fd.is_invalid() { 
            return true
        }
//allow for next fuzzy call in other state.uid / thread / object
        self.fuzzy_uid = 0;
//now load it to SHMEM -> should do exit process too!!
        self.poc.push(self.poc_ind, &self.call_data, self.kin);
        self.stop_fuzzing = self.poc.share(self.cfg.pocmem);
        true
    }
    pub fn notify_ctor_locked_fuzzy(&mut self, state: &StateInfo) -> bool {
//println!("[BFL] fuzzy ctor ==> {:?} vs {:?}", self.call_data.len(), size_of::<PocCallHeader>());
        let poc = PocCall::new(&self.call_data);
        if state.uid != poc.info.uid {
            return false
        }

        //We need to have fd info even for ctor-call due to repro
        self.call_data[size_of::<PocCallHeader>()..][..state.fd.data().len()]
            .clone_from_slice(state.fd.data());

//allow for next fuzzy call in other state.uid / thread / object
        self.fuzzy_uid = 0;
//now load it to SHMEM -> should do exit process too!!
        self.poc.push(self.poc_ind, &self.call_data, self.kin);
        self.stop_fuzzing = self.poc.share(self.cfg.pocmem);
        true
    }

    pub fn notify_locked(&mut self, state: &StateInfo, call: &mut Call) -> bool {
        if self.stop_fuzzing {
//println!("[BFL] STOP FUZZING notify");
            return false
        }
//println!("? [{:?} vs {:?}]", self.poc_ind, self.poc.max_ind());
        if self.poc_ind < self.poc.max_ind() {
            self.notify_locked_repro(state, call)
        } else {
            self.notify_locked_fuzzy(state, call)
        }
    }
    pub fn notify_ctor_locked(&mut self, state: &StateInfo) -> bool {
        if self.stop_fuzzing {
//println!("[BFL] STOP FUZZING ctor");
            return false
        }
        if self.poc_ind < self.poc.max_ind() {
//        if 0 == self.call_data.len() {
//            assert!(self.poc_ind < self.poc.max_ind(), 
//                "[BFL] ctor after repro, but before fuzzy");
            self.notify_ctor_locked_repro(state)
        } else {
            self.notify_ctor_locked_fuzzy(state)
        }
    }
}
