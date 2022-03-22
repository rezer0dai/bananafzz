use std::cmp::Ordering;
use std::ops::{
    BitAnd,
};

/// will represent abstraction of ID per targeted {SYS/API/IN-OUT/..} call
///
/// - w32k, ntos, in/out, vmwp drivers / ... specific
/// - windows / linux / osx / ... specific
///
/// #Example
/// ```
/// #[repr(u64)]
/// pub enum CallIds {
///     Write = 1,
///     ...
/// }
/// impl From<CallTableId> for CallIds {
///     fn from(id: CallTableId) -> CallIds {
///         match id {
///             CallTableId::Id(id) => unsafe{ std::mem::transmute(id) },
///         }
///     }
/// }
/// impl Into<CallTableId> for CallIds {
///     fn into(self) -> CallTableId {
///         CallTableId::Id(self as u64)
///     }
/// }
/// ```
#[derive(Eq, Clone, Debug, Deserialize, Serialize)]
pub enum CallTableId {
    Id(u64),
}

impl CallTableId {
  // first 0x10 enums are reserved to avoid limiter to limit those, 
  // or other modules to filter them out
  pub const fn non_default_start() -> u64 {
      0x10
  }
  pub fn is_default(self) -> bool {
      for id in 0..CallTableId::non_default_start() {
        if CallTableId::Id(id) == self {
          return true;
        }
      }
      false
  }
}

impl PartialOrd for CallTableId {
    fn partial_cmp(&self, other: &CallTableId) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for CallTableId {
    fn cmp(&self, other: &CallTableId) -> Ordering {
        match *self {
            CallTableId::Id(lhs) => match *other {
                CallTableId::Id(rhs) => lhs.cmp(&rhs),
            }
        }
    }
}
impl PartialEq for CallTableId {
    fn eq(&self, other: &CallTableId) -> bool {
        match *self {
            CallTableId::Id(lhs) => match *other {
                CallTableId::Id(rhs) => lhs == rhs,
            }
        }
    }
}
impl BitAnd for CallTableId {
    type Output = bool;

    fn bitand(self, other: CallTableId) -> bool {
        match self {
            CallTableId::Id(lhs) => match other {
                CallTableId::Id(rhs) => 0 != (lhs & rhs),
            }
        }
    }
}

impl From<CallTableId> for u64 {
    fn from(id: CallTableId) -> u64 {
        match id {
            CallTableId::Id(id) => unsafe{ std::mem::transmute(id) },
        }
    }
}
