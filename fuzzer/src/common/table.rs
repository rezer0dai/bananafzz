use std;

extern crate core;
use self::core::state::id::StateTableId;
use self::core::exec::id::CallTableId;

#[allow(dead_code)]
#[repr(u64)]
pub enum StateIds {
    FdMario = 0x100,
    FdShroom = 0x200,
    FdEnemy = 0x400,

    FdCoins = 0x10000,
    FdQBoxs = 0x11000,
    FdBrick = 0x12000,
}

impl From<StateTableId> for StateIds {
    fn from(id: StateTableId) -> StateIds {
        match id {
            StateTableId::Id(id) => unsafe{ std::mem::transmute(id) },
        }
    }
}
impl Into<StateTableId> for StateIds {
    fn into(self) -> StateTableId {
        StateTableId::Id(self as u64)
    }
}

/// WARNING : dont change order, just append!
///
/// - later you can regret it, as one month before you fuzz with different ids so now you can not
/// apply, gathered knowledge from before - genes / code-cov info, to you current fuzzing
#[allow(non_camel_case_types, dead_code)]
#[repr(u64)]
#[derive(PartialEq)]
pub enum CallIds {
    dummy = 0,

    non_defaults = CallTableId::non_default_start(),

    move_mario,
    load_pos,
    game_over,

    eval_pos,
    is_active,
}
impl From<CallTableId> for CallIds {
    fn from(id: CallTableId) -> CallIds {
        match id {
            CallTableId::Id(id) => unsafe{ std::mem::transmute(id) },
        }
    }
}
impl Into<CallTableId> for CallIds {
    fn into(self) -> CallTableId {
        CallTableId::Id(self as u64)
    }
}
