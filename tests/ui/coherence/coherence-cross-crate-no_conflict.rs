//@ build-pass
//@ aux-build:trait_impl_conflict.rs

extern crate trait_impl_conflict;
use trait_impl_conflict::Foo;

trait MyLocal {}

// This is disallowed in standard Rust,
// but allowed in Unchained since there are no actual conflicts
impl<A: MyLocal> Foo for A { }

fn main() {
}
