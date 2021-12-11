extern crate core;

use self::core::exec::call::Call;
use self::core::exec::fd_info::CallInfo;
use self::core::generator::arg::Arg;

use common::table::*;

extern crate api;
use self::api::leafs::bounded_leaf::Bounded;
use self::api::leafs::deref_leaf::*;

type TListen = unsafe extern "system" fn(
    sock: i32,
    backlog: u64,
    ) -> i32;

lazy_static! {
    static ref LISTEN: TListen = unsafe{ std::mem::transmute::<_, TListen>(generic::load_api("./uow", "listen_")) };
}

pub trait ListenExec {
	fn listen() -> Call;
}
impl ListenExec for Call {
	fn listen() -> Call {
		Call::new(
			CallIds::listen.into(),
			"listen",
			vec![
				Arg::primitive_arg(
					Box::new(DeRef::new(SOCKET_FD_SIZE))),
				Arg::primitive_arg(
					Box::new(Bounded::one(0u64..=7u64))),
			], |args| {
        
        let status = if let [sock, backlog] = &mut args[..] {
          unsafe { LISTEN(
            sock.data_const_unsafe::<i32>().clone(),
            backlog.data_const_unsafe::<u64>().clone()) }
        } else { -1 };

        CallInfo::new(-1 != status, unsafe { generic::any_as_u8_slice(&status) })
      })
	}
}
