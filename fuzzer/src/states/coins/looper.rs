extern crate core;
use self::core::state::state::{
    IFuzzyObj,
    State
};
use self::core::exec::fd_info::Fd;

use super::state::*;

impl IFuzzyObj for CoinsState {
    fn fuzzy_loop(&mut self) -> bool {
        if !self.state.do_fuzz_one(&mut self.shared) {
            return false
        }
        if !self.fuzz_one() {
            return false
        }
//        if self.state.call_view().ok() {
//            println!("OK : {}", self.state.call_view().name())
//        }
        if !self.state.do_fuzz_update(&mut self.shared) {
            return false
        }

        true
    }
    fn fuzzy_init(&mut self) -> bool {
        if !self.state.do_fuzz_one(&mut self.shared) {
            return false
        }

        let fd = self.do_init();
        self.state.init(&fd);
        self.state.do_fuzz_update(&mut self.shared)
    }
    fn state(&self) -> &State {
        &self.state
    }
}

unsafe impl Send for CoinsState {}
unsafe impl Sync for CoinsState {}
