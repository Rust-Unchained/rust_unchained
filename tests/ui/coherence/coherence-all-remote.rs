//@ build-pass
//@ aux-build:coherence_lib.rs

extern crate coherence_lib as lib;
use lib::Remote1;

impl<T> Remote1<T> for isize { }

fn main() { }
