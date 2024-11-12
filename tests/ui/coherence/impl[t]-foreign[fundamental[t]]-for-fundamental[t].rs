//@ build-pass
//@ compile-flags:--crate-name=test
//@ aux-build:coherence_lib.rs

extern crate coherence_lib as lib;
use lib::*;
use std::rc::Rc;

// Allowed in Unchained Rust, no conflicts here.
impl<'a, T> Remote1<Box<T>> for &'a T {}

impl<'a, T> Remote1<&'a T> for Box<T> {}

fn main() {}
