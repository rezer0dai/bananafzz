extern crate core;
use self::core::state::state::{
    IFuzzyObj,
    State
};
use super::state::*;

impl IFuzzyObj for CoinsState {
    fn fuzzy_loop(&mut self, _stage_idx: u16) -> Result<(), String> {
        self.state.do_fuzz_one(&mut self.shared)?;

        self.fuzz_one()?;

        self.state.do_fuzz_update(&mut self.shared)
    }
    fn fuzzy_init(&mut self) -> Result<(), String> {
        self.state.do_fuzz_one(&mut self.shared)?;

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
