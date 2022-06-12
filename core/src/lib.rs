#![feature(thread_id_value, backtrace, backtrace_frames)]

extern crate log;

extern crate clock_ticks;

#[macro_use]
extern crate serde_derive;

extern crate rand;

pub mod banana;
pub mod config;
pub mod exec;
pub mod generator;
pub mod state;

extern crate generic;

#[macro_export]
macro_rules! arg {
    ($arg:expr) => {
        Arg::memory_arg(Box::new($arg))
    };
}

#[macro_export]
macro_rules! val {
    ($p:expr) => {
        *$p.data_const_unsafe()
    };
}
#[macro_export]
macro_rules! val_mut {
    ($p:expr) => {
        $p.data_mut_unsafe()
    };
}
