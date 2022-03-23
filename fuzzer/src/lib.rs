#![feature(backtrace)]

#[macro_use]
extern crate lazy_static;

use std::backtrace::Backtrace;

extern crate core;
use core::banana::bananaq::{self,FuzzyQ};
use core::banana::queue;

use std::{
    sync::{Arc, Weak, RwLock},
};

use std::{
    thread,
    time
};

extern crate plugs;
use plugs::Observer;

use core::config::FuzzyConfig;
use core::state::{state::IFuzzyObj, id::StateTableId};
use core::exec::fd_info::Fd;
use core::banana::looper::FuzzyState;

mod common;
use common::table::*;

mod args;

mod calls;

mod states;
use states::mario::state::MarioState;
use states::coins::state::CoinsState;

extern crate libbfl;
use libbfl::{
    info::PocDataHeader, 
    poc::{REPROED, POCDROP},
    shmem::ShmemData,
};

extern crate libbijon;

#[no_mangle]
pub fn push_state(bananaq: &Weak<FuzzyQ>, id: StateTableId, fd: &Fd) {
    type TFuzzyObj = Box<dyn IFuzzyObj>;
    if let Some(mut fuzzy_obj) = match StateIds::from(id.clone()) {
        StateIds::FdCoins => Some::<TFuzzyObj>(Box::new(
            CoinsState::alert(bananaq.clone(), fd.data(), id))),
        _ => None
        } 
    {
        if !fuzzy_obj.is_online() {
            return
        }
        if let Err(msg) = FuzzyState::fuzz(fuzzy_obj) {
            println!("[bananaq] no more pushing objects on bananaq#? {msg}");
        }
    }
}

fn push_fuzz(
    banana: &Arc<FuzzyQ>
    ) -> Result< std::thread::JoinHandle< Result<(), String> >, &'static str > 
{
    FuzzyState::fuzz(
        Box::new(
            MarioState::spawn(
                Arc::downgrade(&Arc::clone(banana)))))
}

fn load_plugins(banana: &mut Arc<FuzzyQ>, noisy: bool, mut plugins: Vec<Observer>) {
    for plugin in plugins.iter_mut() {
        if noisy {
            plugin.stats();
        }

        if let Some(obs) = plugin.state_obs().take() {
            bananaq::attach_state_observer(banana, obs)
        }
        if let Some(obs) = plugin.call_obs().take() {
            bananaq::attach_call_observer(banana, obs)
        }
        //store here reloaders!!
    }
}

extern "C" {
    fn cLLVMFuzzerInitialize(arg: *const i32, argv: *const * const *const u8) -> i32;
    fn cLLVMFuzzerTestOneInput(data: *mut u8, size: usize) -> i32;
    fn cLLVMFuzzerTestJoin();
    fn reset_coins();
}

unsafe fn exec_input(
    bijon: bool, 
    fzzcfg: &FuzzyConfig,
    poc_mem: *mut u8, 
    data: *const u8, 
    size: usize
    ) -> Result<(), String>
{
    REPROED = false;
    
    if 1 == cLLVMFuzzerTestOneInput(
        std::mem::transmute(0usize), std::mem::transmute(0usize)) {
        return Err(format!("[bananafzz] cLLVMFuzzerTestOneInput error"))
    }

    let mut banana = Arc::new(RwLock::new(queue::FuzzyQ::new(
                fzzcfg.clone()))); 

    println!("TESTONE INPUT TO BANANA {:X}", size);

    let mut cfg = match plugs::load_cfg() {
        Ok(cfg) => cfg,
        Err(e) => panic!("[BFL] config err {}", e)
    };

    if let Some(ref mut bfl) = cfg.core.bfl {
        bfl.shmem = std::mem::transmute(data);
        bfl.pocmem = std::mem::transmute(poc_mem);
//println!("[BFL] changed config : {:X} and {:X}", bfl.shmem, bfl.pocmem);
    } else { panic!("[BFL] unable to access bfl config") };

    let plugins = plugs::Plugins::new(cfg);
    load_plugins(&mut banana, fzzcfg.noisy, plugins.observers);

    if bijon {// go for feedback coverage
        bananaq::attach_call_observer(&mut banana, libbijon::observer())
    }

    println!("PLUGINS LOEADED");

    reset_coins();
    let mario = push_fuzz(&banana)?;

    wait_for_fuzzing_over(&Arc::downgrade(&banana), &fzzcfg);

    if let Err(msg) = mario.join() {
        println!("[fuzzing message] : <{:?}>", msg);
    }
    cLLVMFuzzerTestJoin();

    println!("QUEUE FINISHED");
    Ok(())
}

