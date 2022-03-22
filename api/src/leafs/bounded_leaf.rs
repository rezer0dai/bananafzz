use std::mem;
use std::sync::Weak;
use std::fmt::Debug;
use std::ops::{RangeInclusive, Sub, Add, Rem};
use std::cmp::PartialOrd;
use std::collections::HashMap;

extern crate rand;
use rand::Rng;
use rand::distributions::uniform::SampleUniform;
use rand::seq::SliceRandom;

extern crate core;
use self::core::banana::bananaq::FuzzyQ;
use self::core::generator::leaf::IArgLeaf;
use self::core::generator::serialize::ISerializableArg;

use core::config::FZZCONFIG;

/// arg generator for bounded values - ranges ( 1..22, 0..1, 66..888, ..)
pub struct Bounded<T> {
    bounds: Vec< RangeInclusive<T> >,
}

impl<T> Bounded<T>
{
    /// wrapper for using simple range, however for now bit verbose Range{ start: .., end: .. }
    pub fn one<B>(bounds: B) -> Bounded<T>
        where B: Into< RangeInclusive<T> >, T: Clone
    {
        let bounds = bounds.into();
        Bounded {
            bounds : vec![bounds],
        }
    }
    pub fn ranges<B>(bounds: Vec<B>) -> Bounded<T>
        where B: Into< RangeInclusive<T> >, T: Clone
    {
        Bounded {
            bounds : bounds
                .into_iter()
                .map(|val| {
                    let bounds = val.into();
                    bounds
                })
                .collect(),
        }
    }
}

impl<T: Copy + PartialOrd + SampleUniform + Debug + Add<Output = T> + Sub<Output = T> + Rem<Output = T>> ISerializableArg for Bounded<T>
{
    fn load(&mut self, mem: &mut[u8], dump: &[u8], data: &[u8], _fd_lookup: &HashMap<Vec<u8>,Vec<u8>>) -> usize {
        let size = self.default_load(mem, dump, data);
        if !rand::thread_rng().gen_bool(FZZCONFIG.afl_fix_ratio) {
            return size
        }

        let afl_val: &T = generic::data_const_unsafe(mem);
        for bounds in self.bounds.iter() {
            if bounds.contains(afl_val) {
                return size
            }
        }

        let seed = dump.iter().sum::<u8>() as usize;
        let bounds = &self.bounds[seed % self.bounds.len()];

        *generic::data_mut_unsafe::<T>(mem) = 
            *bounds.start() + (*afl_val % (*bounds.end() - *bounds.start()));

        size
    }
}

impl<T: Copy + PartialOrd + SampleUniform + Debug + Add<Output = T> + Sub<Output = T> + Rem<Output = T>> IArgLeaf for Bounded<T>
{
    fn size(&self) -> usize { mem::size_of::<T>() }

    fn name(&self) -> &'static str { "Bounded" }

    fn generate_unsafe(&mut self, _: &Weak<FuzzyQ>, mem: &mut[u8], _: &[u8], _: &[u8]) {
        *generic::data_mut_unsafe::<T>(mem) = match self.bounds.clone().choose(&mut rand::thread_rng()) {
            Some(bounds) => rand::thread_rng().gen_range(bounds.clone()),
            None => panic!("nothing in bound array ?"),
        };
    }

}
