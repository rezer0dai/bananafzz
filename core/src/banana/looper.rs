use super::bananaq;
use state::state::IFuzzyObj;
use config::FZZCONFIG;

extern crate rand;
use rand::Rng;

use std::{
    thread,
    time
};

/// fuzzing logic ( start, init, stop, what to share ) scoped in this object
pub struct FuzzyState {
    istate: Box<dyn IFuzzyObj>
}
/// RAII guard
impl Drop for FuzzyState {
    fn drop(&mut self) {
        bananaq::pop();
    }
}
impl FuzzyState {
/// forwarded new state to fuzz, therefore create special thread for it and handle fuzzing
///
/// - init
/// - handle notifications ( createstate + update )
/// - invoke fuzzy method
/// - yield to allow other threads and fuzzing more shuffling ( better to swap exec time between threads a lot )
/// - check for end-conditions of fuzz and quit
    pub fn fuzz(istate: Box<dyn IFuzzyObj>) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            if !bananaq::push(&istate) {
                return
            }
            let racer = 0 != istate.state().level();

            let mut fuzzy_state = FuzzyState::new(istate);
            if !fuzzy_state.init() {
                return
            }
            if !bananaq::ctor_notify(&fuzzy_state.istate) {
                return
            }

            bananaq::update(&fuzzy_state.istate);

            if !racer {
                thread::sleep(time::Duration::from_millis(
                    rand::thread_rng().gen_range(0..=FZZCONFIG.after_creation_sleep)));
            }

            for i in 0u16.. {//ok we want panic if we overdo it, as 0xFFFF is not reasonable fuzzing for any object ..
                if 0 == (i % FZZCONFIG.state_update_freq) {
                    bananaq::update(&fuzzy_state.istate);
                }
                if !fuzzy_state.istate.fuzzy_loop() {
                    break
                }
                thread::yield_now();
            }
        })
    }
    fn new(istate: Box<dyn IFuzzyObj>) -> FuzzyState {
        FuzzyState {
            istate : istate,
        }
    }
/// try to create state by invoking fuzz_init until is craeted ( level != 0 ) or further fuzzing is
/// denied
    fn init(&mut self) -> bool {
        while 0 == self.istate.state().level() || !self.istate.is_online() {
            if !self.istate.fuzzy_init() {
                return false
            }
        }
        !self.istate.invalid()
    }
}
