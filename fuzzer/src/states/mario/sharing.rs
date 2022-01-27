extern crate core;
use self::core::state::state::IFdState;

use super::state::*;

impl IFdState for MarioState {
    fn invalid(&self) -> bool {
        self.state.fd().is_invalid()
    }
    fn is_online(&mut self) -> bool {
        true
    }
}
