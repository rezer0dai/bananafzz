extern crate core;
use self::core::generator::composite::ArgComposite;
use self::core::generator::leaf::IArgLeaf;

/// basically wrapper for stacking together two arg generators to one
pub trait TupleComposite {
    fn tuple_leaf(
        name: &'static str,
        leafa: Box<dyn IArgLeaf>,
        leafb: Box<dyn IArgLeaf>
        ) -> ArgComposite;
}

impl TupleComposite for ArgComposite {
    fn tuple_leaf(
        name: &'static str,
        leafa: Box<dyn IArgLeaf>,
        leafb: Box<dyn IArgLeaf>
        ) -> ArgComposite
    {
        ArgComposite::new(
            leafa.size() + leafb.size(),
            name,
            vec![
                (leafa.size(), leafb),
                (0, leafa),
            ])
    }
}
