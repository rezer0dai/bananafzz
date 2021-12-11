extern crate core;

use self::core::exec::call::Call;
use self::core::exec::fd_info::CallInfo;
use self::core::generator::arg::Arg;

use common::table::*;

extern crate api;
use self::api::leafs::bounded_leaf::Bounded;
use self::api::leafs::pattern_leaf::Pattern;
use self::api::leafs::deref_leaf::*;

use args::wsl::*;

type TRecv = unsafe extern "system" fn(
    sock: i32,
    buf: &mut u8,
    len: usize,
    flags: u64,
    ) -> i32;

lazy_static! {
    static ref RECV: TRecv = unsafe{ std::mem::transmute::<_, TRecv>(generic::load_api("./uow", "recv_")) };
}

pub trait RecvExec {
	fn recv() -> Call;
}
impl RecvExec for Call {
	fn recv() -> Call {
		Call::new(
			CallIds::recv.into(),
			"recv",
			vec![
				Arg::primitive_arg(
					Box::new(DeRef::new(SOCKET_FD_SIZE))),
				Arg::memory_arg(
					Box::new(Pattern::new(0, 0x400))),
				Arg::primitive_arg(
					Box::new(Bounded::one(0u64..=0x400u64))),
				Arg::primitive_arg(
					Box::new(Bounded::ranges(vec![
						(MSG_CMSG_CLOEXEC) as u64..=(MSG_CMSG_CLOEXEC) as u64,
						(MSG_DONTWAIT) as u64..=(MSG_DONTWAIT) as u64,
						(MSG_ERRQUEUE) as u64..=(MSG_ERRQUEUE) as u64,
						(MSG_OOB) as u64..=(MSG_OOB) as u64,
						(MSG_PEEK) as u64..=(MSG_PEEK) as u64,
						(MSG_TRUNC) as u64..=(MSG_TRUNC) as u64,
						(MSG_WAITALL) as u64..=(MSG_WAITALL) as u64,
						(MSG_WAITFORONE) as u64..=(MSG_WAITFORONE) as u64,
				]))),
			], |args| {
        
        let status = if let [sock, buf, len, flags] = &mut args[..] {
          unsafe { RECV(
            sock.data_const_unsafe::<i32>().clone(),
            buf.data_mut_unsafe::<u8>(),
            len.data_const_unsafe::<usize>().clone(),
            flags.data_const_unsafe::<u64>().clone()) }
        } else { -1 };

        CallInfo::new(-1 != status, unsafe { generic::any_as_u8_slice(&status) })
      })
	}
}
