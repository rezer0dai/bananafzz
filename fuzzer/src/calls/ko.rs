use std;

extern crate core;
use self::core::exec::call::Call;
use self::core::exec::fd_info::CallInfo;
use self::core::generator::arg::Arg;

extern crate api;
use self::api::leafs::deref_leaf::*;

use args::movem::*;

use common::table::CallIds;

pub trait GameOver {
	fn game_over() -> Call;
}
impl GameOver for Call {
	fn game_over() -> Call {
		Call::new(
			CallIds::game_over.into(),
			"game_over",
			vec![
				Arg::memory_arg(
					Box::new(DeRef::new(FD_SIZE))),
			],
			|args| { 
                /*
                if let [fd] = &mut args[..] {
                    let mid: Move = fd.data()[0].into();
                    println!("MARIO KO reached! <{:?}>", mid);
                }
                */
                CallInfo::fail(0)
            })
        
	}
}
