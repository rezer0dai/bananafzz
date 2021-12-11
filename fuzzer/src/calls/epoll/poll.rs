extern crate core;

use self::core::exec::call::Call;
use self::core::exec::fd_info::CallInfo;
use self::core::generator::arg::Arg;
use self::core::generator::composite::ArgComposite;

use common::table::CallIds;

extern crate api;
use self::api::leafs::bounded_leaf::Bounded;
use self::api::leafs::array_comp::*;

use args::epoll::pollfd::*;

type TPoll = unsafe extern "system" fn(
    fds: &mut u8,
    nfds: u64,
    timeout: u64,
    ) -> i32;

lazy_static! {
    static ref POLL: TPoll = unsafe{ std::mem::transmute::<_, TPoll>(generic::load_api("./uow", "poll_")) };
}

pub trait PollExec {
	fn poll() -> Call;
}
impl PollExec for Call {
	fn poll() -> Call {
		Call::new(
			CallIds::poll.into(),
			"poll",
			vec![
				Arg::memory_arg(
					Box::new(ArgComposite::array_leaf("fds_arr_comp", 3, || { Box::new(ArgComposite::pollfd()) }))),
				Arg::primitive_arg(
					Box::new(Bounded::one(0u64..=3u64))),
				Arg::primitive_arg(
					Box::new(Bounded::one(0u64..=500u64))),
			], |args| {
        
        let status = if let [fds, nfds, timeout] = &mut args[..] {
          unsafe { POLL(
            fds.data_mut_unsafe::<u8>(),
            nfds.data_const_unsafe::<u64>().clone(),
            timeout.data_const_unsafe::<u64>().clone()) }
        } else { -1 };

        CallInfo::new(-1 != status, unsafe { generic::any_as_u8_slice(&status) })
      })
	}
}
