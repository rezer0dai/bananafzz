extern crate core;
use self::core::exec::fd_info::Fd;
use self::core::exec::call::Call;
use self::core::state::state::State;

use common::table::*;

use calls::dummy::DummyExec;

use calls::ko::GameOver;
use calls::load::LoadPos;
use calls::movem::MoveMario;

use args::smb2::{Move, FD_SIZE};

use core::banana::looper::FuzzyState;

//extern crate rand;
//use rand::Rng;

pub struct MarioState {
    pub state: State,
    pub shared: [u8; Move::Max as usize],
}
                
impl MarioState {
    pub fn do_init(&mut self) -> Fd {
        Fd::new(&[self.shared[0]; FD_SIZE])
    }

    pub fn fuzz_one(&mut self) -> bool {
//        return true;
        if CallIds::move_mario != self.state.call_view().id().into() {
            return true
        }
        if !self.state.call_view().ok() {
            return true
        }
        let enemies = &self.shared[Move::Shroom as usize..];

//move this to plugin, to call once, all loads was called
        for e in enemies//including shroom
            .iter()
            .enumerate()
            .step_by(2)
            .filter(|(_, x)| &0 != *x)
            .map(|(i, _)| (Move::Shroom as u8 + i as u8).into()) {
                for _ in 0..5 {
                    FuzzyState::fuzz(Box::new(MarioState::enemy_alert(&e)));
                    if Move::Shroom != e {
                        break//shroom is top prio to hit!
                    }
                }
        }
        true
    }

    pub fn spawn() -> MarioState {
        MarioState::mario(Move::Mario)
    }
    pub fn enemy(eid: u8) -> MarioState {
        MarioState::mario((eid * 2 + Move::Enemy as u8).into())
    }
    pub fn shroom() -> MarioState {
        MarioState::mario(Move::Shroom)
    }

    fn mario(mid: Move) -> MarioState {
        let mut shared = [0u8; Move::Max as usize];
        shared[0] = mid.clone() as u8;

        MarioState {
            state : State::new(
                "Mario",
                mid.into(),
                FD_SIZE,
                1111,
                100,
                vec![[0, 1], [0, 1], [-1, -1]],
                MarioState::init_calltable(),
                Call::game_over()),
            shared : shared,
        }
    }

    pub fn enemy_alert(eid: &Move) -> MarioState {
        MarioState::alert(eid.clone())
    }
    fn alert(mid: Move) -> MarioState {
        let mut shared = [0u8; Move::Max as usize];
        shared[0] = mid.clone() as u8;

        MarioState {
            state : State::duped(
                "Mario",
                mid.into(),
                &Fd::new(&[shared[0]; FD_SIZE]),
                22,
                100,
                vec![[0, 1], [0, 1], [-1, -1]],
                MarioState::init_calltable(),
                Call::succ()),
            shared : shared,
        }
    }

    fn init_calltable() -> Vec< Vec<Call> > {
        vec![
            vec![ Call::succ() ],
            vec![ Call::load_pos() ],
            vec![ Call::move_mario() ]
        ]
    }
}
