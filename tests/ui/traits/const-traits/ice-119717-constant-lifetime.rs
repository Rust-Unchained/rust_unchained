#![allow(incomplete_features)]
#![feature(const_trait_impl, effects, try_trait_v2)]

use std::ops::FromResidual;

impl<T> const FromResidual for T { //~ E0119
    //~^ ERROR const `impl` for trait `FromResidual` which is not marked with `#[const_trait]`
    fn from_residual(t: T) -> _ {
        //~^ the placeholder `_` is not allowed
        t
    }
}

fn main() {}
