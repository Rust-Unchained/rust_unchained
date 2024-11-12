//@ build-pass

//@ aux-build:coherence_copy_like_lib.rs
#![allow(dead_code)]

extern crate coherence_copy_like_lib as lib;

struct MyType { x: i32 }

// Allowed in Rust Unchained
impl lib::MyCopy for (MyType,) { }

fn main() { }
