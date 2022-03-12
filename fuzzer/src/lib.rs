#[macro_use]
extern crate lazy_static;

use std::sync::RwLock;

//extern crate rand;
//use rand::Rng;

extern crate core;
use core::banana::bananaq;

use std::{
    thread,
    time
};
use std::sync::mpsc::{
    Sender,
    Receiver,
};
use std::sync::mpsc;

extern crate plugs;
use plugs::{Observer, Plugins};

use core::config::FZZCONFIG;
use core::state::id::StateTableId;
//use core::state::state::IFuzzyObj;
use core::exec::fd_info::Fd;
use core::banana::looper::FuzzyState;

mod common;
//use common::table::*;

mod args;

mod calls;

mod states;
use states::mario::state::MarioState;
//use states::coins::state::CoinsState;

extern crate libsmb;

extern crate libbfl;
use libbfl::{
    KNOWLEDGE_MAP, 
    info::PocDataHeader, 
    poc::{REPROED, POCDROP},
    shmem::ShmemData,
};

pub fn push_state(_id: StateTableId, _fd: &Fd) {
/*
    type TFuzzyObj = Box<dyn IFuzzyObj>;
    if let Some(mut fuzzy_obj) = match StateIds::from(id.clone()) {
//        StateIds::FdSocket => Some::<TFuzzyObj>(Box::new(SocketState::dup(fd, false))),
        _ => None,
        }
    {
        if !fuzzy_obj.is_online() {
            return
        }
        FuzzyState::fuzz(fuzzy_obj);
    }
*/
}

fn push_fuzz() {
    /*
    for i in 0..5 {
        FuzzyState::fuzz(Box::new(MarioState::enemy(i)));
    }
    FuzzyState::fuzz(Box::new(MarioState::shroom()));
    */
    FuzzyState::fuzz(Box::new(MarioState::spawn())).join();
}

fn load_plugins(mut plugins: Vec<Observer>) {
    for plugin in plugins.iter_mut() {
//        if FZZCONFIG.noisy {
//            plugin.stats();
//        }

        if let Some(obs) = plugin.state_obs().take() {
            bananaq::attach_state_observer(obs)
        }
        if let Some(obs) = plugin.call_obs().take() {
            bananaq::attach_call_observer(obs)
        }
        //store here reloaders!!
    }
    bananaq::attach_call_observer(libsmb::observer())
}

extern "C" {
    fn cLLVMFuzzerInitialize(arg: *const i32, argv: *const * const *const u8) -> i32;
    fn cLLVMFuzzerTestOneInput(data: *mut u8, size: usize) -> i32;
    fn cLLVMFuzzerTestJoin();
}

//we need this mut to be able toupdate poc call desriptions, mainly KIN members
pub unsafe fn LLVMFuzzerTestOneInput(poc_mem: *mut u8, data: *const u8, size: usize) -> i32 {
//generic::append_file_raw("info.txt", "{".as_bytes());
 //   println!("TESTONE INPUT TO BANANA {:X}", size);
    KNOWLEDGE_MAP[0] = 0;
    POCDROP = false;

    if 0 == size {
        return -3
    }
 
    let mut header = ShmemData::new(66, std::mem::transmute(data)).head();
    let header = header.clone();
    let do_gen = !0 != header.split_at || !0 != header.insert_ind;

    if 1 == cLLVMFuzzerTestOneInput(
        std::mem::transmute(0usize), std::mem::transmute(0usize)) {
        return -2//test mode
    }

    let mut cfg = match plugs::load_cfg() {
        Ok(cfg) => cfg,
        Err(e) => panic!("[BFL] config err {}", e)
    };

    let bfl = if let Some(ref mut bfl) = cfg.core.bfl {
        bfl.shmem = std::mem::transmute(data);
        bfl.pocmem = std::mem::transmute(poc_mem);
//println!("[BFL] changed config : {:X} and {:X}", bfl.shmem, bfl.pocmem);
        bfl
    } else { panic!("[BFL] unable to access bfl config") };

    let plugins = plugs::Plugins::new(cfg, &push_state);
    load_plugins(plugins.observers);

    push_fuzz();

    while !bananaq::empty() { }

    bananaq::detach_observers();

    cLLVMFuzzerTestJoin();

    let reproed = REPROED;

if !reproed { generic::append_file_raw_with_limit("banana.txt", b"$", 1000); }
else { generic::append_file_raw_with_limit("banana.txt", b"@", 1000); }
    REPROED = false;

if do_gen { generic::append_file_raw_with_limit("banana.txt", b"G", 1000); }
else { generic::append_file_raw_with_limit("banana.txt", b"T", 1000); }

//generic::append_file_raw("info.txt", "}".as_bytes());
    if FZZCONFIG.noisy {
        if do_gen && reproed && POCDROP {
            print!("I({:?}/{:?}=>{:?})", 
            header.insert_ind, 
            header.calls_count,
            ShmemData::new(66, std::mem::transmute(poc_mem)).head().calls_count);
        } else if reproed {
            print!("R");
        } else if do_gen {
            print!("F");
        } else {
            print!("Z");
        }
    }
    if !do_gen && reproed && 1 == KNOWLEDGE_MAP[0] { 
        let poc_vec = std::slice::from_raw_parts(data, size).to_vec();
println!("GOT NEW STUFF! {size}");
        static mut CORPUS_N: usize = 0;
        CORPUS_N += 1;
//        generic::write_file_raw(
//            format!("corpus/bfl_{:?}", CORPUS_N).as_str(),
//            &poc_vec);
    }

//println!("................... REPROED ? => {:?};", reproed);

    if !do_gen && reproed { 0 } else { -1 }
}


pub unsafe fn libafl_targets_libfuzzer_init(_argc: *const i32, _argv: *const *const *const u8) -> i32 {
    if FZZCONFIG.noisy {
        println!("{}", FZZCONFIG.version);
    }
//generic::append_file_raw("info.txt", "\ninit".as_bytes());
//    panic!("OK RESOLVED EXTERNS TO BANANA");
    cLLVMFuzzerInitialize(std::mem::transmute(0usize), std::mem::transmute(0usize))
}

pub unsafe fn feedback_maps() -> &'static mut[u8] {
    &mut KNOWLEDGE_MAP
}
