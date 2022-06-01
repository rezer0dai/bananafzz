extern crate core;
use core::banana::observer::{ICallObserver, WantedMask};
use core::exec::call::Call;
use core::state::state::StateInfo;

pub trait IBananaFeedback {
    fn add_node(&mut self, node: &[u8]);
}

use std::sync::RwLockWriteGuard;

#[allow(improper_ctypes)]
extern "C" {
    fn banana_feedback<'a>() -> RwLockWriteGuard<'a, Vec<Vec<u8>>>;
}

struct Bijon {
    //    feedback: Rc<RwLock<dyn IBananaFeedback>>,
}

static mut HITC: u32 = 0;

impl Bijon {
    fn new(//        feedback: Rc<RwLock<dyn IBananaFeedback>>
    ) -> Self {
        unsafe { HITC = 0 }
        unsafe { banana_feedback().push(vec![0x42]) }
        unsafe { banana_feedback().push(vec![0x42]) }

        //ensure to have at least one input in corpus

        Self {} // Bijon { feedback }
    }

    fn feedback(&self, state: &StateInfo, call: &mut Call) -> Vec<u8> {
        let mut node = vec![];

        if 0x11u64 != call.id().into() && 0x100u64 == state.id.into() {
            return node;
        } // for mario only moves

        node.extend(u64::from(state.id).to_le_bytes().to_vec());

        if !call.ok() {
            return node;
        } // those with sucess will be added additional feedback

        if 0x100u64 == state.id.into() {
            return node;
        } // ok for mario alone it is where we stop giving more feedback

        node.extend(state.fd.data()); //normaly you dont want to do this cuz runtime .. every trun diff

        if 0x11u64 != call.id().into() {
            return node;
        } // pos we will log only for sucessfull moves -> most feedback granted

        let pos = call.args_view(1).data_const_unsafe::<u32>();

        let pos = if 0x100u64 == state.id.into() {
            pos / 25 // if mario we log only few big steps achieved
        } else {
            pos / 5
        };

        node.extend(pos.to_le_bytes());

        return node;
    }

    #[allow(dead_code)]
    fn pos_feedback(&self, state: &StateInfo, call: &mut Call) -> Vec<u8> {
        // position feadback : opt - out, just testing for feedback
        if 0x11u64 != call.id().into() {
            return vec![];
        } // pos we will log only for sucessfull moves -> most feedback granted

        let pos = call.args_view(1).data_const_unsafe::<u32>();

        if 0x100u64 == state.id.into() {
            pos / 25
        } else {
            pos / 5
        }
        .to_le_bytes()
        .to_vec()
    }

    #[allow(dead_code)]
    fn lop_feedback(&self) -> Vec<u8> {
        // length of poc feadback : opt - out, just testing for feedback
        unsafe {
            HITC += 1;
            HITC.to_le_bytes().to_vec()
        }
    }
}

impl ICallObserver for Bijon {
    fn notify(&self, _: &StateInfo, _: &mut Call) -> Result<bool, WantedMask> {
        Ok(true)
    }

    fn aftermath(&self, state: &StateInfo, call: &mut Call) {
        //println!("[B-IJON] pos_x, pos_y {:?}", call.args_view(1).data_const_unsafe::<(u32, u32)>());

        let node = self.feedback(state, call);
        //        let node = self.pos_feedback(state, call);
        //        let node = self.lop_feedback();
        if 0 != node.len() {
            unsafe { banana_feedback().push(node) }
        }
    }
}

pub fn observer(//    feedback: Rc<RwLock<dyn IBananaFeedback>>
) -> Box<dyn ICallObserver> {
    Box::new(Bijon::new()) //feedback))
}
