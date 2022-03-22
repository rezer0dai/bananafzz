use std::sync::Weak;

extern crate byteorder;
use self::byteorder::{LittleEndian, WriteBytesExt};

extern crate core;
use self::core::banana::bananaq::FuzzyQ;
use self::core::generator::leaf::IArgLeaf;
use self::core::generator::serialize::ISerializableArg;

use super::bfl_leaf::Bfl;

/// arg generator for constant values ( primitive types u8..u64, up to arrays of u8 )
pub struct Const {
    data: Vec<u8>
}

impl Const {
    pub fn new<T: Into<Vec<u8>>>(data: T) -> Bfl::<Const> {
        Bfl::new(Const {
            data : data.into(),
        })
    }
    pub fn new8(primitive: u8) -> Bfl::<Const> {
        let data = vec![primitive];
        Const::new(data)
    }
    pub fn new16(primitive: u16) -> Bfl::<Const> {
        let mut data = vec![];
        data.write_u16::<LittleEndian>(primitive).unwrap();
        Const::new(data)
    }
    pub fn new32(primitive: u32) -> Bfl::<Const> {
        let mut data = vec![];
        data.write_u32::<LittleEndian>(primitive).unwrap();
        Const::new(data)
    }
    pub fn new64(primitive: u64) -> Bfl::<Const> {
        let mut data = vec![];
        data.write_u64::<LittleEndian>(primitive).unwrap();
        Const::new(data)
    }
}

impl ISerializableArg for Const { }

impl IArgLeaf for Const {
    fn size(&self) -> usize { self.data.len() }

    fn name(&self) -> &'static str { "Const" }

    fn generate_unsafe(&mut self, _: &Weak<FuzzyQ>, mem: &mut[u8], _: &[u8], _: &[u8]) {
        mem.clone_from_slice(&self.data);
        /*
          .clone()
          .into_iter()
          .rev()
          .collect::<Vec<u8>>()
          .as_slice()); // LITTLE ENDIAN
        */
    }
}
