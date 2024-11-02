//@ build-pass
//@ aux-build:coherence_copy_like_lib.rs

extern crate coherence_copy_like_lib as lib;

struct MyType { x: i32 }

trait MyTrait { fn foo() {} }
impl<T: lib::MyCopy> MyTrait for T { }

// Disallowed in standard Rust, allowed in Unchained, no actual conflicts here.
impl MyTrait for lib::MyStruct<MyType> { }

fn main() { }
