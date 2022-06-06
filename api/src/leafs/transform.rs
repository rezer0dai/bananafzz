extern crate core;
use self::core::generator::leaf::IArgLeaf;
use self::core::generator::serialize::ISerializableArg;

use self::core::banana::bananaq::FuzzyQ;
use std::sync::Weak;

pub struct Transform<F> {
    arg: Box<dyn IArgLeaf>,
    transform: F,
}
impl<F> Transform<F>
where 
    F: Fn(&mut [u8], &[u8], &[u8])
{
    pub fn new(arg: Box<dyn IArgLeaf>, transform: F) -> Self {
        Self { arg, transform }
    }
}

impl<F> ISerializableArg for Transform<F> { }

impl<F> IArgLeaf for Transform<F>
where 
    F: Fn(&mut [u8], &[u8], &[u8])
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
        (self.transform)(mem, fd, shared);
        true
    }
}
