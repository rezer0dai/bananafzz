extern crate core;
use self::core::generator::leaf::IArgLeaf;
use self::core::generator::serialize::ISerializableArg;

use super::bfl_leaf::Bfl;

/// arg generator for patterned data ( nullptr, or others )
pub struct Pattern {
    pattern: u8,
    size: usize
}

impl Pattern {
    pub fn new(pattern: u8, size: usize) -> Bfl::<Pattern> {
        Bfl::new(Pattern {
            pattern : pattern,
            size : size,
        })
    }
}

impl ISerializableArg for Pattern { }

impl IArgLeaf for Pattern {
    fn size(&self) -> usize { self.size }

    fn name(&self) -> &'static str { "Pattern" }

    fn generate_unsafe(&mut self, mem: &mut[u8], _: &[u8], _: &[u8]) {
      mem.fill(self.pattern)
    }
}
