extern crate core;

use self::core::exec::call::Call;
use self::core::exec::fd_info::CallInfo;
use self::core::generator::arg::Arg;
use self::core::generator::composite::ArgComposite;

use common::table::*;
use args::wsl::*;

use args::socket::sockaddr::*;

extern crate api;
use self::api::leafs::deref_leaf::*;

type TAccept = unsafe extern "system" fn(
    sock: i32,
    addr: &mut u8,
    addrlen: &mut usize,
    ) -> i32;

lazy_static! {
    static ref ACCEPT: TAccept = unsafe{ std::mem::transmute::<_, TAccept>(generic::load_api("./uow", "accept_")) };
}

pub trait AcceptExec {
	fn accept(ctor: bool) -> Call;
}
impl AcceptExec for Call {
	fn accept(ctor: bool) -> Call {
		Call::new(
			CallIds::accept.into(),
			"accept",
			vec![
        if ctor {
          Arg::primitive_arg( // this call is also ctor, therefore we pull random socket from queue
            rnd_fd(StateIds::FdSocket.into()))
        } else {
          Arg::primitive_arg( // this call is also ctor, therefore we pull random socket from queue
            Box::new(DeRef::new(SOCKET_FD_SIZE)))
        },
				Arg::memory_arg(
					Box::new(ArgComposite::sockaddr())),
			], |args| {
        
        let status = if let [sock, addr] = &mut args[..] {
          let mut addrlen = addr.data().len();
          unsafe { ACCEPT(
            sock.data_const_unsafe::<i32>().clone(),
            addr.data_mut_unsafe::<u8>(),
            &mut addrlen) }
        } else { -1 };

        CallInfo::new(-1 != status, unsafe { generic::any_as_u8_slice(&status) })
      })
	}
}
