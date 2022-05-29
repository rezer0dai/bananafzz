use std::mem;
use std::sync::Weak;
use std::collections::HashMap;

extern crate core;
use self::core::banana::bananaq::FuzzyQ;
use self::core::generator::arg::Arg;
use self::core::generator::leaf::IArgLeaf;
use self::core::generator::serialize::ISerializableArg;
use self::core::generator::serialize::SerializationInfo;

extern crate generic;

/// lots of arguments for (sys)calls include pointers to another structures
/// Note : it is difference between call and arg
///     in call we use MemoryArg+PrimitiveArg containers
///     in arg we use Leaf+Composite wrappers, and there to include Pointer we need to wrap
///         MemoryArg Container, thus we need this Ptr Leaf do the job!
pub struct Ptr {
    arg: Box<Arg>,
}

impl Ptr {
    pub fn new(leaf: Box<dyn IArgLeaf>) -> Ptr {
        Ptr {
            arg: Box::new(Arg::memory_arg(leaf)),
        }
    }
}

/// special notation for POC, mainly because creating runtime memory pointers!
impl ISerializableArg for Ptr {
    fn serialize(&self, _: &[u8], fd: &[u8], shared: &[u8]) -> Vec<SerializationInfo> {
        vec![SerializationInfo {
            offset: 0,
            prefix: String::from("ArgPtr(") + &self.arg.do_serialize(fd, shared) + ", ",
        }]
    }
    fn dump(&self, _mem: &[u8]) -> Vec<u8> {
        self.arg.dump()
    }
        
    fn load(&mut self, _mem: &mut[u8], dump: &[u8], data: &[u8], fd_lookup: &HashMap<Vec<u8>,Vec<u8>>) -> Result<usize, String> {
        self.arg.load(dump, data, fd_lookup)
    }
}

impl IArgLeaf for Ptr {
    fn size(&self) -> usize {
        mem::size_of::<usize>()
    }

    fn name(&self) -> &'static str {
        "Ptr"
    }

    fn generate_unsafe(&mut self, bananaq: &Weak<FuzzyQ>, mem: &mut [u8], fd: &[u8], shared: &mut[u8]) {
        *generic::data_mut_unsafe::<*const u8>(mem) = self.arg.do_generate(bananaq, fd, shared).data_const_unsafe();
    }
    fn save_shared(&mut self, _: &[u8], shared: &mut [u8]) {
        self.arg.do_save_shared(shared)
    }
}
