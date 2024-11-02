//@ revisions: any_lt static_lt
//@[static_lt] check-pass
//@[any_lt] build-pass // UNCHAINED_TODO: this should not pass

#![feature(negative_impls)]
#![feature(with_negative_coherence)]

trait Foo {}

impl<T> !Foo for &'static T {}

trait Bar {}

impl<T> Bar for T where T: Foo {}

// UNCHAINED_TODO: This passing is currently a bug, there are conflicts here.
#[cfg(any_lt)]
impl<T> Bar for &T {}

#[cfg(static_lt)]
impl<T> Bar for &'static T {}


fn main() {}
