//@ build-pass

//@ aux-build:coherence_lib.rs

extern crate coherence_lib;

use coherence_lib::Remote;

trait Foo {}

// Allowed in Rust Unchained, i16 does not implement `Remote`
impl<T> Foo for T where T: Remote {}

impl Foo for i16 {}


fn main() {}
