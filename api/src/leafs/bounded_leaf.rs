use std::mem;
use std::ops::RangeInclusive;
use std::cmp::PartialOrd;

extern crate rand;
use rand::Rng;
use rand::distributions::uniform::SampleUniform;
use rand::seq::SliceRandom;

extern crate core;
use self::core::generator::leaf::IArgLeaf;
use self::core::generator::serialize::ISerializableArg;

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

impl<T> ISerializableArg for Bounded<T> { }

impl<T: Copy + PartialOrd + SampleUniform + std::fmt::Debug> IArgLeaf for Bounded<T>
{
    fn size(&self) -> usize { mem::size_of::<T>() }

    fn name(&self) -> &'static str { "Bounded" }

    fn generate_unsafe(&mut self, mem: &mut[u8], _: &[u8]) {
        *generic::data_mut_unsafe::<T>(mem) = match self.bounds.clone().choose(&mut rand::thread_rng()) {
            Some(bounds) => rand::thread_rng().gen_range(bounds.clone()),
            None => panic!("nothing in bound array ?"),
        };
    }
}
