use std::ops::{
    RangeInclusive,
};

extern crate core;

use self::core::generator::composite::ArgComposite;

use super::const_leaf::Const;
use super::bounded_leaf::Bounded;
use super::pattern_leaf::Pattern;
use super::tuple_leaf::TupleComposite;
use super::array_comp::ArrayComposite;

/// typical argument, string, you can provide prefix and count in element wise type of string
/// character, windows typical stuff
pub trait StrLeaf {
	fn astr_leaf(prefix: &str, count: usize) -> ArgComposite;
	fn astr_leaf_nz(prefix: &str, bounds: RangeInclusive<u8>, count: usize) -> ArgComposite;
	fn astr_leaf_gen_nz(bounds: RangeInclusive<u8>, count: usize) -> ArgComposite;
	// fn astr_leaf_const_nz(prefix: &str) -> ArgComposite;
	fn wstr_leaf(prefix: &str, count: usize) -> ArgComposite;
	fn wstr_leaf_nz(prefix: &str, count: usize) -> ArgComposite;
	fn wstr_leaf_const(prefix: &str) -> ArgComposite;
}
impl StrLeaf for ArgComposite {
    /// final length of the string is +1 as we appending 0 ( C-like strings )
	fn astr_leaf_gen_nz(bounds: RangeInclusive<u8>, count: usize) -> ArgComposite {
		ArgComposite::new(
			count,
			"AStrLeaf_of_",
			vec![
				(0, Box::new(ArgComposite::array_leaf(
                        "char",
                        count,
                        || { Box::new(Bounded::one(bounds.clone())) }))),
			])
	}
	fn astr_leaf_nz(prefix: &str, bounds: RangeInclusive<u8>, count: usize) -> ArgComposite {
        assert!(0 != count);
        if 0 == count {
            panic!("we need to append at least 0 to c-string!");
        }
		ArgComposite::new(
			prefix.len() + count,
			"AStrLeaf_of_",
			vec![
				(0, Box::new(Const::new(prefix))),
				(prefix.len(), Box::new(ArgComposite::array_leaf(
                        "char",
                        count,
                        || { Box::new(Bounded::one(bounds.clone())) }))),
			])
	}
	fn astr_leaf(prefix: &str, count: usize) -> ArgComposite {
        assert!(0 != count);
        if 0 == count {
            panic!("we need to append at least 0 to c-string!");
        }
		ArgComposite::new(
			prefix.len() + count + 1,
			"AStrLeaf_of_",
			vec![
				(0, Box::new(Const::new(prefix))),
				(prefix.len(), Box::new(ArgComposite::array_leaf(
                        "char",
                        count,
                        || { Box::new(Bounded::one(b'a'..=b'c')) }))),
				(prefix.len() + count, Box::new(Pattern::new(0, 1))),
			])
	}
	// fn astr_leaf_const_nz(prefix: &str) -> ArgComposite {
	//     ArgComposite::new(
	//         prefix.len(),
	//         "AStrLeaf_of_",
	//         vec![
    //         //what about using .ZIP instead of .FOLD ?
	//             (0, Box::new(Const::new(prefix.chars()
    //                 .fold(Vec::new(), |mut b, c| {
    //                     b.extend_from_slice(&[c as u8, 0u8]);
    //                     b } )))),
	//         ])
	// }
    /// final length of the string is +2 as we appending 0 ( C-like strings )
	fn wstr_leaf(prefix: &str, count: usize) -> ArgComposite {
        assert!(0 != count);
        if 0 == count {
            panic!("we need to append at least 0 to c-string!");
        }
		ArgComposite::new(
			2 * (prefix.len() + count + 1),
			"WStrLeaf_of_",
			vec![
            //what about using .ZIP instead of .FOLD ?
				(0, Box::new(Const::new(prefix.chars()
                    .fold(Vec::new(), |mut b, c| {
                        b.extend_from_slice(&[c as u8, 0u8]);
                        b } )))),
				(2 * prefix.len(), Box::new(ArgComposite::array_leaf(
                        "wcomp",
                        count,
                        || { Box::new(ArgComposite::tuple_leaf(
                                "wchar_t",
                                Box::new(Bounded::one(b'a'..=b'c')),
                                Box::new(Const::new8(0))))
                        }))),
				(2 * (prefix.len() + count), Box::new(Pattern::new(0, 2))),
			])
	}
    /// final length of the string is +2 as we appending 0 ( C-like strings )
	fn wstr_leaf_const(prefix: &str) -> ArgComposite {
		ArgComposite::new(
			2 * (prefix.len() + 1),
			"WStrLeaf_of_",
			vec![
            //what about using .ZIP instead of .FOLD ?
				(0, Box::new(Const::new(prefix.chars()
                    .fold(Vec::new(), |mut b, c| {
                        b.extend_from_slice(&[c as u8, 0u8]);
                        b } )))),
				(2 * (prefix.len()), Box::new(Pattern::new(0, 2))),
			])
	}
	fn wstr_leaf_nz(prefix: &str, count: usize) -> ArgComposite {
        assert!(0 != count);
        if 0 == count {
            panic!("we need to append at least 0 to c-string!");
        }
		ArgComposite::new(
			2 * (prefix.len() + count),
			"WStrLeaf_of_",
			vec![
            //what about using .ZIP instead of .FOLD ?
				(0, Box::new(Const::new(prefix.chars()
                    .fold(Vec::new(), |mut b, c| {
                        b.extend_from_slice(&[c as u8, 0u8]);
                        b } )))),
				(2 * prefix.len(), Box::new(ArgComposite::array_leaf(
                        "wcomp",
                        count,
                        || { Box::new(ArgComposite::tuple_leaf(
                                "wchar_t",
                                Box::new(Bounded::one(b'a'..=b'c')),
                                Box::new(Const::new8(0))))
                        }))),
			])
	}
}
