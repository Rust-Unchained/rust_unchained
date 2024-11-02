//@ build-pass
//@ aux-build:coherence_lib.rs

extern crate coherence_lib as lib;
use lib::Remote;

struct Local<T>(T);

// Disallowed in standard Rust, allowed in Unchained, no conflicts happen
// since lib does not provide any impls that could possibly conflict with this one.
impl<T> Remote for Vec<Local<T>> { }

fn main() { }
