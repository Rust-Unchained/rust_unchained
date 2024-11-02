//@ build-pass
//@ compile-flags:--crate-name=test
//@ aux-build:coherence_lib.rs

extern crate coherence_lib as lib;
use lib::*;
use std::rc::Rc;
use std::sync::Arc;

struct Local;

// Not allowed in standard Rust, but okay in unchained.
impl Remote for Rc<Local> {}

// UNCHAINED_TODO: This is currently allowed, but maybe it's too much
impl<T> Remote for Arc<T> {}

fn main() {}
