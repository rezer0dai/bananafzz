extern crate core;
use self::core::state::state::{
    IFuzzyObj,
    State
};

use super::state::*;

impl IFuzzyObj for SocketState {
    fn fuzzy_loop(&mut self) -> bool {
        if !self.state.do_fuzz_one() {
            return false
        }
        if !self.fuzz_one() {
            return false
        }
        if self.state.call_view().ok() {
            println!("OK : {}", self.state.call_view().name())
        }
        self.state.do_fuzz_update()
    }
    fn fuzzy_init(&mut self) -> bool {
        if !self.state.do_fuzz_one() {
            return false
        }
        if !self.state.call_view().ok() {
            return true
        }
        self.do_init();
        self.state.do_fuzz_update()
    }
    fn state(&self) -> &State {
        &self.state
    }
}

unsafe impl Send for SocketState {}
unsafe impl Sync for SocketState {}
