extern crate core;

use self::core::exec::call::Call;
use self::core::exec::fd_info::CallInfo;
use self::core::generator::arg::Arg;

extern crate api;
use self::api::leafs::deref_leaf::DeRef;
use self::api::leafs::shared_leaf::Shared;

use common::table::*;

use args::smb2::*;

extern "C" { fn load_pos(pos: *mut u8); }
//fn load_pos(fd: i32, pos: *mut u8) { print!("l"); }

pub trait LoadPos {
	fn load_pos() -> Call;
	fn eval_pos() -> Call;
	fn is_active() -> Call;
}
//we dont need to provide kin to BFL, as we want to splice at move mario
//splicing at loads does not make any benefit
impl LoadPos for Call {
	fn load_pos() -> Call {
		Call::new(
			CallIds::load_pos.into(),
			"load_pos",
			vec![
				Arg::memory_arg(//Position:
                    Box::new(Shared::new(POS_START, POS_END - POS_START))),//getting state info
				Arg::primitive_arg(//Move ID:
                    Box::new(Shared::new(0, 1))),//morio strategy identifier
			], |args| {
                if let [pos, mid] = &mut args[..] {
                    let mid = *mid.data_const_unsafe::<u8>() as usize;
                    unsafe { load_pos(
                        pos.data_mut_unsafe(),
                        ) };

                    let (rel_x, rel_y) = *generic::data_const_unsafe::<(i8, i8)>(
                        &pos.data()[mid - POS_START..]);

                    if 0 == rel_x && 0 == rel_y {
                        return CallInfo::fail(0)
                    }//seems no target

                    //set rel-Y
                    pos.data_mut()[mid - POS_START + 1] = if rel_y >= 0 {
                        if rel_y > 0x18 { 
                            rel_x.abs() as u8//jump all the way
                        } else { 1 }//fast jump is OK
                    } else { 0 };

//let (pos_x, pos_y) = *pos.data_const_unsafe::<(u32, u32)>();
//println!("LOADed : <{:?}#{:?}> + <{:?}#{:?}>", pos_x, pos_y, rel_x, rel_y);

                    if rel_x.abs() < 90 && rel_y.abs() < 55 {
{
let midx: Move = (mid as u8).into();
if Move::Mario != midx 
{ println!("===> {midx:?} x {:?} -> {:?}", mid as u8, (rel_x, rel_y)); }
}
                        CallInfo::succ(0)
                    } else {
                        CallInfo::fail(0)
                    }
                } else {
                    panic!("NOT ENOUGH PARAMS TO LOAD FROM : LoadPos")
                }
      })
	}
	fn is_active() -> Call {
		Call::new(
			CallIds::is_active.into(),
			"is_active",
			vec![
				Arg::memory_arg(
					Box::new(DeRef::new(FD_SIZE))),
				Arg::memory_arg(
                    Box::new(Shared::new(POS_START, POS_END - POS_START))),//getting state info
				Arg::memory_arg(
                    Box::new(Shared::new(Move::Coin as usize, 2 * std::mem::size_of::<i8>()))),//where to save relative coin coordinates
			], |args| {
                if let [fd, pos, coin] = &mut args[..] {
                    unsafe { load_pos(
                        pos.data_mut_unsafe(),
                        ) };

                    let (tgt_x, tgt_y) = *fd.data_const_unsafe::<(u32, u32)>();
                    let (pos_x, pos_y) = *pos.data_const_unsafe::<(u32, u32)>();
                    let rel_x = (tgt_x - pos_x) as i32;
                    let rel_y = (tgt_y - pos_y) as i8;

                    if 0 == rel_x && 0 == rel_y {
                        return CallInfo::fail(0)
                    }//seems no target

                    if rel_x > 255 || rel_x < -55 {//out of screen approximation
                        return CallInfo::fail(0)
                    }

                    *coin.data_mut_unsafe::<(i8, i8)>() = (rel_x as i8, rel_y);
                    CallInfo::succ(0)
                } else {
                    panic!("NOT ENOUGH PARAMS TO LOAD FROM : LoadPos")
                }
      })
	}
	fn eval_pos() -> Call {
		Call::new(
			CallIds::eval_pos.into(),
			"eval_pos",
			vec![
				Arg::memory_arg(
                    Box::new(Shared::new(Move::Coin as usize, 2 * std::mem::size_of::<i8>()))),//where to save relative coin coordinates
			], |args| {
                if let [coin] = &mut args[..] {

                    let (rel_x, rel_y) = *coin.data_mut_unsafe::<(i8, i8)>();
                    if rel_x.abs() > 90 || rel_y.abs() > 55 {
                        return CallInfo::fail(0)
                    }
                    CallInfo::succ(0)
                } else {
                    panic!("NOT ENOUGH PARAMS TO LOAD FROM : LoadPos")
                }
      })
	}
}
