//@ build-pass

//@ aux-build:coherence_copy_like_lib.rs
#![allow(dead_code)]

extern crate coherence_copy_like_lib as lib;

struct MyType { x: i32 }

// Disallowed in standard Rust, allowed in Unchained, there are no conflicts here.
impl lib::MyCopy for lib::MyStruct<MyType> { }


fn main() { }
