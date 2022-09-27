use std::cmp::Ordering;
use std::ops::{
    BitAnd,
    BitOr,
};

#[derive(Eq, Copy, Clone, Debug, Deserialize, Serialize)]
pub enum StateTableId {
    Id(u64),
}

impl StateTableId {
    // unicorn will not be used in rndfd ( will be automatically
    // disabled ), if not used alone ( aka you querey
    // for anything including Unicorn, then 1 flag is deleted )
    // .. but if you want to RndFd(Unicorn) then is OK
    // .. it will sezrch you in all Keypairs
    pub fn unicorn(&self) -> Self {
        match self {
            StateTableId::Id(id) => StateTableId::Id(id | 1)
        }
    }
    pub fn racer(&self) -> Self {
        match self {
            StateTableId::Id(id) => StateTableId::Id(id | 2)
        }
    }
    pub fn core_flags(self) -> Self {
        match self {
            StateTableId::Id(id) => StateTableId::Id(id & (!0xFF))
        }
    }
    pub fn de_horn(self) -> Self {
        match self {
            StateTableId::Id(id) => 
                if 1 != id { // pure unicorn ?
                    StateTableId::Id(id & !1) // no == de-horn
                } else { self } // yes == keep horn
        }
    }
    pub fn is_unicorn(&self) -> bool {
        StateTableId::Id(1) & *self
    }
    pub fn do_match(&self, mask: &Self) -> bool {
        let id = u64::from(self.de_horn());
        id == id & u64::from(mask.de_horn())
    }
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
