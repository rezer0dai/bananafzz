use std::collections::HashMap;

extern crate core;
use self::core::banana::bananaq::FuzzyQ;
use self::core::generator::leaf::IArgLeaf;
use self::core::generator::serialize::ISerializableArg;
use super::bfl_leaf::Bfl;
use std::sync::Weak;

pub struct Shared {
    offset: usize,
    size: usize,
}

impl Shared {
    //LibAFL should not touch shared data
    // in fact shared data are affected by calls, so LibAFL touching calls
    // will affect Shared, and that it is only way how it should affect it!!
    pub fn new(size: usize) -> Bfl<Shared> {
        Bfl::new(Shared {
            offset: 0,
            size: size,
        })
    }
    pub fn partial(offset: usize, size: usize) -> Bfl<Shared> {
        Bfl::new(Shared {
            offset: offset,
            size: size,
        })
    }
}

impl ISerializableArg for Shared {}

impl IArgLeaf for Shared {
    fn size(&self) -> usize {
        self.size
    }

    fn name(&self) -> &'static str {
        "Shared"
    }
    //reading shared state
    fn generate_unsafe(&mut self, _: &Weak<FuzzyQ>, mem: &mut [u8], _: &[u8], shared: &mut[u8]) -> bool {
        mem.clone_from_slice(&shared[self.offset..][..self.size]);
        true
    }
    fn save_shared(&mut self, mem: &[u8], shared: &mut [u8]) {
        shared[self.offset..][..self.size].clone_from_slice(mem);
    }
}

pub struct SharedWrite {
    arg: Box<dyn IArgLeaf>,
    offset: usize,
}

impl SharedWrite {
    pub fn new(arg: Box<dyn IArgLeaf>) -> SharedWrite {
        SharedWrite {
            arg: arg,
            offset: 0,
        }
    }
    pub fn partial(offset: usize, arg: Box<dyn IArgLeaf>) -> SharedWrite {
        SharedWrite {
            arg: arg,
            offset: offset,
        }
    }
}

impl ISerializableArg for SharedWrite {
    fn mem(&self, mem: &[u8]) -> Vec<u8> {
        self.arg.mem(mem)
    }
    fn dump(&self, mem: &[u8]) -> Vec<u8> { 
        self.arg.dump(mem) 
    }
    fn load(
        &mut self,
        mem: &mut [u8],
        dump: &[u8],
        poc_fd: &[u8],
        prefix: &[u8], 
        fd_lookup: &HashMap<Vec<u8>, Vec<u8>>,
    ) -> Result<usize, String> {
        self.arg.load(mem, dump, poc_fd, prefix, fd_lookup)
    }
}

impl IArgLeaf for SharedWrite {
    fn size(&self) -> usize {
        self.arg.size()
    }

    fn name(&self) -> &'static str {
        self.arg.name()
    }
    //reading shared state
    fn generate_unsafe(&mut self, bananaq: &Weak<FuzzyQ>, mem: &mut [u8], fd: &[u8], shared: &mut[u8]) -> bool {
        if !self.arg.generate_unsafe(bananaq, mem, fd, shared) {
            return false
        }
        self.save_shared(mem, shared);
        true
    }
    fn save_shared(&mut self, mem: &[u8], shared: &mut [u8]) {
        shared[self.offset..][..self.arg.size()].clone_from_slice(mem);
    }
}
