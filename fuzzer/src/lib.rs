#[macro_use]
extern crate lazy_static;

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

extern crate libbfl;
use libbfl::info::PocDataHeader;

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
    
}

fn load_plugins(mut plugins: Vec<Observer>) {
    for plugin in plugins.iter_mut() {
        if FZZCONFIG.noisy {
            plugin.stats();
        }

        if let Some(obs) = plugin.state_obs().take() {
            bananaq::attach_state_observer(obs);
        }
        if let Some(obs) = plugin.call_obs().take() {
            bananaq::attach_call_observer(obs);
        }
        //store here reloaders!!
    }
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

    cLLVMFuzzerTestOneInput(std::mem::transmute(0usize), std::mem::transmute(0usize));

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

//    FuzzyState::fuzz(Box::new(MarioState::spawn())).join();

    bananaq::detach_observers();

    while !bananaq::empty() { }

    cLLVMFuzzerTestJoin();

//generic::append_file_raw("info.txt", "}".as_bytes());

    0
}


pub unsafe fn libafl_targets_libfuzzer_init(_argc: *const i32, _argv: *const *const *const u8) -> i32 {
    if FZZCONFIG.noisy {
        println!("{}", FZZCONFIG.version);
    }
//generic::append_file_raw("info.txt", "\ninit".as_bytes());
//    panic!("OK RESOLVED EXTERNS TO BANANA");
    cLLVMFuzzerInitialize(std::mem::transmute(0usize), std::mem::transmute(0usize))
}
