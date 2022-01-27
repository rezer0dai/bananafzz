use std::collections::HashMap;

extern crate rand;
use rand::Rng;
use rand::seq::SliceRandom;

extern crate generic;

extern crate core;
use self::core::generator::leaf::IArgLeaf;
use self::core::generator::serialize::ISerializableArg;
use self::core::generator::serialize::SerializationInfo;
use self::core::banana::bananaq;
use self::core::state::id::StateTableId;

use self::core::exec::fd_info::Fd;

use super::const_leaf::Const;
use super::phantom_leaf::Phantom;
use super::bfl_leaf::Bfl;

/// state fd for interconnected behaviour
/// - define our state object
/// - represents uniqueness in target state space
pub struct FdHolder {
    size: usize,
    fds: Vec<Box<dyn IArgLeaf>>,
}
impl FdHolder {
    pub fn new(size: usize, fds: Vec<Box<dyn IArgLeaf>>) -> Bfl::<FdHolder> {
        if fds.iter().any(|fd| fd.size() > size) {
            panic!("FdHolder::new .. one of fd have bigger size than declared!")
        }
        assert!(fds.iter().all(|fd| fd.size() <= size));
        Bfl::new(FdHolder {
            size: size,
            fds: fds,
        })
    }
    pub fn dup(fd: &[u8]) -> Bfl::<FdHolder> {
        FdHolder::new(fd.len(), vec![Box::new(Const::new(fd))])
    }
    pub fn holder(size: usize) -> Bfl::<FdHolder> {
        FdHolder::new(size, vec![Box::new(Phantom::new(size))])
    }
}
/// we just copy out whatever was generated, as it is stored in &mem before doing serialization
impl ISerializableArg for FdHolder {
    fn serialize(&self, mem: &[u8], _: &[u8], _: &[u8]) -> Vec<SerializationInfo> {
        vec![SerializationInfo {
            offset: 0,
            prefix: String::from("shared_fd(fd_") + &generic::u8_to_str(mem) + ",",
        }]
    }
    fn load(&mut self, mem: &mut[u8], _dump: &[u8], poc_fd: &[u8], fd_lookup: &HashMap<Vec<u8>,Vec<u8>>) -> usize {
        assert!(poc_fd.len() == mem.len(), "[BFL] we got strange poc_fd {:?} should be of size {}", poc_fd, mem.len());
        mem.clone_from_slice(&fd_lookup[poc_fd]);
        0 
    }
}
impl IArgLeaf for FdHolder {
    fn size(&self) -> usize {
        self.size
    }

    fn name(&self) -> &'static str {
        "Fd"
    }

    fn generate_unsafe(&mut self, mem: &mut [u8], fd: &[u8], shared: &[u8]) {
        self.fds
            .choose_mut(&mut rand::thread_rng())
            .unwrap()
            .generate(mem, fd, shared);
    }
}

pub struct RndFd {
    id: StateTableId,
    size: usize,
}

impl RndFd {
    pub fn new(id: StateTableId, size: usize) -> Bfl::<RndFd> {
        Bfl::new(RndFd { id: id, size: size })
    }
}

impl IArgLeaf for RndFd {
    fn size(&self) -> usize {
        self.size
    }

    fn name(&self) -> &'static str {
        "RndFd"
    }

    /// 4:6 we share valid object / state
    ///
    /// other time we provide NULL or invalid one
    fn generate_unsafe(&mut self, mem: &mut [u8], _: &[u8], _: &[u8]) {
        match rand::thread_rng().gen_range(0u8..=7) {
            0 => mem.clone_from_slice(&Fd::dummy(self.size()).data()),
            1 => mem.clone_from_slice(&Fd::invalid(self.size()).data()),
            _ => {
                mem.clone_from_slice(&Fd::dummy(self.size()).data());

                let fd = bananaq::get_rnd_fd(self.id.clone());
                if fd.data().is_empty() {
                    return;
                }
                if fd.data().len() != mem.len() {
                    //unsafe { asm!("int3") }
                    panic!("Random argument selection failed on size mismatch of : {:?} where : {} vs {}", self.id, fd.data().len(), mem.len())
                }
                //mem[..fd.len()].clone_from_slice(&fd);
                mem.clone_from_slice(fd.data());
            }
        };
    }
}

/// must be used within FdHolder::new argument!
impl ISerializableArg for RndFd {
    fn serialize(&self, _: &[u8], _: &[u8], _: &[u8]) -> Vec<SerializationInfo> {
        panic!("RndFd must be scoped whitin FdHolder argument!");
    }
}
