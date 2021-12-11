use std::mem;
use std::ops::BitAnd;
use std::ops::BitOr;

extern crate rand;
use rand::Rng;
use rand::distributions::{Standard, Distribution};

extern crate core;
use self::core::generator::leaf::IArgLeaf;
use self::core::generator::serialize::ISerializableArg;

extern crate generic;

/// arg generator for flag ( bitwise info inside integer )
pub struct Flag<T> {
    /// some flags need to be turned on everytime
    always: T,
    /// but most of them are volatile
    flag: T,
}

impl<T> Flag<T> {
    pub fn new(always: T, flag: T) -> Flag<T> {
        Flag {
            always : always,
            flag : flag,
        }
    }
}

impl<T> ISerializableArg for Flag<T> { }

impl<T: Copy + BitAnd + BitOr> IArgLeaf for Flag<T>
    where T: From< <T as BitAnd>::Output >,
          T: From< <T as BitOr>::Output >,
          Standard:Distribution<T>
{
    fn size(&self) -> usize { mem::size_of::<T>() }

    fn name(&self) -> &'static str { "Flag" }

    /// we do 6:1 generation based on defiition
    ///
    /// and 1:6 we provide random numero
    fn generate_unsafe(&mut self, mem: &mut[u8], _: &[u8]) {
        *generic::data_mut_unsafe::<T>(mem) = T::from(
            self.always | T::from(
                rand::thread_rng().gen::<T>() & self.flag));
        if rand::thread_rng().gen_bool(1./6.) {
            *generic::data_mut_unsafe::<T>(mem) = rand::thread_rng().gen::<T>();
        }
    }
}
