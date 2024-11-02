//@ build-pass
//@ compile-flags:--crate-name=test
//@ aux-build:coherence_lib.rs

extern crate coherence_lib as lib;
use lib::*;
use std::rc::Rc;

struct Local;

// UNCHAINED_TODO: This is currently allowed, but maybe it's too much
impl<T> Remote2<Box<T>, Local> for u32 {}

// UNCHAINED_TODO: This is currently allowed, but maybe it's too much
impl<'a, T> Remote2<&'a T, Local> for u32 {}

fn main() {}
