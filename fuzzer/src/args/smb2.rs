extern crate core;
use self::core::state::id::StateTableId;

use super::super::common::table::*;

#[derive(Debug, PartialEq, Clone)]
#[repr(u8)]
pub enum Move {
    Mid = 0,
    AbsX = 1,
    AbsY = 4 + 1,
    Cash = 4 + 4 + 1,
    Power = 1 + 4 + 4 + 1,
    Coin = 1 + 1 + 4 + 4 + 1,
    Mario = 2 + 11,//where mario should GO relative x, y; u8
    Shroom = 2 + 2 + 11,//X + Y coord as relative, u8
    Enemy = 2 + 2 + 2 + 11,//up to 5 enemies, X+Y cord of u8
    Max = 5 * 2 + 2 + 2 + 2 + 11,
}

pub const POS_START: usize = Move::AbsX as usize;
pub const POS_END: usize = Move::Max as usize;
pub const FD_SIZE: usize = std::mem::size_of::<u32>() + std::mem::size_of::<u32>();

impl From<u8> for Move {
    fn from(id: u8) -> Move {
        match id {
            11 => Move::Coin,
            13 => Move::Mario,
            15 => Move::Shroom,
            17..=27 => Move::Enemy,
            _ => panic!("no move with {id} value")
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
            _ => panic!("..."),
        }
    }
}
