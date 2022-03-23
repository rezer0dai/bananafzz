extern crate core;
use self::core::exec::call::Call;
use self::core::exec::fd_info::CallInfo;

use common::table::CallIds;

extern crate api;

pub trait DummyExec {
	fn ok_ctor() -> Call;
	fn succ() -> Call;
	fn fail() -> Call;
}
impl DummyExec for Call {
	fn ok_ctor() -> Call {
		Call::new(
			CallIds::ok_ctor.into(),
			"yes-ctor",
			vec![
			],
			|_| { CallInfo::succ(0) })
	}
	fn succ() -> Call {
		Call::new(
			CallIds::dummy.into(),
			"yes-call",
			vec![
			],
			|_| { CallInfo::succ(0) })
	}
	fn fail() -> Call {
		Call::new(
			CallIds::dummy.into(),
			"no-call",
			vec![
			],
			|_| { CallInfo::fail(0) })
	}
}
