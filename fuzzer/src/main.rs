#[macro_use]
extern crate lazy_static;

extern crate rand;
use rand::Rng;

extern crate core;
use core::banana::bananaq;

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
pub fn LLVMFuzzerTestOneInput_mut(data: *mut u8, size: usize) -> i32 {
//println!("PUSH");
    FuzzyState::fuzz(Box::new(MarioState::spawn())).join();
//println!("DONE");
    1
}

//we need this mut to be able toupdate poc call desriptions, mainly KIN members
pub unsafe fn LLVMFuzzerTestOneInput(poc_mem: *mut u8, data: *const u8, size: usize) -> i32 {
//    println!("TESTONE INPUT TO BANANA {:X}", size);

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

//    cfg.core.bfl = Some(bfl); //"ref" mut do the job

    let plugins = plugs::Plugins::new(cfg, &push_state);
    load_plugins(plugins.observers);

    let res = LLVMFuzzerTestOneInput_mut(std::mem::transmute(data), size);

    bananaq::detach_observers();

    while !bananaq::empty() { }

    res
}

pub fn main() {
    println!("OK RESOLVED EXTERNS TO BANANA");

    if FZZCONFIG.noisy {
        println!("{}", FZZCONFIG.version);
    }
//    panic!("OK RESOLVED EXTERNS TO BANANA");

    let mut poc = [0u8; 0x10000];
    generic::data_mut_unsafe::<PocDataHeader>(&mut poc).total_size = std::mem::size_of::<PocDataHeader>();
// create dummy pocjust header

    for i in 0..400 {
// LOAD CRASH
/*
        //f887ea2517424f2f
        if let Ok(data) = generic::read_file_raw("out/3e500a00d08eccbe") {//3e500a00d08eccbe//42ce44f0e0a28214") {
            poc[..data.len()].clone_from_slice(&data);
        } else { panic!("NOP NO CRASH FILE LOCATED") }
*/

        let cc = generic::data_const_unsafe::<PocDataHeader>(&poc).calls_count;
        generic::data_mut_unsafe::<PocDataHeader>(&mut poc).magic = 66;
        let ii = if cc > 1 { 1 } else {cc};//rand::thread_rng().gen_range(0..=cc);
        generic::data_mut_unsafe::<PocDataHeader>(&mut poc).insert_ind = ii;

        let total_size = generic::data_const_unsafe::<PocDataHeader>(&poc).total_size;
/*
        println!("NOW SIZE IS : {total_size} -> {:?}/{i}=>{:?}",
            generic::data_const_unsafe::<PocDataHeader>(&poc).insert_ind,
            generic::data_const_unsafe::<PocDataHeader>(&poc).calls_count,
            );
*/
        let data = poc[..total_size].to_vec();
        if i > 0 {
            generic::write_file_raw(format!("./bfl_in/{:X}", cc).as_str(), &data);
        }
//copy from poc to data
        unsafe {
            LLVMFuzzerTestOneInput(std::mem::transmute(poc.as_ptr()), std::mem::transmute(data.as_ptr()), data.len());
        }
        if total_size != generic::data_mut_unsafe::<PocDataHeader>(&mut poc).total_size {
            if ii != cc { println!("****************>>> INSERTED {ii}/{cc}"); }
        }
        if i > 0 {
            if let Ok(data) = generic::read_file_raw(format!("./bfl_in/{:X}", cc).as_str()) {
                poc[..data.len()].clone_from_slice(&data);
            } else { panic!("NOP NO CRASH FILE LOCATED") }
        }
        unsafe {
            LLVMFuzzerTestOneInput(std::mem::transmute(poc.as_ptr()), std::mem::transmute(data.as_ptr()), data.len());
        }
    }

unsafe { println!("***** FINAL SIZE IS : {:?} -> {:?}",
    generic::data_const_unsafe::<PocDataHeader>(&poc).total_size,
    generic::data_const_unsafe::<PocDataHeader>(&poc).calls_count,
    ) }

}
