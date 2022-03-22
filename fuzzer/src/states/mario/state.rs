use std::collections::HashSet;
use std::sync::RwLock;

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

use std::sync::Weak;
use core::banana::bananaq;
use core::banana::bananaq::FuzzyQ;
use core::state::id::StateTableId;

use super::super::coins::state::*;

//extern crate rand;
//use rand::Rng;
//
lazy_static! {
    static ref COINS: RwLock<HashSet<Vec<u8>>> = RwLock::new(HashSet::new());
    static ref BRICK: RwLock<HashSet<Vec<u8>>> = RwLock::new(HashSet::new());
    static ref QBOXS: RwLock<HashSet<Vec<u8>>> = RwLock::new(HashSet::new());
}

pub struct MarioState {
    pub state: State,
    pub shared: [u8; Move::Max as usize],
}
                
impl MarioState {
    pub fn do_init(&mut self) -> Fd {
        panic!("dupers only");
        Fd::new(&[self.shared[0]; FD_SIZE])
    }

    fn enemies(&mut self) -> Result<(), String> {
        let enemies = &self.shared[Move::Shroom as usize..];

//move this to plugin, to call once, all loads was called
        for e in enemies//including shroom
            .iter()
            .enumerate()
            .step_by(2)
            .filter(|(_, x)| &0 != *x)
//            .filter(|(i, _)| *i < 2)
            .map(|(i, _)| (Move::Shroom as u8 + i as u8).into()) {
println!("ENEMY SPOTED {e:?}");
                FuzzyState::fuzz(Box::new(
                        MarioState::enemy_alert(self.state.info().bananaq(), &e)))?;
                if Move::Shroom == e {// shrooms are priority!!
                    FuzzyState::fuzz(Box::new(
                            MarioState::enemy_alert(self.state.info().bananaq(), &e)))?;
                }
        }

        Ok(())
    }

    pub fn fuzz_one(&mut self) -> Result<(), String> {

        COINS.write().unwrap().drain().for_each(|ref fd| CoinsState::coins(
            self.state.info().bananaq().clone(), fd));
        BRICK.write().unwrap().drain().for_each(|ref fd| CoinsState::brick(
            self.state.info().bananaq().clone(), fd));
        QBOXS.write().unwrap().drain().for_each(|ref fd| CoinsState::qboxs(
            self.state.info().bananaq().clone(), fd));

//return Ok(());
        if CallIds::move_mario != self.state.call_view().id().into() {
            return Ok(())
        }
        if StateIds::FdMario as u64 != u64::from(self.state.id()) {
            return Ok(())
        }
        /*
        if !self.state.call_view().ok() {
            return true
        }
        */
        self.enemies()
    }

    pub fn spawn(banana: Weak<FuzzyQ>) -> MarioState {
        let mid = Move::Mario;
        let mut shared = [0u8; Move::Max as usize];
        shared[0] = mid.clone() as u8;

        MarioState {
            state : State::duped(
                banana,
                "MarioBroS",
                mid.into(),
                &Fd::new(&[shared[0]; FD_SIZE]),
                333,
                66,
                vec![[0, 1], [0, 1], [-1, -1]],
                MarioState::init_calltable(),
                Call::game_over()),
            shared : shared,
        }
    }

    pub fn enemy_alert(banana: Weak<FuzzyQ>, eid: &Move) -> MarioState {
        MarioState::alert(banana, eid.clone())
    }
    fn alert(banana: Weak<FuzzyQ>, mid: Move) -> MarioState {
        let mut shared = [0u8; Move::Max as usize];
        shared[0] = mid.clone() as u8;

        MarioState {
            state : State::duped(
                banana,
                "CatchingMArio",
                mid.into(),
                &Fd::new(&[shared[0]; FD_SIZE]),
                5,
                0x42,
                vec![[0, 1], [0, 1], [-1, -1]],
                MarioState::init_calltable(),
                Call::succ()),
            shared : shared,
        }
    }

    fn init_calltable() -> Vec< Vec<Call> > {
        vec![
            vec![ ],
            vec![ Call::load_pos() ],
            vec![ Call::move_mario() ]
        ]
    }
}

static mut SMB_LEVEL_MAP: [u8; 4000] = [0; 4000];
static mut HITCOUNT: [u8; 4000] = [0; 4000];
static mut Y_BASE: usize = 80;

unsafe fn register_coin(x: usize, y: usize) -> Vec<u8> { 
    assert!(x < SMB_LEVEL_MAP.len());
    let z = ((y - Y_BASE) / 0x10) / 8;
    if x > SMB_LEVEL_MAP.len() || z > 6 {
        return vec![]
    }
    HITCOUNT[x] = (HITCOUNT[x] + 1) % 4;
    if 0 == HITCOUNT[x] {
        SMB_LEVEL_MAP[x..][..8].fill(0);
    }
    let z = (2 << z) as u8; 
    let new_brick = 0 != SMB_LEVEL_MAP[x..][..8]
        .iter_mut()
        .filter(|b| 0 == **b & z)
        .map(|b| *b |= z)
        .count();
    if !new_brick {
        return vec![]
    }
//println!("GOOD COIN? {:?} + {:?}", x, y);
    [x as u32, y as u32]
        .iter()
        .flat_map(|n| n.to_le_bytes().to_vec())
        .collect()
}

#[no_mangle] 
#[must_use] 
pub unsafe extern "C" fn spot_coins(x: usize, y: usize) { 
    let fd = register_coin(x, y);
    if 0 == fd.len() {
        return
    }
    COINS.write().unwrap().insert(fd);
}
#[no_mangle] 
#[must_use] 
pub unsafe extern "C" fn spot_brick(x: usize, y: usize) { 
    let fd = register_coin(x, y + 10);//we want on top of BRICK!!
    if 0 == fd.len() {
        return
    }
    BRICK.write().unwrap().insert(fd);
}
#[no_mangle] 
#[must_use] 
pub unsafe extern "C" fn spot_qboxs(x: usize, y: usize) { 
    let fd = register_coin(x, y);
    if 0 == fd.len() {
        return
    }
    QBOXS.write().unwrap().insert(fd);
}

#[no_mangle] 
#[must_use] 
pub fn reset_coins() {
    COINS.write().unwrap().clear();
    BRICK.write().unwrap().clear();
    QBOXS.write().unwrap().clear();
}
