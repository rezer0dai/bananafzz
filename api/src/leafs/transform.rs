use std::collections::HashMap;

extern crate core;
use self::core::generator::leaf::IArgLeaf;
use self::core::generator::serialize::{ISerializableArg,SerializationInfo};

use self::core::banana::bananaq::FuzzyQ;
use std::sync::Weak;

pub struct Transform<F> {
    arg: Box<dyn IArgLeaf>,
    transform: F,
    mem: Vec<u8>,
}
impl<F> Transform<F>
where 
    F: Fn(&mut [u8])
{
    pub fn new(arg: Box<dyn IArgLeaf>, transform: F) -> Self {
        let size = arg.size();
        Self { arg, transform, mem: vec![0u8; size] }
    }
}

impl<F> ISerializableArg for Transform<F>
where 
    F: Fn(&mut [u8])
{
    fn serialize(&self, mem: &[u8], fd: &[u8], shared: &[u8]) -> Vec<SerializationInfo> {
        self.arg.serialize(mem, fd, shared)
    }
    fn mem(&self, _mem: &[u8]) -> Vec<u8> {
        self.mem.clone()
    }
    fn dump(&self, mem: &[u8]) -> Vec<u8> {
        self.arg.dump(mem)
    }
        
    // we need to de-transform, at very least for fd-arg
    fn load(&mut self, mem: &mut[u8], dump: &[u8], data: &[u8], prefix: &[u8], fd_lookup: &HashMap<Vec<u8>,Vec<u8>>) -> Result<usize, String> {
        let out = self.arg.load(mem, dump, data, prefix, fd_lookup);
        (self.transform)(mem);
        out
    }
}

impl<F> IArgLeaf for Transform<F>
where 
    F: Fn(&mut [u8])
{
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
        self.mem.clone_from_slice(mem); // save data for repro
        (self.transform)(mem);
        true
    }
}
