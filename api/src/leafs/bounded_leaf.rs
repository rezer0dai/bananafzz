use std::mem;
use std::ops::RangeInclusive;
use std::cmp::PartialOrd;
//use std::collections::HashMap;

extern crate rand;
use rand::Rng;
use rand::distributions::uniform::SampleUniform;
use rand::seq::SliceRandom;

extern crate core;
use self::core::generator::leaf::IArgLeaf;
use self::core::generator::serialize::ISerializableArg;

//use core::config::FZZCONFIG;

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

impl<T: Copy + PartialOrd + SampleUniform + std::fmt::Debug> ISerializableArg for Bounded<T>
{/*
    fn load(&mut self, mem: &mut[u8], dump: &[u8], data: &[u8], fd_lookup: &HashMap<Vec<u8>,Vec<u8>>) -> usize {
        let size = ISerializableArg::load(self, mem, dump, data, fd_lookup);
        if rand::thread_rng().gen_bool(1./FZZCONFIG.afl_fix_ratio) {
            return size
        }

        let afl_data = *generic::data_const_unsafe::<T>(mem);
        let afl_data = &afl_data;
        for bounds in self.bounds.iter() {
            if bounds.start() > afl_data || bounds.end() < afl_data {
                continue
            }
            return size
        }

        let seed = dump.iter().sum::<u8>() as usize;
        let bounds = &self.bounds[seed % self.bounds.len()];
        *generic::data_mut_unsafe::<T>(mem) = if bounds.start() > afl_data {
                *bounds.start()
            } else {
                *bounds.end()
            };

        size
    }*/
}

impl<T: Copy + PartialOrd + SampleUniform + std::fmt::Debug> IArgLeaf for Bounded<T>
{
    fn size(&self) -> usize { mem::size_of::<T>() }

    fn name(&self) -> &'static str { "Bounded" }

    fn generate_unsafe(&mut self, mem: &mut[u8], _: &[u8], _: &[u8]) {
        *generic::data_mut_unsafe::<T>(mem) = match self.bounds.clone().choose(&mut rand::thread_rng()) {
            Some(bounds) => rand::thread_rng().gen_range(bounds.clone()),
            None => panic!("nothing in bound array ?"),
        };
    }

}
