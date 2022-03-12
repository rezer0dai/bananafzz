use std;

extern crate core;
use self::core::state::id::StateTableId;
use self::core::exec::id::CallTableId;

#[allow(dead_code)]
#[repr(u64)]
pub enum StateIds {
    FdGeneric = 0x1FF0,
    FdMario = 0x100,
    FdShroom = 0x200,
    FdEnemy = 0x400,
    FdCoins = 0x800,
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
    dup,

    close = 2,

    mario = 0x10,
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
