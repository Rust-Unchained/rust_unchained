//@ build-pass
//@ compile-flags:--crate-name=test
//@ aux-build:coherence_lib.rs

extern crate coherence_lib as lib;
use lib::*;
use std::rc::Rc;

struct Local;

// Allowed in Rust Unchained
impl Remote1<u32> for f64 {
}

fn main() {}
