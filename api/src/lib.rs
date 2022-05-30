#![feature(unboxed_closures, type_ascription)]
extern crate rand;
extern crate byteorder;
extern crate core;
extern crate generic;

pub mod leafs;

pub use leafs::state_link::StateLink;

// be carefull! if you put statelink you can deadlock!!
// you need to be sure in time you call this arg
// there is ensured approve() = true for some!
#[macro_export]
macro_rules! state_link {
    ($iarg:expr, $approve:expr) => {
        api::StateLink::new(Box::new($iarg), $approve)
    };
}

pub use leafs::shared_leaf::SharedWrite;

#[macro_export]
macro_rules! shared_write {
    ($off:expr, $iarg:expr) => {
        api::SharedWrite::partial($off, Box::new($iarg))
    };
}
