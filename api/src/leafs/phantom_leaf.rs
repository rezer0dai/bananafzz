use std::sync::Weak;

extern crate core;
use self::core::banana::bananaq::FuzzyQ;
use self::core::generator::leaf::IArgLeaf;
use self::core::generator::serialize::ISerializableArg;

/// sometime we dont want to touch data, it is just there, higher ups will handle other details
pub struct Phantom {
    size: usize
}

impl Phantom {
    pub fn new(size: usize) -> Phantom {
        Phantom {
            size : size,
        }
    }
}

impl ISerializableArg for Phantom { }

impl IArgLeaf for Phantom {
    fn size(&self) -> usize { self.size }

    fn name(&self) -> &'static str { "Phantom" }

    fn generate_unsafe(&mut self, _: &Weak<FuzzyQ>, _: &mut[u8], _: &[u8], _: &[u8]) { }
}
