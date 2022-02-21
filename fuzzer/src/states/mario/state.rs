extern crate core;
use self::core::exec::fd_info::Fd;
use self::core::exec::call::Call;
use self::core::state::state::State;

use common::table::*;

use calls::dummy::DummyExec;

use calls::ko::GameOver;
use calls::load::LoadPos;
use calls::movem::MoveMario;

use args::smb2::POS_SIZE;

//use core::banana::looper::FuzzyState;

//extern crate rand;
//use rand::Rng;

pub struct MarioState {
    pub(in super::super::mario) state: State,
    pub(in super::super::mario) shared: [u8; POS_SIZE],
}

impl MarioState {
    pub(in super::super::mario) fn fuzz_one(&mut self) -> bool {
        true
    }
    pub(in super::super::mario) fn do_init(&mut self) -> Fd {
        Fd::new(&[0x88; 4])
    }
    pub(crate) fn spawn() -> MarioState {
        MarioState {
            state : State::new(
                "Mario",
                StateIds::FdMario.into(),
                1000,
                vec![[1, 1], [0, 1], [-1, -1]],
                MarioState::init_calltable(),
                Call::game_over()),
            shared : [0u8; POS_SIZE],
        }
    }
    fn init_calltable() -> Vec< Vec<Call> > {
        vec![
            vec![
                Call::dummy(),
            ],
            vec![
                Call::load_pos(),
                Call::move_mario(),
            ],
            vec![
                Call::load_pos(),
                Call::move_mario(),
            ]]
    }
}
