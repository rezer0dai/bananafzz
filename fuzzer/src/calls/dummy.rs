extern crate core;
use self::core::exec::call::Call;
use self::core::exec::fd_info::CallInfo;

use common::table::CallIds;

extern crate api;

pub trait DummyExec {
	fn succ() -> Call;
	fn fail() -> Call;
}
impl DummyExec for Call {
	fn succ() -> Call {
		Call::new(
			CallIds::dummy.into(),
			"dummy",
			vec![
			],
			|_| { CallInfo::succ(0) })
	}
	fn fail() -> Call {
		Call::new(
			CallIds::dummy.into(),
			"dummy",
			vec![
			],
			|_| { CallInfo::fail(0) })
	}
}
