//@ build-pass

#![feature(rustc_attrs)]


pub trait Foo<P> { fn foo() {} }

pub trait Bar {
    type Output: 'static;
}

impl Foo<i32> for i32 { }

// Allowed in Rust Unchained, i32 does not implement `Iterator` 
impl<A:Iterator> Foo<A::Item> for A { }

fn main() {}
