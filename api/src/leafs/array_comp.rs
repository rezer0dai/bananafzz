extern crate core;
use self::core::generator::composite::ArgComposite;
use self::core::generator::leaf::IArgLeaf;

pub trait ArrayComposite {
    fn array_leaf<F>(name: &'static str, ecount: usize, leafgen: F) -> ArgComposite
    where
        F: Fn() -> Box<dyn IArgLeaf>;
}

/// PERFOMANCE WARNING : discouraged to use ArrayArgs with more than 10 elemets - slow down fuzzing
/// without some reasonable (?) gain ..
impl ArrayComposite for ArgComposite {
    fn array_leaf<F>(name: &'static str, ecount: usize, leafgen: F) -> ArgComposite
    where
        F: Fn() -> Box<dyn IArgLeaf>,
    {
        let mut leafs = Vec::new();
        let size = leafgen().size();
        for i in 0..ecount {
            leafs.push((i * size, leafgen()))
        }
        ArgComposite::new(size * ecount, name, leafs)
    }
}
