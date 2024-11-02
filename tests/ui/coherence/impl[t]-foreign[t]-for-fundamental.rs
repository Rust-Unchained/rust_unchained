//@ build-pass
//@ compile-flags:--crate-name=test
//@ aux-build:coherence_lib.rs

extern crate coherence_lib as lib;
use lib::*;
use std::rc::Rc;

struct Local;

// UNCHAINED_TODO: This is currently allowed, but maybe it's too much
impl<T> Remote1<T> for Box<T> {}

// UNCHAINED_TODO: This is currently allowed, but maybe it's too much
impl<'a, A, B> Remote1<A> for &'a B {}

fn main() {}