fn wait_for_fuzzing_over(bananaq: &Weak<FuzzyQ>, fzzcfg: &FuzzyConfig) {
    for _ in 0..fzzcfg.active_time / fzzcfg.push_sleep {
        match bananaq::is_active(bananaq) {
            Ok(is_active) => if !is_active {
                break 
            },
            Err(msg) => {
                println!("[bananaq] over, go for next input, banana#msg : <{}>", msg);
                break
            }
        }
        thread::sleep(time::Duration::from_millis(
                fzzcfg.push_sleep));
    }
    if let Err(msg) = bananaq::stop(bananaq) {
        println!("[bananaq] done waiting, queue stoped, banana#msg : <{}>", msg);
    }
}

use std::sync::RwLockWriteGuard;

#[allow(improper_ctypes)]
extern "C" {
    fn banana_feedback<'a>() -> RwLockWriteGuard<'a, Vec<Vec<u8>>>;
}

//we need this mut to be able toupdate poc call desriptions, mainly KIN members
#[allow(non_snake_case)]
pub unsafe fn LLVMFuzzerTestOneInput(poc_mem: *mut u8, data: *const u8, size: usize) -> i32 {
//generic::append_file_raw("info.txt", "{".as_bytes());
 //   println!("TESTONE INPUT TO BANANA {:X}", size);
    POCDROP = false;

    if 0 == size {
        return -3
    }
    
    let header = ShmemData::new(66, std::mem::transmute(data)).head().clone();
    let poc = ShmemData::new(
        header.magic,
        std::mem::transmute(poc_mem)).data().to_vec();

    let do_gen = !0 != header.split_at || !0 != header.insert_ind;

    let fzzcfg = FuzzyConfig::new();

    if fzzcfg.noisy {
        print!("\n$N*I({size})->PID:<{:?}>//{:?}$\n bakctrace : {:?}", std::process::id(), header.insert_ind, Backtrace::force_capture());
    }

    let mut msg = match exec_input(!do_gen, &fzzcfg, poc_mem, data, size) {
        Err(e) => e,
        _ => format!("Fuzzing was done OK"),
    };

    if REPROED && POCDROP {
        assert!(do_gen);
        for _ in 0..10 {
            let poc_size = ShmemData::new(
                header.magic,
                std::mem::transmute(poc_mem)).head().total_size;
            msg = match exec_input(false, &fzzcfg, poc_mem, poc_mem, poc_size) {
                Err(e) => e,
                _ => format!("Fuzzing was done OK"),
            };
            if REPROED {
                break
            }
        }

//        while !REPROED { println!("YES IS THAT HAPPENING !! <{msg}> |{} vs {}|", poc_size, size) }

        if !REPROED {
            if poc[..8].iter().sum::<u8>() > 0 {
                std::slice::from_raw_parts_mut(poc_mem, poc.len())
                    .clone_from_slice(&poc);
            } else {
                std::slice::from_raw_parts_mut(poc_mem, std::mem::size_of::<PocDataHeader>())
                    .fill(0);
            }
        }
    }

    let reproed = REPROED;
    println!("INPUT OUT : <{msg}> stats::|{:?}|", (do_gen, reproed, POCDROP));

    if !do_gen && !reproed {// if do_gen bijon is offline
        let n_failed_to_repro = banana_feedback()
            .drain(2..)
            .count();
        println!("[BFL] failed to repro : {n_failed_to_repro}");
    }
//    let reproed = REPROED;

    if fzzcfg.noisy {
        if do_gen && reproed && POCDROP {
for _ in 0..1000 {print!("I({:?}/{:?}=>{:?})", 
            header.insert_ind, 
            header.calls_count,
            ShmemData::new(66, std::mem::transmute(poc_mem)).head().calls_count) }
        } else if reproed {
            print!("R");
        } else if do_gen {
            print!("F");
        } else {
            print!("Z");
        }
    }
/*
    if !do_gen && reproed && 1 == KNOWLEDGE_MAP[0] { 
        let poc_vec = std::slice::from_raw_parts(data, size).to_vec();
println!("GOT NEW STUFF! {size}");
        static mut CORPUS_N: usize = 0;
        CORPUS_N += 1;
        generic::write_file_raw(
            format!("corpus/bfl_{:?}", CORPUS_N).as_str(),
            &poc_vec);
    }
*/
    if !do_gen && reproed { 0 } else { -1 }
}


pub unsafe fn libafl_targets_libfuzzer_init(_argc: *const i32, _argv: *const *const *const u8) -> i32 {
    let fzzcfg = FuzzyConfig::new();
    if fzzcfg.noisy {
        println!("{}", fzzcfg.version);
    }
//generic::append_file_raw("info.txt", "\ninit".as_bytes());
//    panic!("OK RESOLVED EXTERNS TO BANANA");
    cLLVMFuzzerInitialize(std::mem::transmute(0usize), std::mem::transmute(0usize))
}
