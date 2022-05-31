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
use self::core::banana::bananaq::{self, FuzzyQ};
use self::core::generator::leaf::IArgLeaf;
use self::core::generator::serialize::ISerializableArg;

pub struct Bounded<T> {
    bounds: Vec< RangeInclusive<T> >,
    afl_fix_ratio: f64,
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
            afl_fix_ratio: -1.0,
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
            afl_fix_ratio: -1.0,
        }
    }
}

impl<T: Copy + PartialOrd + SampleUniform + Debug + Add<Output = T> + Sub<Output = T> + Rem<Output = T>> ISerializableArg for Bounded<T>
{
    fn load(&mut self, mem: &mut[u8], dump: &[u8], data: &[u8], _fd_lookup: &HashMap<Vec<u8>,Vec<u8>>) -> Result<usize, String> {
        let size = self.default_load(mem, dump, data);
        if !rand::thread_rng().gen_bool(self.afl_fix_ratio) {
            return Ok(size)
        }

        let afl_val: &T = generic::data_const_unsafe(mem);
        for bounds in self.bounds.iter() {
            if bounds.contains(afl_val) {
                return Ok(size)
            }
        }

        let seed = dump.iter().sum::<u8>() as usize;
        let bounds = &self.bounds[seed % self.bounds.len()];

        *generic::data_mut_unsafe::<T>(mem) = 
            *bounds.start() + (*afl_val % (*bounds.end() - *bounds.start()));

        Ok(size)
    }
}

impl<T: Copy + PartialOrd + SampleUniform + Debug + Add<Output = T> + Sub<Output = T> + Rem<Output = T>> IArgLeaf for Bounded<T>
{
    fn size(&self) -> usize { mem::size_of::<T>() }

    fn name(&self) -> &'static str { "Bounded" }

    fn generate_unsafe(&mut self, bananaq: &Weak<FuzzyQ>, mem: &mut[u8], _: &[u8], _: &mut[u8]) {
        if self.afl_fix_ratio < 0.0 {
            if let Ok(config) = bananaq::config(bananaq) {
                self.afl_fix_ratio = config.afl_fix_ratio
            }
        }
        *generic::data_mut_unsafe::<T>(mem) = match self.bounds.clone().choose(&mut rand::thread_rng()) {
            Some(bounds) => rand::thread_rng().gen_range(bounds.clone()),
            None => panic!("nothing in bound array ?"),
        };
    }

}

pub struct BoundedOverX<T> {
    x: u64,
    bounded: Bounded<T>,
}
impl<T> BoundedOverX<T>
{
    pub fn one<B>(x: u64, bounds: B) -> Self
        where B: Into< RangeInclusive<T> >, T: Clone
    {
        BoundedOverX { 
            x: x,
            bounded : Bounded::one(bounds)
        }
    }
    pub fn ranges<B>(x: u64, bounds: Vec<B>) -> Self
        where B: Into< RangeInclusive<T> >, T: Clone
    {
        BoundedOverX { 
            x: x,
            bounded : Bounded::ranges(bounds)
        }
    }
}
impl<T: Copy + PartialOrd + SampleUniform + Debug + Add<Output = T> + Sub<Output = T> + Rem<Output = T> + From<u64> + Into<u64>> IArgLeaf for BoundedOverX<T>
{
    fn size(&self) -> usize { mem::size_of::<T>() }

    fn name(&self) -> &'static str { "BoundedOverX" }

    fn generate_unsafe(&mut self, bananaq: &Weak<FuzzyQ>, mem: &mut[u8], fd: &[u8], shared: &mut[u8]) {
        self.bounded.generate_unsafe(bananaq, mem, fd, shared);
        let x = self.x.pow((*generic::data_const_unsafe::<T>(mem)).into() as u32);
        *generic::data_mut_unsafe::<T>(mem) = x.into();
    }
}
// bfl should learn fast that it does not make sense to modify
// or it finds some modifier qute effecgtive ( +-1 )
impl<T> ISerializableArg for BoundedOverX<T> {}
