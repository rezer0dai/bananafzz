extern crate core;
use self::core::state::id::StateTableId;

use super::super::common::table::*;

#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum Move {
    Mid = 0,
    Brick = 1,
    AbsX = 1 + 1,
    AbsY = 4 + 1 + 1,
    Cash = 4 + 4 + 1 + 1,
    Power = 1 + 4 + 4 + 1 + 1,
    Mario = 12,//where mario should GO relative x, y; u8
    Coin = 2 + 12,
    Shroom = 2 + 2 + 12,//X + Y coord as relative, u8
    Enemy = 2 + 2 + 2 + 12,//up to 5 enemies, X+Y cord of u8
    Max = 5 * 2 + 2 + 2 + 2 + 12,
}

pub const POS_START: usize = Move::AbsX as usize;
pub const POS_END: usize = Move::Max as usize;
pub const FD_SIZE: usize = std::mem::size_of::<u32>() + std::mem::size_of::<u32>();

use std::backtrace::Backtrace;

impl From<u8> for Move {
    fn from(id: u8) -> Move {
        match id {
            1 => Move::Brick,
            12 => Move::Mario,
            14 => Move::Coin,
            16 => Move::Shroom,
            18..=28 => Move::Enemy,
            id => loop { println!("no move with {id} value ==> {}", Backtrace::force_capture()) }
        }
    }
}

impl Into<StateTableId> for Move {
    fn into(self) -> StateTableId {
        match self {
            Move::Mario => StateIds::FdMario.into(),
            Move::Shroom => StateIds::FdShroom.into(),
            Move::Enemy => StateIds::FdEnemy.into(),
            Move::Coin => StateIds::FdCoins.into(),
            Move::Brick => StateIds::FdBrick.into(),
            _ => panic!("..."),
        }
    }
}
