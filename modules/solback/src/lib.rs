#![feature(map_first_last)]

use std::collections::{HashMap, BTreeSet};
use std::time::Instant;

extern crate serde_derive;
extern crate serde;

extern crate rand;
use rand::Rng;

extern crate core;

use core::exec::call::Call;
use core::banana::observer::{ICallObserver, IStateObserver, WantedMask};
use core::state::state::StateInfo;

extern crate common;

const MAX_CID: usize = 0x1000;
const MAX_AID: usize = 8;//1 + 3/*atackers*/ + 2/*mints*/; (X+1).pow(2).count_ones()

struct SolBack {
    wanted: Option<WantedMask>,
    hitmap: [u8; MAX_CID],
    hitset: BTreeSet<usize>,
    ctormap: HashMap<u64, usize>,
    cc: usize,
    level: usize,
    tictoc: Option<Instant>
}
impl SolBack {
    fn new() -> Self {
        Self {
            wanted: None,
            hitmap: [0u8; MAX_CID],
            hitset: BTreeSet::new(),
            ctormap: HashMap::new(),
            cc: 0,
            level: 0,
            tictoc: None,
        }
    }
    fn notify(&mut self, state: &StateInfo, call: &mut Call) -> Result<bool, WantedMask> {
        if None == self.tictoc {
            self.tictoc.insert(Instant::now());
        }
        if call.id().is_default() {
            return Ok(true)
        }
        self.level = state.level; // syncer module must be installed !!

        if 0 != state.level {
            return Ok(true)
        } // not too much necessary check tbh
        let _ = self.ctormap.insert(
            state.uid(), 
            unsafe { std::mem::transmute::<_, usize>(call.id()) }
            );
        Ok(true)
    }

    fn ctor(&mut self, state: &StateInfo) -> bool {
        if let Some(&cid) = self.ctormap.get(&state.uid()) {
            self.cc += 1;
        // read aid from the back of the structure, last byte ?
            unsafe { self.logic(
                state, 
                state.fd.data().iter().last().unwrap().clone(),
                cid
            ) }
        }
        true
    }
    fn aftermath(&mut self, state: &StateInfo, call: &mut Call) -> bool {
        if call.id().is_default() {
            return true
        }
        // this check is same as in bfl after_fuzzy
        // syncer module must be installed !!
        if !call.ok() && state.level == self.level {
            return false
        }
        self.cc += 1; // ok libbfl have stored this call to POC

        if !call.ok() {
            return false
        }
        assert!(
            1 == call.einfo().len(),
            "call info {:?}, size ? {:?}", call.name(), call.einfo().len()
        );

        let slot = call.einfo()[0];
        let cid = unsafe { std::mem::transmute::<_, usize>(call.id()) };
        unsafe { self.logic(state, slot, cid) }
        true
    }

    unsafe fn logic(&mut self, state: &StateInfo, slot: u8, cid: usize) {
        self.hitmap[cid] |= slot;
        let _ = self.hitset.insert(cid);

        //assert!(self.hitmap[cid].count_ones() < MAX_AID as u32);
        
        let _aid = self.hitmap[cid].count_ones() as usize;

        // heatmap
        ::log_feedback(
            cid + _aid * MAX_CID,
            1,
            self.cc,
            self.tictoc.as_ref().unwrap().elapsed().as_millis()
        );

        for &i in &self.hitset { // go over all calls
            if 1 != slot && 1 != self.hitmap[i] && 0 == slot & self.hitmap[i] {
                continue // this cid is not hit by this attacker yet
            }

// wrong .. thus we go for the second option, horizontal marking

            let aid = if 1 != self.hitmap[i] {
                self.hitmap[i].count_ones() as usize
            } else { _aid };
if aid < _aid {
            // storing edge
            ::log_feedback(
                i + aid * MAX_CID + (MAX_CID + cid) * (MAX_AID * MAX_CID),
                1,
                self.cc,
                self.tictoc.as_ref().unwrap().elapsed().as_millis()
            );
}
            // giving hint
            ::log_feedback(
                cid + aid * MAX_CID + i * (MAX_AID * MAX_CID),
                1,
                self.cc,
                self.tictoc.as_ref().unwrap().elapsed().as_millis()
            );

        }
    }
    pub fn dtor(&mut self, _state: &StateInfo) { }
    pub fn revert(&mut self, _info: &StateInfo, _call: &Call, mask: WantedMask) { }
    pub fn stop(&mut self) { }
}

common::callback_proxy!(SolBack);

pub fn observers() -> (
    Option<Box<dyn IStateObserver>>,
    Option<Box<dyn ICallObserver>>,
) {
    let lookup = Arc::new(RwLock::new(SolBack::new()));
    (
        Some(Box::new(Proxy::new(Arc::clone(&lookup)))),
        Some(Box::new(Proxy::new(Arc::clone(&lookup)))),
    )
}

#[allow(warnings)]
extern "C" {
    #[no_mangle]
    fn log_feedback(index: usize, info: u8, cc: usize, tictoc: u128);
    //hitmap: [u8; MAX_CID * MAX_AID * MAX_CID],
    //hitmap: [0u8; MAX_CID * MAX_AID * MAX_CID],
}
