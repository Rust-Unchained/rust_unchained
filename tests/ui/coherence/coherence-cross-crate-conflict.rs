//@ aux-build:trait_impl_conflict.rs

extern crate trait_impl_conflict;
use trait_impl_conflict::Foo;

// Conflicting implementations, upstream defines Foo for isize
impl<A> Foo for A { //~ ERROR E0119
}

fn main() {
}
