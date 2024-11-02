//@ build-pass
//@ aux-build:coherence_concrete_lib.rs

extern crate coherence_concrete_lib as lib;
use lib::{Remote, Pair, RemoteStruct};

struct Local<T>(T);

// Disallowed in standard Rust, okay in Unchained Rust
// lib only implements Remote for Pair<T, RemoteStruct>,
// no conflicts can happen with the impl bellow
impl<T,U> Remote for Pair<T, Local<U>> { }

fn main() { }
