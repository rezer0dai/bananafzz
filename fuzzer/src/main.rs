#![feature(backtrace)]

#[macro_use]
extern crate lazy_static;

extern crate rand;
use rand::Rng;

extern crate core;
use core::banana::bananaq::{self,FuzzyQ};
use core::banana::queue;

use std::{
    sync::{Arc, Weak, RwLock},
};

extern crate plugs;
use plugs::{Observer, Plugins};

use core::config::FZZCONFIG;
use core::state::id::StateTableId;
use core::state::state::IFuzzyObj;
use core::exec::fd_info::Fd;
use core::banana::looper::FuzzyState;

mod common;
use common::table::*;

mod calls;

mod states;
use states::mario::state::MarioState;
use states::coins::state::CoinsState;

extern crate libbfl;
use libbfl::info::PocDataHeader;
use libbfl::poc::REPROED;

extern crate libsmb;

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
        FuzzyState::fuzz(fuzzy_obj);
    }
}

fn load_plugins(banana: &mut Arc<FuzzyQ>, mut plugins: Vec<Observer>) {
    for plugin in plugins.iter_mut() {
        if FZZCONFIG.noisy {
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
pub fn LLVMFuzzerTestOneInput_mut(
    banana: &Arc<FuzzyQ>, 
    data: *mut u8, 
    size: usize
    ) -> Result< std::thread::JoinHandle< Result<(), String> >, &'static str > 
{
    FuzzyState::fuzz(
        Box::new(
            MarioState::spawn(
                Arc::downgrade(&Arc::clone(banana)))))
}

extern "C" {
    fn reset_coins();
}

//we need this mut to be able toupdate poc call desriptions, mainly KIN members
pub unsafe fn LLVMFuzzerTestOneInput(poc_mem: *mut u8, data: *const u8, size: usize) -> Result<(), String> {
    let mut banana = Arc::new(RwLock::new(queue::FuzzyQ::new()));

    println!("[BANANQ#{:X}] TESTONE INPUT TO BANANA {:X}", bananaq::qid(&Arc::downgrade(&banana)).unwrap(), size);

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

    let plugins = plugs::Plugins::new(cfg);
    load_plugins(&mut banana, plugins.observers);

    println!("PLUGINS LOEADED");

    reset_coins();
    let out = LLVMFuzzerTestOneInput_mut(&banana, std::mem::transmute(data), size)?;
    if let Err(msg) = out.join() {
        println!("[fuzzing message] : <{:?}>", msg);
    }
    let alive = Arc::downgrade(&banana);
//println!("going to DROP");
//    banana.write().unwrap().stop();

    println!("QUEUE FINISHED");
    Ok(())
}

pub fn main() {
    println!("OK RESOLVED EXTERNS TO BANANA");

    if FZZCONFIG.noisy {
        println!("{}", FZZCONFIG.version);
    }
//    panic!("OK RESOLVED EXTERNS TO BANANA");

    let mut poc = [0x42u8; 0x10000];
    poc[..0x100].fill(0);
    generic::data_mut_unsafe::<PocDataHeader>(&mut poc).total_size = std::mem::size_of::<PocDataHeader>();
    generic::data_mut_unsafe::<PocDataHeader>(&mut poc).split_at = !0;
// create dummy pocjust header

    let mut counter = 0;
    loop {
/*
        counter += 1;
        if counter > 100 {
            break
        }
// LOAD CRASH
        //f887ea2517424f2f
        if let Ok(data) = generic::read_file_raw(&format!("bfl_in/{counter:X}")) {//3e500a00d08eccbe//42ce44f0e0a28214") {
            poc[..data.len()].clone_from_slice(&data);
        } else { continue }
*/

        let cc = generic::data_const_unsafe::<PocDataHeader>(&poc).calls_count;
        generic::data_mut_unsafe::<PocDataHeader>(&mut poc).magic = 66;
//        let ii = if cc > 1 { 1 } else {cc};//rand::thread_rng().gen_range(0..=cc);
        let ii = if cc > 0 { rand::thread_rng().gen_range(1..=cc) } else {cc};
        if 0 == counter {//not repro, but generation
            generic::data_mut_unsafe::<PocDataHeader>(&mut poc).insert_ind = ii;
        }

        let total_size = generic::data_const_unsafe::<PocDataHeader>(&poc).total_size;
        if 0x2000 + total_size > poc.len() {
            break
        }
/*
        println!("NOW SIZE IS : {total_size} -> {:?}/{i}=>{:?}",
            generic::data_const_unsafe::<PocDataHeader>(&poc).insert_ind,
            generic::data_const_unsafe::<PocDataHeader>(&poc).calls_count,
            );
*/
        let data = poc[..total_size].to_vec();
//copy from poc to data
println!("\n\n ***********\n\n INSERTING -> {:?}", (ii, cc));
//
unsafe { REPROED = false; }

        if let Err(_) = unsafe {
            LLVMFuzzerTestOneInput(std::mem::transmute(poc.as_ptr()), std::mem::transmute(data.as_ptr()), data.len())
        } { continue }

        if 0 != counter {
            unsafe { println!("reproed ? {REPROED} -> {counter:X}") }
            continue;
        }

        if total_size != generic::data_mut_unsafe::<PocDataHeader>(&mut poc).total_size {
            if 0 != ii && cc != ii { println!("****************>>> INSERTED {ii}/{cc}"); }
        }

        let total_size = generic::data_const_unsafe::<PocDataHeader>(&poc).total_size;
        let data = poc[..total_size].to_vec();

//        assert!(!0 == generic::data_const_unsafe::<PocDataHeader>(&poc).insert_ind);
        if !0 != generic::data_const_unsafe::<PocDataHeader>(&poc).insert_ind {
            poc[..data.len()].clone_from_slice(&data);
            continue//like we run out of ctors.. and we did try to add one more at the begining
        }
println!("\n\n ***********\n\n CLEANING -> {:?} |", generic::data_const_unsafe::<PocDataHeader>(&poc).calls_count);
        if let Err(_) = unsafe {
            LLVMFuzzerTestOneInput(std::mem::transmute(poc.as_ptr()), std::mem::transmute(data.as_ptr()), data.len())
        } { continue }

    let reproed = unsafe { REPROED };
if !reproed { generic::append_file_raw("banana.txt", b"$"); }
else { generic::append_file_raw("banana.txt", b"@"); }
assert!(reproed);

//        assert!(total_size == generic::data_mut_unsafe::<PocDataHeader>(&mut poc).total_size);
        let total_size = generic::data_mut_unsafe::<PocDataHeader>(&mut poc).total_size;
        let data = poc[..total_size].to_vec();
        generic::write_file_raw(format!("./bfl_in/{:X}", cc).as_str(), &data);

        assert!(!0 == generic::data_const_unsafe::<PocDataHeader>(&poc).insert_ind);
println!("\n\n ***********\n\n TESTING");
        if let Err(_) = unsafe {
            LLVMFuzzerTestOneInput(std::mem::transmute(poc.as_ptr()), std::mem::transmute(data.as_ptr()), data.len())

        } { continue }

println!("***** FINAL SIZE IS : {:?} x {:?} -> {:?}",
    generic::data_const_unsafe::<PocDataHeader>(&poc).total_size, total_size,
    generic::data_const_unsafe::<PocDataHeader>(&poc).calls_count,
    );

    }

}
/*
        if i > 0 {
            if let Ok(data) = generic::read_file_raw(format!("./bfl_in/{:X}", cc).as_str()) {
                poc[..data.len()].clone_from_slice(&data);
            } else { panic!("NOP NO CRASH FILE LOCATED") }
        }
        assert!(!0 == generic::data_const_unsafe::<PocDataHeader>(&poc).insert_ind);
        let data = poc[..total_size].to_vec();
*/

mod args;
use args::smb2::Move;

#[no_mangle] 
unsafe fn load_pos(pos: *mut u8) { 
    std::slice::from_raw_parts_mut(pos, 25)[Move::Mario as usize - 1] = 1;
    let xy = 0 + std::slice::from_raw_parts_mut(pos, 25)[0];
    std::slice::from_raw_parts_mut(pos, 27 - 1)[8..Move::Shroom as usize - 1]
        .fill(8);

    std::slice::from_raw_parts_mut(pos, 8)
        .fill(xy);

    print!("l") 
}
#[no_mangle]
unsafe fn do_move(action: *const u8, _size: usize, pos: *mut u8) { 
    let xy = 0 + std::slice::from_raw_parts_mut(pos, 25)[0];
    let mut x = std::slice::from_raw_parts_mut(pos, 4);
    *generic::data_mut_unsafe::<u32>(&mut x) += 2 * *action as u32;
    std::slice::from_raw_parts_mut(pos, 27 - 1)[8..Move::Shroom as usize - 1]
        .fill(xy);
    std::slice::from_raw_parts_mut(pos, Move::Mario as usize + 2)[Move::Mario as usize - 1 + 0] = 3;
    std::slice::from_raw_parts_mut(pos, Move::Mario as usize + 2)[Move::Mario as usize - 1 + 1] = 0;
    print!("m") 
}
