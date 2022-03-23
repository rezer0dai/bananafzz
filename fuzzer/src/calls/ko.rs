extern crate core;
use self::core::exec::call::Call;
use self::core::exec::fd_info::CallInfo;
use self::core::generator::arg::Arg;
use self::core::generator::leaf::IArgLeaf;
use self::core::generator::serialize::ISerializableArg;
use self::core::banana::bananaq::{self, FuzzyQ};

use common::table::CallIds;

struct KOLeaf { }

use std::sync::Weak;

impl ISerializableArg for KOLeaf { }

impl IArgLeaf for KOLeaf {
    fn size(&self) -> usize { 0 }
    fn name(&self) -> &'static str { "K.O. Leaf" }

    fn generate_unsafe(&mut self, bananaq: &Weak<FuzzyQ>, _mem: &mut[u8], _fd: &[u8], _shared: &[u8]) {
        bananaq::stop(bananaq).unwrap()
    }
}

pub trait GameOver {
	fn game_over() -> Call;
}
impl GameOver for Call {
	fn game_over() -> Call {
		Call::new(
			CallIds::game_over.into(),
			"game_over",
			vec![
				Arg::primitive_arg(
					Box::new( KOLeaf { } )),
			],
			|_args| { 
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
