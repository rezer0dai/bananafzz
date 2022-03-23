extern crate core;
use self::core::state::state::{
    IFuzzyObj,
    State
};
use super::state::*;

impl IFuzzyObj for MarioState {
    fn fuzzy_loop(&mut self, _stage_idx: u16) -> Result<(), String> {
        if let Err(e) = self.state.do_fuzz_one(&mut self.shared) {
            println!("[loop] quit : <{}>", e);
            return Err(e)
        }

        self.fuzz_one()?;

        if let Err(e) = self.state.do_fuzz_update(&mut self.shared) {
            println!("[loop] quit : <{}>", e);
            return Err(e)
        }
        Ok(())
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

unsafe impl Send for MarioState {}
unsafe impl Sync for MarioState {}
