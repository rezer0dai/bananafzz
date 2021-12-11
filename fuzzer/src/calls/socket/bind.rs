extern crate core;

use self::core::exec::call::Call;
use self::core::exec::fd_info::CallInfo;
use self::core::generator::arg::Arg;
use self::core::generator::composite::ArgComposite;

use common::table::*;

extern crate api;
use self::api::leafs::deref_leaf::*;

use args::socket::sockaddr::*;

type TBind = unsafe extern "system" fn(
    sock: i32,
    addr: &mut u8,
    addrlen: usize,
    ) -> i32;

lazy_static! {
    static ref BIND: TBind = unsafe{ std::mem::transmute::<_, TBind>(generic::load_api("./uow", "bind_")) };
}

#[allow(non_camel_case_types)]
pub trait BindExec {
	fn bind() -> Call;
}
impl BindExec for Call {
	fn bind() -> Call {
		Call::new(
			CallIds::bind.into(),
			"bind",
			vec![
				Arg::primitive_arg(
					Box::new(DeRef::new(SOCKET_FD_SIZE))),
				Arg::memory_arg(
					Box::new(ArgComposite::sockaddr())),
			], |args| {
        
        let status = if let [sock, addr] = &mut args[..] {
          let addrlen = addr.data().len();
          unsafe { BIND(
            sock.data_const_unsafe::<i32>().clone(),
            addr.data_mut_unsafe::<u8>(),
            addrlen) }
        } else { -1 };

        CallInfo::new(-1 != status, unsafe { generic::any_as_u8_slice(&status) })
      })
	}
}
