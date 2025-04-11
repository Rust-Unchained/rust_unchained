//@ aux-build:coherence_lib.rs
//@ build-pass

extern crate coherence_lib as lib;
use lib::Remote;

struct Foo;

// Disallowed in Standard Rust, allowed in Unchained since 
// lib does not provide any implementations of Remote,
// which means there are no conflicts here.
impl<T> Remote for lib::Pair<T,Foo> { }

fn main() { }
