use std::mem;
use std::ops::BitAnd;
use std::ops::BitOr;
use std::sync::Weak;
use std::collections::HashMap;

extern crate rand;
use rand::Rng;
use rand::distributions::{Standard, Distribution};

extern crate core;
use self::core::banana::bananaq::FuzzyQ;
use self::core::generator::leaf::IArgLeaf;
use self::core::generator::serialize::ISerializableArg;

use core::config::FZZCONFIG;

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

impl<T: Copy + BitAnd + BitOr> ISerializableArg for Flag<T>
    where T: From< <T as BitAnd>::Output >,
          T: From< <T as BitOr>::Output >
{
    fn load(&mut self, mem: &mut[u8], dump: &[u8], data: &[u8], _fd_lookup: &HashMap<Vec<u8>,Vec<u8>>) -> usize {
        let size = self.default_load(mem, dump, data);
        if !rand::thread_rng().gen_bool(FZZCONFIG.afl_fix_ratio) {
            return size
        }
        let afl_data = *generic::data_mut_unsafe::<T>(mem);
        *generic::data_mut_unsafe::<T>(mem) = T::from(
            self.always | T::from(
                afl_data & self.flag));
        size
    }
}

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
    fn generate_unsafe(&mut self, _: &Weak<FuzzyQ>, mem: &mut[u8], _: &[u8], _: &[u8]) {
        *generic::data_mut_unsafe::<T>(mem) = T::from(
            self.always | T::from(
                rand::thread_rng().gen::<T>() & self.flag));
        //with BFL for code cov, we should kick this out
        //BFL with afl_fix_ratio<1. will do this
        //and in readl fuzzing for races we dont want to do this
        //otherwise we could this intro per type
        //like once a time do totaly random ...
        if rand::thread_rng().gen_bool(1./6.) {//based on config ??
            *generic::data_mut_unsafe::<T>(mem) = rand::thread_rng().gen::<T>();
        }
    }
}
