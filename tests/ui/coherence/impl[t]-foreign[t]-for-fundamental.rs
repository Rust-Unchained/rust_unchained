//@ build-pass
//@ compile-flags:--crate-name=test
//@ aux-build:coherence_lib.rs

extern crate coherence_lib as lib;
use lib::*;
use std::rc::Rc;

struct Local;

// Allowed in Unchained Rust, no conflicts here.
impl<T> Remote1<T> for Box<T> {}

impl<'a, A, B> Remote1<A> for &'a B {}

fn main() {}
