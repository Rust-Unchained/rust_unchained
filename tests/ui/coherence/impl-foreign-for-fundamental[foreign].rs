//@ compile-flags:--crate-name=test
//@ aux-build:coherence_lib.rs

extern crate coherence_lib as lib;
use lib::*;
use std::rc::Rc;

struct Local;

impl Remote for Box<i32> {
    //~^ ERROR only traits defined in the current crate
    // | can be implemented for arbitrary types [E0117]
}

// UNCHAINED_TODO: This is currently allowed, but maybe it's too much
impl<T> Remote for Box<Rc<T>> {}

fn main() {}
