extern crate core;

use self::core::exec::call::Call;
use self::core::exec::fd_info::CallInfo;
use self::core::generator::arg::Arg;
use self::core::generator::composite::ArgComposite;

use common::table::*;

extern crate api;
use self::api::leafs::deref_leaf::*;

use args::socket::sockaddr::*;

type TConnect = unsafe extern "system" fn(
    sock: i32,
    addr: &mut u8,
    addrlen: usize,
    ) -> i32;

lazy_static! {
    static ref CONNECT: TConnect = unsafe{ std::mem::transmute::<_, TConnect>(generic::load_api("./uow", "connect_")) };
}

pub trait ConnectExec {
	fn connect() -> Call;
}
impl ConnectExec for Call {
	fn connect() -> Call {
		Call::new(
			CallIds::connect.into(),
			"connect",
			vec![
				Arg::primitive_arg(
					Box::new(DeRef::new(SOCKET_FD_SIZE))),
				Arg::memory_arg(
					Box::new(ArgComposite::sockaddr())),
			], |args| {
        
        let status = if let [sock, addr] = &mut args[..] {
          let addrlen = addr.data().len();
          unsafe { CONNECT(
            sock.data_const_unsafe::<i32>().clone(),
            addr.data_mut_unsafe::<u8>(),
            addrlen) }
        } else { -1 };

        CallInfo::new(-1 != status, unsafe { generic::any_as_u8_slice(&status) })
      })
	}
}
