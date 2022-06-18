use std::collections::HashMap;

extern crate rand;
use rand::Rng;

extern crate generic;

extern crate core;
use self::core::generator::leaf::IArgLeaf;
use self::core::generator::serialize::{ISerializableArg,SerializationInfo};
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
    idx: usize,
}
impl FdHolder {
    pub fn new(size: usize, fds: Vec<Box<dyn IArgLeaf>>) -> FdHolder {
        if fds.iter().any(|fd| fd.size() > size) {
            panic!("FdHolder::new .. one of fd have bigger size than declared!")
        }
        assert!(fds.iter().all(|fd| fd.size() <= size));
        FdHolder {
            size: size,
            fds: fds,
            idx: 0,
        }
    }
    pub fn dup(fd: &[u8]) -> FdHolder {
        FdHolder::new(fd.len(), vec![Box::new(Const::new(fd))])
    }
    pub fn holder(size: usize) -> FdHolder {
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
    fn mem(&self, mem: &[u8]) -> Vec<u8> {
        self.fds[self.idx].mem(mem)
    }
    // TODO : it will break BFL, this is just temporary, need to update bfl plugin
    // to forward arguments specific metadata!!
    fn dump(&self, mem: &[u8]) -> Vec<u8> { 
        let mut dump = self.default_dump(&self.idx.to_le_bytes());
        dump.extend(self.fds[self.idx].dump(mem));
        dump
    }
    fn load(
        &mut self,
        mem: &mut [u8],
        dump: &[u8],
        poc_mem: &[u8],
        prefix: &[u8],
        fd_lookup: &HashMap<Vec<u8>, Vec<u8>>,
    ) -> Result<usize, String> {

        let mut idx = [0u8; 8];
        let n_loaded = self.default_load(&mut idx, dump, &dump[..8]);
        if idx.len() * 2 != n_loaded {
            panic!("wrong size data : {dump:?} --> {:?}", self.default_load(&mut idx, dump, &dump[8..]))
        }
        self.idx = usize::from_le_bytes(idx);

        assert!(self.idx < self.fds.len());

        self.fds[self.idx].load(
            mem, &dump[n_loaded..], poc_mem, prefix, fd_lookup)
            .map(|size| size + n_loaded)
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
    ) -> bool {
        self.idx = rand::thread_rng().gen_range(0..self.fds.len());
        self.fds[self.idx].generate(bananaq, mem, fd, shared)
    }
}

pub struct RndFd {
    sid: StateTableId,
    size: usize,
    offset: usize,
}

impl RndFd {
    pub fn new(sid: StateTableId, size: usize) -> FdHolder {
        FdHolder::new(size, vec![Box::new(RndFd { sid, size, offset: 0 })])
    }
    pub fn off(sid: StateTableId, size: usize, offset: usize) -> FdHolder {
        FdHolder::new(size, vec![Box::new(RndFd { sid, size, offset })])
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
    fn generate_conditioned(&mut self, bananaq: &Weak<FuzzyQ>, mem: &mut [u8], _: &[u8], _: &mut[u8]) -> bool {
        // for soly we skip dummy & invalid
        match rand::thread_rng().gen_range(2u8..=7) {
            0 => mem.clone_from_slice(&Fd::dummy(self.size()).data()),
            1 => mem.clone_from_slice(&Fd::invalid(self.size()).data()),
            _ => {
                mem.clone_from_slice(&Fd::dummy(self.size()).data());

                let fd = match bananaq::get_rnd_fd(bananaq, self.sid.clone()) {
                    Ok(Some((fd, _))) => fd,
                    _ => return false,
                };

                if fd.data().iter().take(8).all(|&b| 0 == b) {
                    return false
                }

                if fd.data().len() < mem.len() {
                    panic!("Random argument selection failed on size mismatch of : {:?} where : {} vs {}", self.sid, fd.data().len(), mem.len())
                } else if fd.data().len() != mem.len() {
                    log::debug!("Random argument selection failed on size mismatch of : {:?} where : {} vs {}", self.sid, fd.data().len(), mem.len());
                }
                mem.clone_from_slice(&fd.data()[self.offset..][..self.size]);
            }
        };
        true
    }
}

/// must be used within FdHolder::new argument!
impl ISerializableArg for RndFd {
    fn dump(&self, _mem: &[u8]) -> Vec<u8> { vec![] }
    fn load(
        &mut self,
        mem: &mut [u8],
        _dump: &[u8],
        poc_mem: &[u8],
        prefix: &[u8],
        fd_lookup: &HashMap<Vec<u8>, Vec<u8>>,
    ) -> Result<usize, String> {
        // bfl specific
        let mut fd = prefix.to_vec();
        fd.extend_from_slice(poc_mem);
        
// iterate trough all fds in case if not in lookup table, as one
// may have transformed it ?? calling load on it should -detransform
// rnd fd load is empy, but we can make transform not-empty load
// and there it will resolve, if return > 1, we did and return
// else we exec following code

        if let Some(fd) = fd_lookup.get(&fd) {
            mem.clone_from_slice(&fd[fd.len() - poc_mem.len()..])
        } // as we may try constant FD at fuzzing, not yet added to queue ?
        else { // should be empty or invalid FD !! 
            return Err(format!("--> FD {fd:?} NOT FOUND at table\n{fd_lookup:?}"))
        } // keep for now debug print, later we will kick it off once we debuged it enough :)
        Ok(0) // no any of dump memory was used
    }
    fn serialize(&self, _: &[u8], _: &[u8], _: &[u8]) -> Vec<SerializationInfo> {
        panic!("RndFd must be scoped whitin FdHolder argument!");
    }
}
