//@ build-pass
//@ compile-flags:--crate-name=test
//@ aux-build:coherence_lib.rs

extern crate coherence_lib as lib;
use lib::*;
use std::rc::Rc;

struct Local;

// Allowed in Unchained Rust, upstream doesn't provide any impls.
impl<T> Remote1<u32> for T {}

fn main() {}
