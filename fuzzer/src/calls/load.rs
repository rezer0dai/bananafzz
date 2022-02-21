extern crate core;

use self::core::exec::call::Call;
use self::core::exec::fd_info::CallInfo;
use self::core::generator::arg::Arg;

extern crate api;
use self::api::leafs::deref_leaf::DeRef;
use self::api::leafs::shared_leaf::Shared;
//use self::api::leafs::bounded_leaf::Bounded;
//use self::api::leafs::phantom_leaf::Phantom;

//use self::core::generator::composite::ArgComposite;
//use self::api::leafs::tuple_leaf::TupleComposite;

use common::table::*;

use args::smb2::POS_SIZE;

extern "C" { fn load_pos(fd: i32, pos: *mut u8); }
//fn load_pos(fd: i32, pos: *mut u8) { print!("l"); }

pub trait LoadPos {
	fn load_pos() -> Call;
}
impl LoadPos for Call {
	fn load_pos() -> Call {
		Call::new(
			CallIds::load_pos.into(),
			"load_pos",
			vec![
				Arg::primitive_arg(
					Box::new(DeRef::new(FD_SIZE))),
				Arg::memory_arg(
                    Box::new(Shared::new(0, POS_SIZE))),
//                    Box::new(Phantom::new(POS_SIZE))),
//
//Arg::memory_arg(Box::new(ArgComposite::tuple_leaf("tuple1", Box::new(Shared::new(0, POS_SIZE)), 
//            Box::new(ArgComposite::tuple_leaf("tuple1", 
//                    Box::new(ArgComposite::tuple_leaf("tuple1", Box::new(Shared::new(0, POS_SIZE)), Box::new(Shared::new(0, POS_SIZE)))),
//                    Box::new(Shared::new(0, POS_SIZE))))))),

			], |args| {
        
                if let [fd, pos] = &mut args[..] {
//println!("ARGS:{:X} :: {:?}", *fd.data_const_unsafe::<u32>(), pos.data());
//for b in pos.data_mut().iter_mut() { *b += 1 }

                    unsafe { load_pos(
                        *fd.data_const_unsafe(),
                        pos.data_mut_unsafe(),
                        ) };
                    let pos = *generic::data_const_unsafe::<u32>(&pos.data());
                    return CallInfo::succ(pos as usize)//kin is X coordinate, for crossovers
                } else {
                    panic!("NOT ENOUGH PARAMS TO LOAD FROM : LoadPos")
                }
      })
	}
}
