extern crate core;
use self::core::exec::fd_info::Fd;
use self::core::exec::call::Call;
use self::core::state::{state::State, id::StateTableId};

use common::table::*;

use calls::dummy::DummyExec;

use calls::load::LoadPos;
use calls::movem::MoveMario;

use args::smb2::{Move, FD_SIZE};

use std::sync::Weak;
use core::banana::bananaq::FuzzyQ;

#[allow(improper_ctypes)]
extern "C" {
    pub fn push_state(bananaq: &Weak<FuzzyQ>, state: StateTableId, fd: &Fd);
}

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
        unsafe {
            push_state(&banana, StateIds::FdCoins.into(), &Fd::new(fd))
        }
    }
    pub fn brick(banana: Weak<FuzzyQ>, fd: &[u8]) {
        unsafe {
            push_state(&banana, StateIds::FdBrick.into(), &Fd::new(fd))
        }
    }
    pub fn qboxs(banana: Weak<FuzzyQ>, fd: &[u8]) {
        unsafe {
            push_state(&banana, StateIds::FdQBoxs.into(), &Fd::new(fd))
        }
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
                vec![[0, 1], [0, 1], [-1, 1], [-2, -2]],
                CoinsState::init_calltable(),
                Call::succ()),
            shared : shared,
        }
    }

    fn init_calltable() -> Vec< Vec<Call> > {
        vec![
            vec![ Call::ok_ctor() ],
            vec![ Call::is_active() ],
            vec![ Call::eval_pos() ],
            vec![ Call::move_mario() ]]
    }
}
