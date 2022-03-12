#[macro_use]
extern crate serde_derive;
extern crate serde;

use std::{
    rc::Rc,
    sync::RwLock,
};

extern crate generic;

extern crate core;

use core::exec::call::Call;
use core::banana::observer::{ICallObserver, IStateObserver};
use core::state::state::StateInfo;

pub use smb::SuperMarioBros2Config
use smb::SuperMarioBros2

struct Smb2Proxy {
    smb2: Rc<RwLock<SuperMarioBros2>>,
}
impl Smb2Proxy {
    fn new(smb2: Rc<RwLock<SuperMarioBros2>>) -> Smb2Proxy {
        Smb2Proxy {
            smb2: smb2,
        }
    }
}
impl ICallObserver for Smb2Proxy {
    fn notify(&self, state: &StateInfo, call: &mut Call) -> bool {
        match self.smb2.write() {
            Ok(mut bfl) => bfl.notify_locked(state, call),
            Err(_) => panic!("[SMB2] lock failed - CALLS")
        }
    }
    fn aftermath(&self, state: &StateInfo, call: &mut Call) {
        match self.smb2.write() {
            Ok(mut bfl) => bfl.aftermath_locked(state, call),
            Err(_) => panic!("[SMB2] lock failed - CALLS")
        }
    }
}

pub fn observers(
    cfg: &Option<SuperMarioBros2Config>,
) -> (
    Option<Box<dyn IStateObserver>>,
    Option<Box<dyn ICallObserver>>,
) {
    match *cfg {
        Some(ref cfg) => {
            let smb2 = Rc::new(RwLock::new(SuperMarioBros2::new(cfg)));
            (
                Some(Box::new(Smb2Proxy::new(Rc::clone(&smb2)))),
                None,
            )
        }
        _ => (None, None),
    }
}
