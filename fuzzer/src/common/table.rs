use std;

extern crate core;
use self::core::state::id::StateTableId;
use self::core::exec::id::CallTableId;

pub const FD_SIZE: usize = 4;

#[allow(dead_code)]
#[repr(u64)]
pub enum StateIds {
    FdGeneric = 0x1F0,
    FdMario = 0x100,
    FdEnemy = 0x10,
    FdShroom = 0x20,
    FdQBox = 0x40,
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
pub enum CallIds {
    dummy = 0,
    dup,

    close = 0x100,

    mario = 0x1000,
    move_mario,
    load_pos,
    game_over,
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
