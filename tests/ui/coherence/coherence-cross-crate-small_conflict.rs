//@ aux-build:trait_impl_conflict.rs

extern crate trait_impl_conflict;
use trait_impl_conflict::Foo;

trait MyLocal {}

// This creates a conflict, upstream defines Foo for isize
impl<A: MyLocal> Foo for A { } //~ ERROR E0119

impl MyLocal for isize {}

fn main() {
}
