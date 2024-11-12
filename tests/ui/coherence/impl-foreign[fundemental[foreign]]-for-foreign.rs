//@ build-pass
//@ compile-flags:--crate-name=test
//@ aux-build:coherence_lib.rs

extern crate coherence_lib as lib;
use lib::*;
use std::rc::Rc;

struct Local;
struct Local1<T>(Rc<T>);


// Allowed in Unchained Rust, no conflicts here.
impl Remote1<Box<String>> for i32 {}

impl Remote1<Box<Rc<i32>>> for f64 {}

impl<T> Remote1<Box<Rc<T>>> for f32 { }

fn main() {}
