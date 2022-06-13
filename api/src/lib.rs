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
macro_rules! requires {
    ($iarg:expr, $approve:expr) => {
        api::StateLink::new(Box::new($iarg), $approve)
    };
}

pub use leafs::shared_leaf::SharedWrite;

#[macro_export]
macro_rules! pin2state {
    ($off:expr, $iarg:expr) => {
        api::SharedWrite::partial($off, Box::new($iarg))
    };
}

pub use leafs::transform::Transform;

#[macro_export]
macro_rules! transform {
    ($transform:expr, $iarg:expr) => {
        api::Transform::new(Box::new($iarg), $transform)
    };
}
