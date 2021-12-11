extern crate core;

use self::core::exec::call::Call;
use self::core::exec::fd_info::CallInfo;
use self::core::generator::arg::Arg;

use common::table::*;

extern crate api;
use self::api::leafs::bounded_leaf::Bounded;
use self::api::leafs::deref_leaf::*;

use args::wsl::*;

type TShutDown = unsafe extern "system" fn(
    sock: i32,
    how: u64,
    ) -> i32;

lazy_static! {
    static ref SHUTDOWN: TShutDown = unsafe{ std::mem::transmute::<_, TShutDown>(generic::load_api("./uow", "shutdown_")) };
}

pub trait ShutDownExec {
	fn shutdown() -> Call;
}
impl ShutDownExec for Call {
	fn shutdown() -> Call {
		Call::new(
			CallIds::shutdown.into(),
			"shutdown",
			vec![
				Arg::primitive_arg(
					Box::new(DeRef::new(SOCKET_FD_SIZE))),
				Arg::primitive_arg(
					Box::new(Bounded::ranges(vec![
						(SHUT_RD) as u64..=(SHUT_RD) as u64,
						(SHUT_WR) as u64..=(SHUT_WR) as u64,
				]))),
			], |args| {
        
        let status = if let [sock, how] = &mut args[..] {
          unsafe { SHUTDOWN(
            sock.data_const_unsafe::<i32>().clone(),
            how.data_const_unsafe::<u64>().clone()) }
        } else { -1 };

        CallInfo::new(-1 != status, unsafe { generic::any_as_u8_slice(&status) })
      })
	}
}
