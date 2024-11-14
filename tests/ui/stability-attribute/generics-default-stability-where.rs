//@ aux-build:unstable_generic_param.rs

extern crate unstable_generic_param;

use unstable_generic_param::*;

impl<T> Trait3<usize> for T where T: Trait2<usize> { //~ ERROR E0658
    fn foo() -> usize { T::foo() }
}

fn main() {}
