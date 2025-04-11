//@ build-pass
//@ aux-build:coherence_lib.rs


extern crate coherence_lib as lib;
use lib::{Remote1, Pair};

pub struct Local<T>(T);

// Disallowed in standard Rust, okay in Unchained since lib
// does not provide any implementations of Remote1
impl<T, U> Remote1<Pair<T, Local<U>>> for i32 { }


fn main() { }
