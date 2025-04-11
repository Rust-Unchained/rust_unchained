//@ build-pass
//@ compile-flags:--crate-name=test
//@ aux-build:coherence_lib.rs

extern crate coherence_lib as lib;
use lib::*;
use std::rc::Rc;

struct Local;

// Allowed in Unchained Rust, no conflicts here.
impl<T> Remote1<Box<T>> for u32 {
}

impl<'a, T> Remote1<&'a T> for u32 {}

fn main() {}
