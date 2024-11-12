//@ build-pass
//@ aux-build:coherence_lib.rs

extern crate coherence_lib;

use coherence_lib::Remote;

struct A<X>(X);

// Allowed in Rust Unchained, i16 does not implement `Remote`
impl<T> A<T> where T: Remote { fn dummy(&self) { } }

impl A<i16> { fn dummy(&self) { } }

fn main() {}
