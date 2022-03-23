use std::sync::Weak;

extern crate core;
use self::core::banana::bananaq::FuzzyQ;
pub use self::core::generator::composite::ArgComposite;
use self::core::generator::leaf::IArgLeaf;
use self::core::generator::serialize::ISerializableArg;

extern crate api;
use self::api::leafs::bounded_leaf::Bounded;
use self::api::leafs::flag_leaf::Flag;
use self::api::leafs::bfl_leaf::Bfl;

pub use super::smb2::*;

extern crate generic;

struct MoveMarioLeaf { }

impl ISerializableArg for MoveMarioLeaf { }

impl IArgLeaf for MoveMarioLeaf {
    fn size(&self) -> usize { panic!("[bananafzz] unsized arg") }
    fn name(&self) -> &'static str { panic!("[bananafzz] unnamed arg") }

    fn generate_unsafe(&mut self, _: &Weak<FuzzyQ>, mem: &mut[u8], _fd: &[u8], shared: &[u8]) {
        let mid = *generic::data_const_unsafe::<u8>(shared);
        let (rel_x, rel_y) = *generic::data_const_unsafe::<(i8, u8)>(
            &shared[mid as usize..]);

        static SPEED_MAP: [u8; 0x10] = [1, 2, 3, 4, 4, 5, 5, 6, 6, 6, 7, 7, 7, 7, 8, 8, ];
        let move_x = if (rel_x.abs() as usize) < SPEED_MAP.len() {
            SPEED_MAP[rel_x.abs() as usize]
        } else { mem[0] };

        if rel_x > 0 {
            mem[0] = move_x;
        }
        mem[4] += rel_y;

        if rel_x < 0 && rel_x > -8 {
            mem[0] = 0;
            mem[1] = move_x//going to left instead
        }

        if Move::Enemy == mid.into() {
            if 1 < mem[5] {
                mem[5] = mem[0]//fire all the way!!
            } 
        }
//        let mid: Move = mid.into();
//println!("[SMB] move : {:?} => {:?} : WORLD : {:?} :: of : {:?}", mem, rel_x, generic::data_const_unsafe::<(u32, u32)>(&shared[Move::AbsX as usize..]), mid);
    }
}

pub trait MarioComposite {
    fn move_mario() -> ArgComposite;
}

impl MarioComposite for ArgComposite {
    fn move_mario() -> ArgComposite {
        ArgComposite::new_w_logic(
            7, 
            "move-mario-keys", 
            vec![
                (0, Box::new(Bounded::one(3u8..=20))),//right
                (1, Box::new(Bounded::one(0u8..=4))),//left
                (2, Box::new(Bfl::new(Bounded::one(0u8..=3)))),//climb
                (3, Box::new(Bfl::new(Bounded::one(0u8..=3)))),//slide
                (4, Box::new(Bounded::one(0u8..=10))),//jump
                (5, Box::new(Bfl::new(Bounded::one(0u8..=3)))),//fire
//                    (5, Box::new(Flag::new(0, 1u8))),//fire
//                    (6, Box::new(Bounded::one(0u8..=10))),//start
                (6, Box::new(Bfl::new(Flag::new(0, 1u8)))),//start
            ],
            Box::new( MoveMarioLeaf{} )
        )
    }
}
