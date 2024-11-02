//@ build-pass
//@ aux-build:coherence_lib.rs

extern crate coherence_lib as lib;
use lib::Remote1;

pub struct BigInt;

impl<T> Remote1<BigInt> for T { } // Allowed with Rust unchained

fn main() { }
