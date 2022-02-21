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

impl ISerializableArg for Shared { }

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
