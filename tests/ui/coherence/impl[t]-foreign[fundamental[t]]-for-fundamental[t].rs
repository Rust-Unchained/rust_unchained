//@ build-pass
//@ compile-flags:--crate-name=test
//@ aux-build:coherence_lib.rs

extern crate coherence_lib as lib;
use lib::*;
use std::rc::Rc;

// UNCHAINED_TODO: This is currently allowed, but maybe it's too much
impl<'a, T> Remote1<Box<T>> for &'a T {}

// UNCHAINED_TODO: This is currently allowed, but maybe it's too much
impl<'a, T> Remote1<&'a T> for Box<T> {}

fn main() {}
