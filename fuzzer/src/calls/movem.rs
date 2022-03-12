extern crate core;

use self::core::exec::call::Call;
use self::core::exec::fd_info::CallInfo;
use self::core::generator::arg::Arg;

extern crate api;
use self::api::leafs::deref_leaf::*;
use self::api::leafs::shared_leaf::Shared;
use self::api::leafs::random_leaf::RndData;

use common::table::*;

use args::movem::*;

extern "C" { fn do_move(_action: *const u8, _size: usize, _pos: *mut u8); }
//fn do_move(_fd: i32, _action: *const u8, _size: usize, _pos: *mut u8) { print!("m"); }

pub trait MoveMario {
	fn move_mario() -> Call;
}
impl MoveMario for Call {
	fn move_mario() -> Call {
		Call::new(
			CallIds::move_mario.into(),
			"move_mario",
			vec![
				Arg::memory_arg(
					Box::new(ArgComposite::move_mario())),//RndData::new(3))),//
                Arg::memory_arg(
                    Box::new(Shared::new(POS_START, POS_END - POS_START))),//just for reading after call to kin
                Arg::primitive_arg(
                    Box::new(Shared::new(0, 1))),//mid
			], |args| {
                if let [action, pos, info] = &mut args[..] {
                    let mid = info.data_const_unsafe::<u8>().clone();

                    let (before_x, before_y) = *pos.data_const_unsafe::<(i32, u32)>();
                    unsafe { do_move(
                        action.data_const_unsafe(),
                        action.data().len(),
                        pos.data_mut_unsafe(),
                        ) }
                    let (after_x, after_y) = *pos.data_const_unsafe::<(i32, u32)>();
                    if  0 == after_x {
//println!("GAME OVE BY EMULATOR");
                        return CallInfo::fail(0)
                    }//game over

                    let (move_r, move_l) = *action.data_const_unsafe::<(i8, i8)>();
                    let move_x = move_r - move_l;
                    let target_x = before_x + move_x as i32;

                    let ok = match mid.into() {
                        Move::Mario => //we go at least so far, or closer than before
                            after_x + 3 > target_x,
//                            || (after_x - target_x).abs() < (before_x - target_x).abs(),
                        Move::Coin => //reached target ? lets only on X
                            (after_x-3..after_x+3).contains(&target_x)
                        //coins++ ?
                            || 0 != pos.data()[Move::Cash as usize - POS_START],
                        //enemy out ? 
                        Move::Enemy => 0 == pos.data()[mid as usize - POS_START],
                        //boosted ? 
                        Move::Shroom => 0 != pos.data()[Move::Power as usize - POS_START],
                        _ => panic!("[SMB2] unknown match of MID"),
                    };

//let mid: Move = mid.into();
//println!("[{:?}]move call : {:?} -> {:?} x {:?} x {:?}", mid, ok, after_x, target_x, before_x);
//return CallInfo::fail(after_x as usize);

                    return if ok { //kin is X coordinate, for crossovers
                        CallInfo::succ(after_x as usize)
                    } else { 
                        CallInfo::fail(after_x as usize) };
                }
                CallInfo::fail(0)
      })
	}
}
