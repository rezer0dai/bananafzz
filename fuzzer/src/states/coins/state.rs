extern crate core;
use self::core::exec::fd_info::Fd;
use self::core::exec::call::Call;
use self::core::state::{state::State, id::StateTableId};

use common::table::*;

use calls::dummy::DummyExec;

use calls::ko::GameOver;
use calls::load::LoadPos;
use calls::movem::MoveMario;

use args::smb2::{Move, FD_SIZE};

use core::banana::looper::FuzzyState;

use std::sync::Weak;
use core::banana::bananaq;
use core::banana::bananaq::FuzzyQ;

pub struct CoinsState {
    pub state: State,
    pub shared: [u8; Move::Max as usize],
}

impl CoinsState {
    pub fn do_init(&mut self) -> Fd {
        Fd::new(&self.shared[Move::AbsX as usize..][..FD_SIZE])
    }
    //this function is called only if bananaq is active!!
    pub fn fuzz_one(&mut self) -> Result<(), String> {
        Ok(())
    }

    pub fn coins(banana: Weak<FuzzyQ>, fd: &[u8]) {
        FuzzyState::fuzz(Box::new(
            CoinsState::alert(banana, fd, StateIds::FdCoins.into())));
    }
    pub fn brick(banana: Weak<FuzzyQ>, fd: &[u8]) {
        FuzzyState::fuzz(Box::new(
            CoinsState::alert(banana, fd, StateIds::FdBrick.into())));
    }
    pub fn qboxs(banana: Weak<FuzzyQ>, fd: &[u8]) {
        FuzzyState::fuzz(Box::new(
            CoinsState::alert(banana, fd, StateIds::FdQBoxs.into())));
    }

    pub fn alert(banana: Weak<FuzzyQ>, fd: &[u8], sid: StateTableId) -> CoinsState {
        let mut shared = [0u8; Move::Max as usize];

        shared[0] = Move::Coin as u8;
        shared[Move::AbsX as usize..][..FD_SIZE]
            .clone_from_slice(fd);

        CoinsState {
            state : State::new(
                banana,
                "Coins",
                sid,
                FD_SIZE,
                20,
                42,
                vec![[0, 1], [0, 1], [-1, 1], [-2, -2]],
                CoinsState::init_calltable(),
                Call::succ()),
            shared : shared,
        }
    }

    fn init_calltable() -> Vec< Vec<Call> > {
        vec![
            vec![ Call::succ() ],
            vec![ Call::is_active() ],
            vec![ Call::eval_pos() ],
            vec![ Call::move_mario() ]]
    }
}
