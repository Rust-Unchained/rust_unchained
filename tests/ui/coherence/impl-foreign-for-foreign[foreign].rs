//@ build-pass
//@ compile-flags:--crate-name=test
//@ aux-build:coherence_lib.rs

extern crate coherence_lib as lib;
use lib::*;
use std::rc::Rc;

struct Local;

// Allowed in Unchained Rust, no conflicts here.
impl Remote1<Rc<i32>> for i32 { }

impl Remote1<Rc<Local>> for f64 {}

impl<T> Remote1<Rc<T>> for f32 {}

fn main() {}
