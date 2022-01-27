extern crate core;
use self::core::generator::leaf::IArgLeaf;
use self::core::generator::serialize::ISerializableArg;

pub struct Shared {
    offset: usize,
    size: usize,
}

impl Shared {
    pub fn new(offset: usize, size: usize) -> Shared {
        Shared {
            offset : offset,
            size : size,
        }
    }
}

use std::collections::HashMap;

impl ISerializableArg for Shared { 
    fn load(&mut self, mem: &mut[u8], dump: &[u8], _data: &[u8], _fd_lookup: &HashMap<Vec<u8>,Vec<u8>>) -> usize {
println!("SHARED LOAD : {} vs {} :: {}", mem.len(), self.size(), _data.len());
assert!(mem.len() == self.size);
        mem.copy_from_slice(&dump[..mem.len()]);
        mem.len()
    }
}

impl IArgLeaf for Shared {
    fn size(&self) -> usize { self.size }

    fn name(&self) -> &'static str { "Shared" }
//reading shared state
    fn generate_unsafe(&mut self, mem: &mut[u8], _: &[u8], shared: &[u8]) { 
        mem.clone_from_slice(&shared[self.offset..][..self.size]);
    }// in case that our call should modify this and other use it, then is best
    // to do it trough proxy at target, aka dllexport, and this as additional argument
    // like open(..) has 2 arguments, then export open_(a1, a2, a3) { open(a1, a2); memcpy(a3, a2) }
    // could do also special argument f.e. SharedWriter, but above seems more clean
//saving shared state
    fn save_shared(&mut self, mem: &[u8], shared: &mut[u8]) { 
        shared[self.offset..][..self.size].clone_from_slice(mem);
    }
}
