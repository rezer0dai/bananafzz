use std::collections::HashMap;

extern crate core;
use self::core::banana::bananaq::FuzzyQ;
use std::sync::Weak;
use self::core::generator::leaf::IArgLeaf;
use self::core::generator::serialize::ISerializableArg;
use self::core::generator::serialize::SerializationInfo;

extern crate generic;

/// arguments which implements part of state or knowledge
/// those we dont want AFL to fuzz/mutate
pub struct Bfl<T> {
    leaf: T,
}

impl<T> Bfl<T> {
    pub fn new(leaf: T) -> Bfl<T> {
        Bfl { leaf: leaf }
    }
}

impl<T> ISerializableArg for Bfl<T> 
    where T : IArgLeaf
{
    fn serialize(&self, mem: &[u8], fd: &[u8], shared: &[u8]) -> Vec<SerializationInfo> {
        self.leaf.serialize(mem, fd, shared)
    }
    fn dump(&self, _mem: &[u8]) -> Vec<u8> { vec![] }
    fn load(&mut self, mem: &mut[u8], _dump: &[u8], data: &[u8], _fd_lookup: &HashMap<Vec<u8>,Vec<u8>>) -> Result<usize, String> { 
        mem.clone_from_slice(data);
        Ok(0) 
    }
}

impl<T> IArgLeaf for Bfl<T>
    where T : IArgLeaf
{
    fn size(&self) -> usize {
        self.leaf.size()
    }

    fn name(&self) -> &'static str {
        self.leaf.name()
    }

    fn generate_unsafe(&mut self, bananaq: &Weak<FuzzyQ>, mem: &mut [u8], fd: &[u8], shared: &mut[u8]) -> bool {
        self.leaf.generate_unsafe(bananaq, mem, fd, shared)
    }
    fn save_shared(&mut self, mem: &[u8], shared: &mut[u8]) { 
        self.leaf.save_shared(mem, shared)
    }
}
