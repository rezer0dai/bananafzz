use std::collections::HashMap;

extern crate rand;
use rand::seq::SliceRandom;
use rand::Rng;

extern crate generic;

extern crate core;
use self::core::generator::leaf::IArgLeaf;
use self::core::generator::serialize::ISerializableArg;
use self::core::generator::serialize::SerializationInfo;
use self::core::state::id::StateTableId;

use self::core::banana::bananaq::{self, FuzzyQ};
use std::sync::Weak;

use self::core::exec::fd_info::Fd;

use super::const_leaf::Const;
use super::phantom_leaf::Phantom;

/// state fd for interconnected behaviour
/// - define our state object
/// - represents uniqueness in target state space
pub struct FdHolder {
    size: usize,
    fds: Vec<Box<dyn IArgLeaf>>,
    sid: u64,
}
impl FdHolder {
    pub fn new(size: usize, sid: u64, fds: Vec<Box<dyn IArgLeaf>>) -> FdHolder {
        if fds.iter().any(|fd| fd.size() > size) {
            panic!("FdHolder::new .. one of fd have bigger size than declared!")
        }
        assert!(fds.iter().all(|fd| fd.size() <= size));
        FdHolder {
            size: size,
            fds: fds,
            sid: sid,
        }
    }
    pub fn dup(sid: u64, fd: &[u8]) -> FdHolder {
        FdHolder::new(fd.len(), sid, vec![Box::new(Const::new(fd))])
    }
    pub fn holder(sid: u64, size: usize) -> FdHolder {
        FdHolder::new(size, sid, vec![Box::new(Phantom::new(size))])
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
    fn dump(&self, _mem: &[u8]) -> Vec<u8> { vec![] }
    fn load(
        &mut self,
        mem: &mut [u8],
        _dump: &[u8],
        _poc_fd: &[u8],
        fd_lookup: &HashMap<Vec<u8>, Vec<u8>>,
    ) -> Result<usize, String> {
        // bfl specific
        let mut fd = self.sid.to_le_bytes().to_vec();
        fd.extend_from_slice(&[0x42u8; 4+2]);
        fd.extend_from_slice(mem);
        
        if let Some(fd) = fd_lookup.get(&fd) {
            mem.clone_from_slice(&fd[fd.len() - mem.len()..])
        } // as we may try constant FD at fuzzing, not yet added to queue ?
        else { // should be empty or invalid FD !! 
            return Err(format!("--> FD {fd:?} NOT FOUND at table\n{fd_lookup:?}"))
        } // keep for now debug print, later we will kick it off once we debuged it enough :)
        Ok(0) // no any of dump memory was used
    }
}
impl IArgLeaf for FdHolder {
    fn size(&self) -> usize {
        self.size
    }

    fn name(&self) -> &'static str {
        "Fd"
    }

    fn generate_unsafe(
        &mut self,
        bananaq: &Weak<FuzzyQ>,
        mem: &mut [u8],
        fd: &[u8],
        shared: &mut[u8],
    ) {
        self.fds
            .choose_mut(&mut rand::thread_rng())
            .unwrap()
            .generate(bananaq, mem, fd, shared);
    }
}

pub struct RndFd {
    sid: StateTableId,
    size: usize,
}

impl RndFd {
    pub fn new(sid: StateTableId, size: usize) -> FdHolder {
        FdHolder::new(size, sid.into(), vec![Box::new(RndFd { sid: sid, size: size })])
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
    fn generate_unsafe(&mut self, bananaq: &Weak<FuzzyQ>, mem: &mut [u8], _: &[u8], _: &mut[u8]) {
        // for soly we skip dummy & invalid
        match rand::thread_rng().gen_range(2u8..=7) {
            0 => mem.clone_from_slice(&Fd::dummy(self.size()).data()),
            1 => mem.clone_from_slice(&Fd::invalid(self.size()).data()),
            _ => {
                mem.clone_from_slice(&Fd::dummy(self.size()).data());

                let fd = match bananaq::get_rnd_fd(bananaq, self.sid.clone(), self.size) {
                    Ok(fd) => fd,
                    Err(_) => return,
                };
                if fd.data().is_empty() {
                    return;
                }
                if fd.data().len() != mem.len() {
                    //unsafe { asm!("int3") }
                    panic!("Random argument selection failed on size mismatch of : {:?} where : {} vs {}", self.sid, fd.data().len(), mem.len())
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
