use std::sync::Weak;
use std::collections::HashMap;

extern crate core;
use self::core::banana::bananaq::FuzzyQ;
use self::core::generator::leaf::IArgLeaf;
use self::core::generator::serialize::ISerializableArg;
use self::core::generator::serialize::SerializationInfo;

use super::bfl_leaf::Bfl;

extern crate generic;

pub struct DeRef {
    size: usize,
    offset: usize,
}

impl DeRef {
    pub fn new(size: usize) -> Bfl<DeRef> {
        Bfl::new(DeRef {
            size: size,
            offset: 0,
        })
    }
    pub fn partial(offset: usize, size: usize) -> Bfl<DeRef> {
        Bfl::new(DeRef {
            size: size,
            offset: offset,
        })
    }
}

/// Backbone of whole state fuzzing
impl ISerializableArg for DeRef {
    fn serialize(&self, _: &[u8], fd: &[u8], _: &[u8]) -> Vec<SerializationInfo> {
        vec![SerializationInfo {
            offset: 0,
            prefix: String::from("state_fd(fd_")
                + &generic::u8_to_str(fd) + ", "
                + &self.offset.to_string() + ","
                + &self.size().to_string() + ",",
        }]
    }
}

impl IArgLeaf for DeRef {
    fn size(&self) -> usize {
        self.size
    }

    fn name(&self) -> &'static str {
        "Fd"
    }

    fn generate_unsafe(&mut self, _: &Weak<FuzzyQ>, mem: &mut [u8], fd: &[u8], _: &mut[u8]) {
        mem.copy_from_slice(&fd[self.offset..self.offset + self.size]);
    }
}
