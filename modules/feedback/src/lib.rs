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
const MAX_AID: usize = 4;

struct FeedBack {
    wanted: Option<WantedMask>,
    hitmap: [u8; MAX_CID],
    hitset: BTreeSet<usize>,
    ctormap: HashMap<u64, usize>,
    cc: usize,
    level: usize,
    tictoc: Option<Instant>
}
impl FeedBack {
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
        if let Some(cid) = self.ctormap.get(&state.uid()) {
            self.cc += 1;
        // read aid from the back of the structure, last byte ?
            self.logic(
                state, 
                state.fd.data().iter().last().unwrap().clone() - 2,//[state.fd.data().len()-1] - 2,
                cid - 0x100
            );
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

        let aid = 1 + call.einfo()[0] - 2; // - Users::Attacker, get from config!!
        let aid = if aid > 7 { 0 } else { aid };

        let cid = unsafe { std::mem::transmute::<_, usize>(call.id()) } - 0x100; // - 0x100 from config, first custom call id
        self.logic(state, aid, cid);
        true
    }

    fn logic(&mut self, state: &StateInfo, aid: u8, cid: usize) {
if aid > 3 {
    println!("AID GOES WRONG : {aid:?} for {cid:?} || {:?}", state.name);
    std::process::exit(0);
}
        let aim: u8 = 2u8.pow(aid as u32).into();
        self.hitmap[cid] |= aim;
        let _ = self.hitset.insert(cid);
/*
if aim != self.hitmap[cid] {
    println!(" ATTACKER : {:X} -> |{:?}|", self.hitmap[cid], self.hitmap[cid].count_ones());
    std::process::exit(0)
}
*/

        for &i in &self.hitset { // go over all calls
            if 1 != aim && 1 != self.hitmap[i] && 0 == aim & self.hitmap[i] {
                continue // this cid is not hit by this attacker yet
            }

            let aid = if 1 == self.hitmap[i] {
                (self.hitmap[cid].count_ones() - 1) as usize
            } else {
                (self.hitmap[i].count_ones() - 1) as usize
            };
/*
if aid > 1 || (aid > 0 && 0 == 1 & self.hitmap[i])
//if aid > 0 && 1 == 1 & self.hitmap[i]
{
    println!(" OK MULTIPLE ATTACKERS HAPPENING");
    println!(" ATTACKER : {:X} -> |{:?}|", self.hitmap[i], self.hitmap[i].count_ones());
    std::process::exit(0)
}
*/
            // ok all cid we already hit - by this attacker -, we mark
            //self.hitmap[cid + aid * MAX_CID + i * (MAX_AID * MAX_CID)] = 1;
            unsafe {
                ::log_feedback(
                    cid + aid * MAX_CID + i * (MAX_AID * MAX_CID),
                    1,
                    self.cc,
                    self.tictoc.as_ref().unwrap().elapsed().as_millis()
                );
            }
        }
    }
    pub fn dtor(&mut self, _state: &StateInfo) { }
    pub fn revert(&mut self, _info: &StateInfo, _call: &Call, mask: WantedMask) { }
    pub fn stop(&mut self) { }
}

common::callback_proxy!(FeedBack);

pub fn observers() -> (
    Option<Box<dyn IStateObserver>>,
    Option<Box<dyn ICallObserver>>,
) {
    let lookup = Arc::new(RwLock::new(FeedBack::new()));
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
