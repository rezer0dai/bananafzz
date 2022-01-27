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
use plugs::Observer;

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
    FuzzyState::fuzz(Box::new(MarioState::spawn()));
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
    }
}
fn fuzzy_pool() {
    if FZZCONFIG.noisy {
        println!("{}", FZZCONFIG.version);
    }

    match plugs::plug(&push_state) {
        Ok(plugins) => load_plugins(plugins),
        Err(e) => println!("problem with loading of plugins : {:?}", e),
    };

    let (tx, rx): (Sender<bool>, Receiver<bool>) = mpsc::channel();
    let looper = thread::spawn(move || {
        thread::sleep(time::Duration::from_millis(1000 * FZZCONFIG.active_seconds));
        tx.send(true).unwrap();
    });

    while rx.try_recv().is_err() {
        push_fuzz();
        thread::sleep(time::Duration::from_millis(FZZCONFIG.push_sleep));
        break // in SMB2 one push is OK
    }

    looper.join().unwrap();

    plugs::stop_fuzzing()
}

pub fn main() {
    fuzzy_pool()
}
