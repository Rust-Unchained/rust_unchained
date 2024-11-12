//@ build-pass
//@ compile-flags:--crate-name=test
//@ aux-build:coherence_lib.rs

extern crate coherence_lib as lib;
use lib::*;
use std::rc::Rc;

struct Local;

// Allowed in Unchained Rust, no conflicts here.
impl Remote for Box<i32> {}

impl<T> Remote for Box<Rc<T>> {}

fn main() {}
