use std::collections::HashMap;

extern crate core;
use self::core::generator::leaf::IArgLeaf;
use self::core::generator::serialize::ISerializableArg;
use self::core::generator::serialize::SerializationInfo;

use self::core::banana::bananaq::FuzzyQ;
use std::sync::Weak;

pub struct StateLink<F> {
    arg: Box<dyn IArgLeaf>,
    approve: F,
}
impl<F> StateLink<F>
where 
    F: Fn(&[u8], &[u8], &[u8]) -> bool
{
    pub fn new(arg: Box<dyn IArgLeaf>, approve: F) -> Self {
        Self {
            arg: arg,
            approve: approve,
        }
    }
}
impl<F> ISerializableArg for StateLink<F> {
    fn serialize(&self, mem: &[u8], fd: &[u8], shared: &[u8]) -> Vec<SerializationInfo> {
        self.arg.serialize(mem, fd, shared)
    }
    fn dump(&self, mem: &[u8]) -> Vec<u8> { self.arg.dump(mem) }
    fn load(
        &mut self,
        mem: &mut [u8],
        dump: &[u8],
        poc_fd: &[u8],
        fd_lookup: &HashMap<Vec<u8>, Vec<u8>>,
    ) -> Result<usize, String> {
        self.arg.load(mem, dump, poc_fd, fd_lookup)
    }
}
impl<F> IArgLeaf for StateLink<F>
where 
    F: Fn(&[u8], &[u8], &[u8]) -> bool
{
    fn size(&self) -> usize {
        self.arg.size()
    }

    fn name(&self) -> &'static str {
        self.arg.name()
    }

    fn generate_conditioned(
        &mut self,
        bananaq: &Weak<FuzzyQ>,
        mem: &mut [u8],
        fd: &[u8],
        shared: &mut[u8],
    ) -> bool {
        if !self.arg.generate_unsafe(bananaq, mem, fd, shared) {
            return false
        }
        (self.approve)(mem, fd, shared)
    }
    fn save_shared(&mut self, mem: &[u8], shared: &mut [u8]) {
        self.arg.save_shared(mem, shared)
    }
}
