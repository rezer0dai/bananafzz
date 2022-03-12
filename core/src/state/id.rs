use std::cmp::Ordering;
use std::ops::{
    BitAnd,
    BitOr,
};

#[derive(Eq, Copy, Clone, Debug, Deserialize, Serialize)]
pub enum StateTableId {
    Id(u64),
}

impl PartialOrd for StateTableId {
    fn partial_cmp(&self, other: &StateTableId) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for StateTableId {
    fn cmp(&self, other: &StateTableId) -> Ordering {
        match *self {
            StateTableId::Id(lhs) => match *other {
                StateTableId::Id(rhs) => lhs.cmp(&rhs),
            }
        }
    }
}
impl PartialEq for StateTableId {
    fn eq(&self, other: &StateTableId) -> bool {
        match *self {
            StateTableId::Id(lhs) => match *other {
                StateTableId::Id(rhs) => lhs == rhs,
            }
        }
    }
}

impl BitOr for StateTableId {
    type Output = Self;

    fn bitor(self, other: StateTableId) -> Self {
        match self {
            StateTableId::Id(lhs) => match other {
                StateTableId::Id(rhs) => StateTableId::Id(lhs | rhs),
            }
        }
    }
}
impl BitAnd for StateTableId {
    type Output = bool;

    fn bitand(self, other: StateTableId) -> bool {
        match self {
            StateTableId::Id(lhs) => match other {
                StateTableId::Id(rhs) => 0 != (lhs & rhs),
            }
        }
    }
}

use std;

impl From<StateTableId> for u64 {
    fn from(id: StateTableId) -> u64 {
        match id {
            StateTableId::Id(id) => unsafe{ std::mem::transmute(id) },
        }
    }
}
