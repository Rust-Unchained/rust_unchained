//@ revisions: any_lt static_lt
//@[static_lt] check-pass
//@[any_lt] build-pass

#![feature(negative_impls)]
#![feature(with_negative_coherence)]

trait Foo {}

impl<T> !Foo for &'static T {}

trait Bar {}

impl<T> Bar for T where T: Foo {}

// Allowed in Rust Unchained, no conflicts here.
// Note: there is no implementation of Foo, it only has a negative impl, 
// therefore the line 14 doesn't implement Bar for any actual Type. 
#[cfg(any_lt)]
impl<T> Bar for &T {}

#[cfg(static_lt)]
impl<T> Bar for &'static T {}


fn main() {}
