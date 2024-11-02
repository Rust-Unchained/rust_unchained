//@ build-pass
#![feature(negative_impls)]
#![feature(with_negative_coherence)]

struct Wrap<T>(T);

trait Foo {}
impl<T: 'static> !Foo for Box<T> {}

trait Bar {}
impl<T> Bar for T where T: Foo {}

// Disallowed in standard Rust, okay in Unchained
// Box<T> already implies T: 'static, so the two bellow are equal:
// impl<T: 'static> !Foo for Box<T> {}
// impl<T> !Foo for Box<T> {}
//
// Which means no implementations overlap here.
impl<T> Bar for Box<T> {}

fn main() {}
