#![feature(map_first_last)]
#[macro_use]
extern crate serde_derive;
extern crate serde;


use std::collections::{HashMap, BTreeSet, BTreeMap};
use std::time::Instant;

extern crate rand;
use rand::Rng;

extern crate core;

use core::exec::call::Call;
use core::banana::observer::{ICallObserver, IStateObserver, WantedMask};
use core::state::state::StateInfo;

extern crate common;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BehaviorConfig {
    // level, object, offset, call-id
    // important not hashmap but always-key-same-ordered btreemap ( given same input )
    info: BTreeMap<String, Vec<(usize, String, String)>>,
}

#[derive(Debug)]
struct BehaviorInfo {
    covered: bool,
    sname: String,
    cname: String,
    actor: String,
}

struct Behavior {
    wanted: Option<WantedMask>,
    cc: usize,
    level: usize,
    tictoc: Option<Instant>,


    ctormap: HashMap<u64, String>,

    order: usize,
    actors: HashMap<u8, String>,
    behavior: Vec<Vec<BehaviorInfo>>,
}
impl Behavior {
    fn new(cfg: &BehaviorConfig) -> Self {
        let mut behavior: Vec<Vec<BehaviorInfo>> = vec![];

        // carefull, if cfg.info HashMap we can get different loads different order
        // it will then affect behavior lol!!
        for key in cfg.info.keys() {
            for info in &cfg.info[key] {
                while behavior.len() <= info.0 {
                    behavior.push(vec![]);
                }
                behavior[info.0].push(
                    BehaviorInfo{
                        covered: false,
                        sname: info.1.to_string(),
                        cname: info.2.to_string(),
                        actor: key.to_string(),
                    });
            }
        }
        /*
        behavior
            .iter()
            .inspect(|&info| println!("-> {info:?}"))
            .count();
        */

        Self {
            wanted: None,
            cc: 0,
            level: 0,
            tictoc: None,


            ctormap: HashMap::new(),

            order: 0,
            actors: HashMap::new(),
            behavior: behavior,
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
        let _ = self.ctormap.insert(state.uid(), call.name().to_string());
        Ok(true)
    }

    fn ctor(&mut self, state: &StateInfo) -> bool {
        if let Some(ref cname) = self.ctormap.get(&state.uid()) {
            self.cc += 1;
        // read aid from the back of the structure, last byte ?
            self.logic(
                state, 
                &cname.to_string(), 
                state.fd.data().iter().last().unwrap().clone());
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
        let slot = call.einfo()[0];
        self.logic(state, &call.name().to_string(), slot);
        true
    }

    fn logic(&mut self, state: &StateInfo, cname: &String, slot: u8) {
        if self.behavior.len() == self.order {
            return
        }
        if !self.resolve_by_slot(state, cname, slot) {
            return
        }
        if 0 != self.behavior[self.order][0].cname.len() {
            return // only checkpoints to save
        }
        unsafe {
            ::log_feedback(
                self.order,
                1,
                self.cc,
                self.tictoc.as_ref().unwrap().elapsed().as_millis()
            );
        }
std::fs::write(format!("ORDER.{:?}.INFO", self.order), format!("{:?}\n\tw/actors {:?}", self.behavior[self.order+1], self.actors));
        self.order += 1;
    }

    fn resolve_by_slot(&mut self, state: &StateInfo, cname: &String, slot: u8) -> bool {
        if 1 == slot {
            return self.resolve(state, cname, 1).is_ok()
        }
        const N_ATTACKER: u8 = 3;
        for i in 0u8.. {
            for j in 0..N_ATTACKER {
                let actor = 1 + i * N_ATTACKER + j;
                if actor > 7 {
                    return false
                }
                let actor = 2u32.pow(actor as u32) as u8;
                if 0 == actor & slot {
                    continue
                }
                if let Ok(()) = self.resolve(state, cname, 2u32.pow(1 + j as u32) as u8) {
                    return true
                }
                return false
            }
        }
        panic!("..")
    }

    fn resolve(&mut self, state: &StateInfo, cname: &String, slot: u8) -> Result<(), ()> {
        let bid = self.resolve_call(state, cname, slot)?;
        if !self.resolve_actor(state, bid, slot) {
            return Err(())
        }

        let behavior = &mut self.behavior[self.order][bid];
        behavior.covered = true;

        if self.behavior[self.order].iter().all(|behavior| behavior.covered) {
            std::fs::write(format!("ORDER.{:?}.INFO", self.order), format!("{:?}\n\tw/actors {:?}", self.behavior[self.order+1], self.actors));
            self.order += 1
        }
        Ok(())
    }
    fn resolve_call(&self, state: &StateInfo, cname: &String, slot: u8) -> Result<usize, ()> {
        if 0 == self.behavior[self.order][0].cname.len() {
            assert!(1 == self.behavior[self.order].len(), "behavior checkpoint is single level entry!");
            return Ok(0) // checkpoint
        }
        let mut bid = None;
        for (i, behavior) in self.behavior[self.order]
            .iter()
            .enumerate() 
            .filter(|(_, behavior)| !behavior.covered 
                && behavior.cname.eq(cname) 
                && (behavior.sname.eq(&state.name) || 0 == behavior.sname.len()))
        {
            if behavior.actor.eq("noone") {
                return Ok(i)
            }
            bid = if let Some(ctor) = self.actors.get(&slot) {
                if ctor.eq(&behavior.actor) { // is this state corresponding actor
                    return Ok(i)
                } else { bid } // or it took role of other one ?
            } else { Some(i) };
        }
        bid.ok_or(())
    }
    fn resolve_actor(&mut self, state: &StateInfo, bid: usize, slot: u8) -> bool {
        let behavior = &self.behavior[self.order][bid];
        if behavior.actor.eq("noone") {
            return true // checkpoint
        }
        if self.actors.contains_key(&slot) {
            assert!(self.actors[&slot].eq(&behavior.actor));
            return true // ok its yours
        }
        for (&key, actor) in &self.actors {
            assert!(key != slot);
// did somebody else took this actor role already ?!
            if actor.eq(&behavior.actor) { 
                return false 
            }
        }
        self.actors.insert(slot, behavior.actor.to_string()); 
        true
    }

    pub fn dtor(&mut self, _state: &StateInfo) { }
    pub fn revert(&mut self, _info: &StateInfo, _call: &Call, mask: WantedMask) { }
    pub fn stop(&mut self) { }
}

common::callback_proxy!(Behavior);

pub fn observers(
    cfg: &Option<BehaviorConfig>,
) -> (
    Option<Box<dyn IStateObserver>>,
    Option<Box<dyn ICallObserver>>,
) {
    let lookup = Arc::new(RwLock::new(Behavior::new(&cfg.as_ref().unwrap())));
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
