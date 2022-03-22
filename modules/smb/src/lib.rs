extern crate core;
use core::exec::call::Call;
use core::banana::observer::{ICallObserver, IStateObserver};
use core::state::state::StateInfo;

use std::sync::{
    Arc, 
    atomic::{AtomicU64, Ordering},
};

struct SuperMarioBros2 {
    syncer: Option<Arc<SuperMarioBros2>>,
    theone: AtomicU64,
    target: AtomicU64,
    dcount: AtomicU64,
}

impl ICallObserver for SuperMarioBros2 {
    fn notify(&self, state: &StateInfo, call: &mut Call) -> bool {
        if 0 == state.level {
//println!("--> notify CTOR");
            return true // ctors may pass through by default
        }
        if state.finished {
            return true // dtors are allowed too
        }

        let syncer = self.syncer.as_ref().unwrap();

//println!("-SMB2-> notify {:?}+{:?} :: {:?} :: {:?}", state.uid(), u64::from(state.id), state.level, state.fd);
        let uid = syncer.theone.load(Ordering::SeqCst);

        if state.uid() == uid { // approved
            if 0x12u64 != call.id().into() { // everything except loader
                return true
            }
            // otherwise loader tolds this object not to move
            syncer.theone.store(0, Ordering::SeqCst) // therefore give chane to other object's loader
        }
/*// approximation for dtor, we need to callback lol ..
        if call.n_attempts() > 5 
            && 0x12u64 == call.id().into() 
        {
            self.theone.store(0, Ordering::SeqCst);
            return false // ok maybe it was dtored ...
        }
*/
        if 0 != uid { // new call to do ?
//println!("denying {uid}");
            return false // nope we still need to finish one
        }

        if 0x12u64 != call.id().into() {
//println!("WRONG CID");
            return false
        }// only loaders are approve starters!

        assert!(0x12u64 == call.id().into(), "[SMB2] ({:?}) non-loader competing for action !!", call.id()); // only loaders are approve starters!

        //we try ( cuze 3==%4 passtrough ) to force to do not only mario
        if 0x100u64 == state.id.into() // classy mario
            && 0 != syncer.target.load(Ordering::SeqCst) // we have non classy inside
//            && 3 != self.dcount.fetch_add(1, Ordering::Relaxed) % 4 // give chance for the others
//            && 99 != self.dcount.fetch_add(1, Ordering::Relaxed) % 100 // give chance for the others
        { return false }
/*
        {
println!("waiting for SMBC2 cool stuff {:?}", self.theone.load(Ordering::SeqCst));
            return false
        }
//        { println!("deny mario"); return false }
*/
        // we will xor targets once it will move, so target == 0 once a second move

        if let Ok(v) = syncer.theone.compare_exchange(0, state.uid(), 
            Ordering::SeqCst, Ordering::SeqCst) {
            return 0 == v
        }
        false
    }

    fn aftermath(&self, state: &StateInfo, call: &mut Call) {
        let syncer = self.syncer.as_ref().unwrap();
        if state.uid() != syncer.theone.load(Ordering::SeqCst) {
            return
        }
        assert!(state.uid() == syncer.theone.load(Ordering::SeqCst), 
            "[SMB2] aftermath with different target {:?} vs {:?}",
            state.uid(), syncer.theone);

        if 0x11u64 != call.id().into() {//move mario
            return // wait for move call to happen
        }

        syncer.theone.store(0, Ordering::SeqCst);

        if 0x100u64 != state.id.into() { // ok 1:#enemies in queue to choose
            return syncer.target.store(0, Ordering::SeqCst)
        }

//xor will keep active targets, but giving chance to classy mario too
        syncer.target.fetch_or(
            call.args_view(1)
                .data()[8..][2..][2..]
                    .iter()
                    .step_by(2)
                    .enumerate()
                    .filter(|(i, x)| &0 != *x)
                    .map(|(i, _)| 2 << i)
                    .fold(0, |acc, t| acc | t),
            Ordering::SeqCst);

        let n_e = syncer.target.load(Ordering::SeqCst);
        if 0 != n_e { println!("TARGETS -> : {n_e}") }
    }
}

impl IStateObserver for SuperMarioBros2 {
    fn notify_ctor(&self, state: &StateInfo) -> bool {
        true
    }

    fn notify_dtor(&self, state: &StateInfo) { 
        let syncer = self.syncer.as_ref().unwrap();
        if let Ok(v) = syncer.theone.compare_exchange(state.uid(), 0,
            Ordering::SeqCst, Ordering::SeqCst) {
            println!("[smb] one mario objective falls down <{v}>")
        }
    }
}

impl SuperMarioBros2 {
    fn new() -> Self {
        SuperMarioBros2 {
            syncer: None,
            theone: AtomicU64::new(0),
            target: AtomicU64::new(0),
            dcount: AtomicU64::new(0),
        }
    }
    fn fork(syncer: &Arc<SuperMarioBros2>) -> Self {
        SuperMarioBros2 {
            syncer: Some(Arc::clone(syncer)),
            theone: AtomicU64::new(0),
            target: AtomicU64::new(0),
            dcount: AtomicU64::new(0),
        }
    }
}

pub fn observer() -> (
    Option<Box<dyn IStateObserver>>,
    Option<Box<dyn ICallObserver>>,
    ) { 
//    Box::new(SuperMarioBros2::new()) }
    let syncer = Arc::new(SuperMarioBros2::new());
    let syncer_o = SuperMarioBros2::fork(&Arc::clone(&syncer));
    let syncer_s = SuperMarioBros2::fork(&Arc::clone(&syncer));

    assert!(!syncer_o.syncer.is_none() && !syncer_s.syncer.is_none());

    (
    Some(Box::new(syncer_s)),
    Some(Box::new(syncer_o)),
    )
}

#[no_mangle]
fn test_interop() {}
