use std;

extern crate core;
use self::core::exec::call::Call;
use self::core::exec::fd_info::CallInfo;

use common::table::CallIds;

extern crate api;

pub trait GameOver {
	fn game_over() -> Call;
}
impl GameOver for Call {
	fn game_over() -> Call {
		Call::new(
			CallIds::game_over.into(),
			"game_over",
			vec![
			],
			|_| { 
//                println!("MARIO limit reached!");
                CallInfo::fail(0)
            })
        
	}
}
