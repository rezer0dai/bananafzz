extern crate core;

use self::core::exec::call::Call;
use self::core::exec::fd_info::CallInfo;
use self::core::generator::arg::Arg;

extern crate api;
use self::api::leafs::deref_leaf::*;
use self::api::leafs::shared_leaf::Shared;

use common::table::*;

use args::smb2::{MoveArg, POS_SIZE};

type TDoMove = unsafe extern "system" fn(
    fd: u32,
    action: &u8,
    ) -> u8;
lazy_static! {
    static ref DO_MOVE: TDoMove = unsafe{ std::mem::transmute::<_, TDoMove>(generic::load_api("./smbc", "do_move")) };
}

pub trait MoveMario {
	fn move_mario() -> Call;
}
impl MoveMario for Call {
	fn move_mario() -> Call {
		Call::new(
			CallIds::move_mario.into(),
			"move_mario",
			vec![
				Arg::primitive_arg(
					Box::new(DeRef::new(FD_SIZE))),
				Arg::memory_arg(
					Box::new(MoveArg::new())),
                Arg::memory_arg(
                    Box::new(Shared::new(0, POS_SIZE))//just for reading after call to kin
                ),
			], |args| {
        
                if let [fd, action, pos] = &mut args[..] {
                    unsafe { DO_MOVE(
                        *fd.data_const_unsafe(),
                        action.data_const_unsafe()); }
                    let pos = *generic::data_const_unsafe::<u32>(&pos.data());
                    return CallInfo::succ(pos as usize)//kin is X coordinate, for crossovers
                }
                CallInfo::fail(0)
      })
	}
}
