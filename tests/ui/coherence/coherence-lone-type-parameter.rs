//@ build-pass
//@ aux-build:coherence_lib.rs

extern crate coherence_lib as lib;
use lib::Remote;

// This is disallowed in standard Rust, however, if for some reason lib
// doesn't provide any implementation of Remote, then Unchained Rust is okay with this impl.
impl<T> Remote for T { }


fn main() { }
