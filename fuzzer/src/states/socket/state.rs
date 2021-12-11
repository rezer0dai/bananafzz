extern crate core;
use self::core::exec::fd_info::Fd;
use self::core::exec::call::Call;
use self::core::state::state::State;

use common::table::*;

use calls::socket::socket::SocketExec;
use calls::socket::accept::AcceptExec;
use calls::socket::bind::BindExec;
use calls::socket::connect::ConnectExec;
use calls::socket::listen::ListenExec;
use calls::socket::recv::RecvExec;
use calls::socket::send::SendExec;
use calls::socket::shutdown::ShutDownExec;

use calls::epoll::poll::PollExec;

use calls::dup::DupExec;
use calls::dummy::DummyExec;
use calls::close::CloseExec;

use core::banana::looper::FuzzyState;

use super::super::super::common::table::SOCKET_FD_SIZE;

extern crate rand;
use rand::Rng;

pub struct SocketState {
    pub(in super::super::socket) state: State,
}

impl SocketState {
    pub(in super::super::socket) fn fuzz_one(&mut self) -> bool {
        match CallIds::from(self.state.call_view().id()) {
            CallIds::accept => if self.state.call_view().ok() {
                let mut fd = [0u8; SOCKET_FD_SIZE];
                fd.clone_from_slice(&self.state.call_view().einfo());
// accept seems rather specific case, must be called from server only thread
// in this case we must manually push object to fuzzing queue
                println!("ACCEPTED!");
// - first push is mainly for enabling races ~ its ctor is dup, after is dupped raceunlock plugin can
// start putin racing threads to it
                FuzzyState::fuzz(Box::new(SocketState::dup(&Fd::new(&fd), true)));
// - second push is to allow original accepted handle be closed for good, as first push allows to
// close only duped one ~ though certain probability that it will dangle
                FuzzyState::fuzz(Box::new(SocketState::dup(&Fd::new(&fd), false)));
// - yep i can close or dup handle here manually but that will be problematic for POC reproduction
// while in this official way poclog plugin can easily make authentic reproducer
            },
            _ => ()
        };
        true
    }
    pub(in super::super::socket) fn do_init(&mut self) {
        let fd = Fd::new(self.state.call_view().einfo());
        self.state.init(&fd); //this is to let know global handle table
    }
    pub(crate) fn server() -> SocketState {
        SocketState {
            state : State::new(
                "Socket-Server",
                StateIds::FdSocket.into(),
                300,
//server table
//vec![[0, 1], [0, 0], [-1, 1], [1, 1], [-3, -3], [0, 0], [0, 0], [0, 0], [0, 0]],
//vec![[0, 5], [0, 0], [0, 0], [0, 0], [0, 0], [1, 1], [0, 1], [-2, 0], [0, 0]],

                if rand::thread_rng().gen_bool(0.66) { vec![[0, 1], [0, 1], [-1, 1], [1, 1], [-3, -3], [0, 0], [0, 0], [0, 0], [0, 0]] }
                else { vec![[0, 5], [0, 0], [0, 0], [0, 0], [0, 0], [1, 1], [0, 1], [-2, 0], [0, 0]] },

//                vec![[0, 5], [0, 0], [0, 0], [0, 0], [0, 0], [1, 1], [0, 1], [-2, 0], [0, 0]],
                SocketState::init_calltable(&[]),
                Call::close()),
        }
    }
    pub(crate) fn client() -> SocketState {
        SocketState {
            state : State::new(
                "Socket-Client",
                StateIds::FdSocket.into(),
                200,
                vec![[0, 8], [0, 1], [-1, 1], [1, 1], [4, 4], [0, 0], [0, 0], [0, 0], [0, -7]],
                SocketState::init_calltable(&[]),
                Call::close()),
        }
    }
    pub(crate) fn dup(fd: &Fd, force: bool) -> SocketState {
        SocketState {
            state : if !force && rand::thread_rng().gen_bool(0.5) {
              State::duped(
                "Socket-Racer-Shadow",
                StateIds::FdSocket.into(),
                fd,
                100,
                vec![[0, 1], [0, 1], [-1, 1], [1, 1], [-3, -3], [0, 0], [0, 0], [0, 0], [0, 0]],
                SocketState::init_calltable(&[]),
                if rand::thread_rng().gen_bool(0.9) {
                  Call::dummy()
                } else {
                  Call::close()
                })
            } else {
              State::new(
                "Socket-Racer-Dup",
                StateIds::FdSocket.into(),
                100,
                vec![[0, 8], [0, 1], [-1, 1], [1, 1], [4, 4], [0, 0], [0, 0], [0, 0], [0, -7]],
                SocketState::init_calltable(fd.data()),
                Call::close())
            }
        }
    }
    fn init_calltable(fd: &[u8]) -> Vec< Vec<Call> > {
        vec![
            if 0 == fd.len() {
              vec![
                  Call::socket(),
                  Call::socket(),
                  Call::accept(true),//seems does not make any sense here
              ]
            } else {
              vec![
                  Call::dup(fd),
              ]
            },
            vec![

                Call::send(),
                Call::recv(),
                Call::send(),
                Call::recv(),

            ],
            vec![
                Call::send(),
                Call::recv(),
            ], vec![
                Call::send(),
                Call::recv(),
            ], vec![
                Call::send(),
                Call::recv(),
                Call::poll(),
                Call::shutdown(),
            ], vec![
                Call::bind(),
            ], vec![
                Call::listen(),
            ], vec![
                Call::accept(false),
            ],
            vec![
                Call::connect(),
            ]]
    }
}
