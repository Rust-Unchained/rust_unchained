//@ build-pass
//@ compile-flags:--crate-name=test
//@ aux-build:coherence_lib.rs

extern crate coherence_lib as lib;
use lib::*;
use std::rc::Rc;

struct Local;

// Disallowed in standard Rust, okay in Unchained
impl<T> Remote1<Local> for Box<T> {}

// Disallowed in standard Rust, okay in Unchained
impl<T> Remote1<Local> for &T {}

fn main() {}
