extern crate core;

use self::core::exec::call::Call;
use self::core::exec::fd_info::*;
use self::core::generator::arg::Arg;

use common::table::*;

extern crate api;
use self::api::leafs::fd_leaf::FdHolder;

type TDup = unsafe extern "system" fn(
    fd: i32,
    ) -> i32;
lazy_static! {
    static ref DUP: TDup = unsafe{ std::mem::transmute::<_, TDup>(generic::load_api("./uow", "dup_")) };
}
fn dup_impl(fd: &[u8]) -> [u8; FD_SIZE] {
    let fdi = unsafe { DUP(*generic::data_const_unsafe::<i32>(&fd)) };
    let mut fd = [0u8; FD_SIZE];
    fd.clone_from_slice(unsafe { generic::any_as_u8_slice(&fdi) } );
    fd
}

pub trait DupExec {
	fn dup(fd: &[u8]) -> Call;
}
impl DupExec for Call {
	fn dup(fd: &[u8]) -> Call {
		Call::new(
			CallIds::dup.into(),
			"dup",
			vec![
				Arg::primitive_arg(
					Box::new(FdHolder::dup(fd))),
			], |args| {
        
        let fd = if let [fd] = &mut args[..] {
          dup_impl(fd.data())
        } else {
          [0xffu8; FD_SIZE]
        };

        CallInfo::infofromfd(Fd::new(&fd), 0)
      })
	}
}
