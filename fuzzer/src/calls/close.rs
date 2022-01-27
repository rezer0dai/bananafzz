extern crate core;

use self::core::exec::call::Call;
use self::core::exec::fd_info::CallInfo;
use self::core::generator::arg::Arg;

use common::table::*;

extern crate api;
use self::api::leafs::deref_leaf::*;

type TClose = unsafe extern "system" fn(
    fd: i32,
    );
lazy_static! {
    static ref CLOSE: TClose = unsafe{ std::mem::transmute::<_, TClose>(generic::load_api("./uow", "close_")) };
}

pub trait CloseExec {
	fn close() -> Call;
}
impl CloseExec for Call {
	fn close() -> Call {
		Call::new(
			CallIds::close.into(),
			"close",
			vec![
				Arg::primitive_arg(
					Box::new(DeRef::new(FD_SIZE))),
			], |args| {
        
        if let [sock] = &mut args[..] {
          unsafe { CLOSE(*sock.data_const_unsafe()) }
        }

        CallInfo::succ(0)
      })
	}
}
