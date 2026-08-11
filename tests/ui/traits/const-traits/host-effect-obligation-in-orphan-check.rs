//! Regression test for #149703.
//!
//! Orphan checking used to ICE when proving a const `FnOnce` host-effect obligation for an
//! associated type projection.

#![feature(const_trait_impl)]

trait Z {
    type Assoc;
}

struct A;

impl<T: const FnOnce()> Z for T {
    type Assoc = ();
}

impl<T> From<<A as Z>::Assoc> for T {}
//~^ ERROR not all trait items implemented
//~| ERROR expected an `FnOnce()` closure, found `A`

fn main() {}
