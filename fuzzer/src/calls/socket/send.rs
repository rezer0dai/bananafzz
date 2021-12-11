extern crate core;

use self::core::exec::call::Call;
use self::core::exec::fd_info::CallInfo;
use self::core::generator::arg::Arg;
use self::core::config::FZZCONFIG;

use common::table::*;

extern crate api;
use self::api::leafs::bounded_leaf::Bounded;
use self::api::leafs::random_leaf::RndData;
use self::api::leafs::pattern_leaf::Pattern;
use self::api::leafs::deref_leaf::*;

use args::wsl::*;

type TSend = unsafe extern "system" fn(
    sock: i32,
    buf: &u8,
    len: usize,
    flags: u64,
    ) -> i32;

lazy_static! {
    static ref SEND: TSend = unsafe{ std::mem::transmute::<_, TSend>(generic::load_api("./uow", "send_")) };
}

pub trait SendExec {
	fn send() -> Call;
}
impl SendExec for Call {
	fn send() -> Call {
		Call::new(
			CallIds::send.into(),
			"send",
			vec![
				Arg::primitive_arg(
					Box::new(DeRef::new(SOCKET_FD_SIZE))),
				Arg::memory_arg(
          if !FZZCONFIG.rnd_data_to_pattern { Box::new(RndData::new(0x400)) }
          else { Box::new(Pattern::new(0x66, 0x400)) } ),
				Arg::primitive_arg(
					Box::new(Bounded::one(0u64..=0x400u64))),
				Arg::primitive_arg(
					Box::new(Bounded::ranges(vec![
						(MSG_CONFIRM) as u64..=(MSG_CONFIRM) as u64,
						(MSG_DONTROUTE) as u64..=(MSG_DONTROUTE) as u64,
						(MSG_DONTWAIT) as u64..=(MSG_DONTWAIT) as u64,
						(MSG_EOR) as u64..=(MSG_EOR) as u64,
						(MSG_MORE) as u64..=(MSG_MORE) as u64,
						(MSG_NOSIGNAL) as u64..=(MSG_NOSIGNAL) as u64,
						(MSG_OOB) as u64..=(MSG_OOB) as u64,
				]))),
			], |args| {
        
        let status = if let [sock, buf, len, flags] = &mut args[..] {
          unsafe { SEND(
            sock.data_const_unsafe::<i32>().clone(),
            buf.data_const_unsafe::<u8>(),
            len.data_const_unsafe::<usize>().clone(),
            flags.data_const_unsafe::<u64>().clone()) }
        } else { -1 };

        CallInfo::new(-1 != status, unsafe { generic::any_as_u8_slice(&status) })
      })
	}
}
