use std::mem;

extern crate core;
use self::core::generator::arg::Arg;
use self::core::generator::leaf::IArgLeaf;
use self::core::generator::serialize::ISerializableArg;
use self::core::generator::serialize::SerializationInfo;

use std::cell::RefCell;

extern crate generic;

/// lots of arguments for (sys)calls include pointers to another structures
/// Note : it is difference between call and arg
///     in call we use MemoryArg+PrimitiveArg containers
///     in arg we use Leaf+Composite wrappers, and there to include Pointer we need to wrap
///         MemoryArg Container, thus we need this Ptr Leaf do the job!
pub struct Ptr {
    arg: RefCell<Box<Arg>>,
}

impl Ptr {
    pub fn new(leaf: Box<dyn IArgLeaf>) -> Ptr {
        Ptr {
            arg: RefCell::new(Box::new(Arg::memory_arg(leaf))),
        }
    }
}

/// special notation for POC, mainly because creating runtime memory pointers!
impl ISerializableArg for Ptr {
    fn serialize(&self, _: &[u8], fd: &[u8]) -> Vec<SerializationInfo> {
        vec![SerializationInfo {
            offset: 0,
            prefix: String::from("ArgPtr(") + &self.arg.borrow().do_serialize(fd) + ", ",
        }]
    }
}

impl IArgLeaf for Ptr {
    fn size(&self) -> usize {
        mem::size_of::<usize>()
    }

    fn name(&self) -> &'static str {
        "Ptr"
    }

    fn generate_unsafe(&mut self, mem: &mut [u8], fd: &[u8]) {
        *generic::data_mut_unsafe::<*const u8>(mem) = self.arg.borrow_mut().do_generate(fd).data_const_unsafe();
    }
}
