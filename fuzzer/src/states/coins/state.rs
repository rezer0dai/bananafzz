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

pub struct CoinsState {
    pub state: State,
    pub shared: [u8; Move::Max as usize],
}

impl CoinsState {
    pub fn do_init(&mut self) -> Fd {
        Fd::new(&self.shared[Move::AbsX as usize..][..FD_SIZE])
    }
    pub fn fuzz_one(&mut self) -> bool {
        true
    }
    pub fn coins(fd: &[u8]) -> CoinsState {
        CoinsState::alert(fd)
    }
    pub fn brick(fd: &[u8]) -> CoinsState {
        CoinsState::alert(fd)
    }
    pub fn qboxs(fd: &[u8]) -> CoinsState {
        CoinsState::alert(fd)
    }
    fn alert(fd: &[u8]) -> CoinsState {
        let mut shared = [0u8; Move::Max as usize];

        shared[0] = Move::Coin as u8;
        shared[Move::AbsX as usize..][..FD_SIZE]
            .clone_from_slice(fd);

        CoinsState {
            state : State::new(
                "Coins",
                StateIds::FdCoins.into(),
                FD_SIZE,
                111,
                100,
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

static mut SMB_LEVEL_MAP: [u8; 4000] = [0; 4000];

unsafe fn register_coin(x: usize, y: usize) -> Vec<u8> { 
    assert!(x < SMB_LEVEL_MAP.len());
println!("===========> GOT COIN : {x} + {y}");
    if x < 0 {
        return vec![]
    }
    let z = y / 0x10;
    if z > 6 {
        return vec![]
    }
    let z = (2 << z) as u8; 
    let new_brick = 0 != SMB_LEVEL_MAP[x..][..8]
        .iter_mut()
        .filter(|b| 0 == **b & z)
        .map(|b| *b += z)
        .count();
    if !new_brick {
        return vec![]
    }
    [x as u32, y as u32]
        .iter()
        .flat_map(|n| n.to_be_bytes().to_vec())
        .collect()
}
#[no_mangle] 
#[must_use] 
pub unsafe extern "C" fn spot_coins(x: usize, y: usize) { 
    let fd = register_coin(x, y);
    if 0 == fd.len() {
        return
    }
    FuzzyState::fuzz(Box::new(
            CoinsState::coins(&fd)));
}
#[no_mangle] 
#[must_use] 
pub unsafe extern "C" fn spot_brick(x: usize, y: usize) { 
    let fd = register_coin(x, y);
    if 0 == fd.len() {
        return
    }
    FuzzyState::fuzz(Box::new(
            CoinsState::brick(&fd)));
}
#[no_mangle] 
#[must_use] 
pub unsafe extern "C" fn spot_qboxs(x: usize, y: usize) { 
println!("QBOXS");
    let fd = register_coin(x, y);
    if 0 == fd.len() {
        return
    }
    FuzzyState::fuzz(Box::new(
            CoinsState::qboxs(&fd)));
}
#[no_mangle] 
#[must_use] 
pub unsafe extern "C" fn abc() { 
    panic!("...");
}
