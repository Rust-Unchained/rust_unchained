#![allow(incomplete_features)]
#![feature(const_trait_impl, const_try, try_trait_v2)]

use std::ops::FromResidual;

impl<T> const FromResidual for T { //~ E0119
    fn from_residual(t: T) -> _ {
        //~^ the placeholder `_` is not allowed
        t
    }
}

fn main() {}
