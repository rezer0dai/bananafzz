extern crate log;
use self::log::{info, debug};

use super::bananaq::{self, FuzzyQ};
use state::state::IFuzzyObj;

extern crate rand;
use rand::Rng;

use std::{
    sync::{Arc, Weak},
    thread, time,
};

/// fuzzing logic ( start, init, stop, what to share ) scoped in this object
pub struct FuzzyState {
    qid: u64,
    banana: Weak<FuzzyQ>,
    istate: Box<dyn IFuzzyObj>,
}
/// RAII guard
impl Drop for FuzzyState {
    fn drop(&mut self) {
        if let Err(e) = bananaq::dtor(&self.banana) {
            debug!(
                "[fuzzing] delayloaded drop of {} for bananaq#{:X} <{e}>",
                self.istate.state().info().name,
                self.qid
            );
        }
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
    pub fn fuzz(
        istate: Box<dyn IFuzzyObj>,
    ) -> Result<thread::JoinHandle<Result<(), String>>, &'static str> {
        Ok(thread::spawn(move || {
            let banana = istate.state().info().bananaq();

            if !bananaq::push(&banana, &istate)? {
                return Err(format!(
                    "[bananaq] failed to push {}",
                    istate.state().info().name
                ));
            }

            let racer = 0 != istate.state().level();
            if !racer {
                thread::sleep(time::Duration::from_millis(
                    rand::thread_rng()
                        .gen_range(0..=bananaq::config(&banana)?.push_sleep),
                ));
            }

            let mut fuzzy_state = FuzzyState::new(
                banana
                    .upgrade()
                    .ok_or(format!("[bananaq] bananaq#? is no longer"))?,
                istate,
            );

            if let Err(e) = fuzzy_state.init() {
                return Err(format!(
                    "[bananaq] FuzzyState {} failed to init with message <{}>",
                    fuzzy_state.istate.state().info().name,
                    e
                ));
            }
            if !bananaq::ctor_notify(fuzzy_state.istate.state().info()) {
                return Err(format!(
                    "[bananaq] FuzzyState {} <uid:{:?}> failed to register to queue",
                    fuzzy_state.istate.state().info().name,
                    fuzzy_state.istate.state().info().uid()
                ));
            }

            bananaq::update(&fuzzy_state.istate.state().info())?;

            if !racer {
                thread::sleep(time::Duration::from_millis(
                    rand::thread_rng()
                        .gen_range(0..=bananaq::config(&banana)?.after_creation_sleep),
                ));
            }

            info!("\t>> ALL GOO PROCEED TO FUZZ : {:?}", fuzzy_state.istate.state().info().name);

            for i in 0u16.. {
                //ok we want panic if we overdo it, as 0xFFFF is not reasonable fuzzing for any object ..
                fuzzy_state.istate.fuzzy_loop(i)?;

                thread::yield_now();
            }
            Ok(())
        }))
    }
    fn new(banana: Arc<FuzzyQ>, istate: Box<dyn IFuzzyObj>) -> FuzzyState {
        FuzzyState {
            qid: banana.read().unwrap().qid(),
            banana: Arc::downgrade(&banana),
            istate: istate,
        }
    }
    /// try to create state by invoking fuzz_init until is craeted ( level != 0 ) or further fuzzing is
    /// denied
    fn init(&mut self) -> Result<(), String> {
        while 0 == self.istate.state().level() || !self.istate.is_online() {
            self.istate.fuzzy_init()?;
        }
        if self.istate.invalid() {
            return Err(format!("invalid state after initialization"));
        }
        Ok(())
    }
}
