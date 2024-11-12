//@ aux-build:coherence_lib.rs

// Test that the `Pair` type reports an error if it contains type
// parameters, even when they are covered by local types. This test
// was originally intended to test the opposite, but the rules changed
// with RFC 1023 and this became illegal.

extern crate coherence_lib as lib;
use lib::{Remote,Pair};

pub struct Cover<T>(T);

// It may seem like it, but the two bellow can never conflict:
impl<T> Remote for Pair<T,Cover<T>> { }

impl<T> Remote for Pair<Cover<T>,T> { }

// This one conflicts though
impl<T,U> Remote for Pair<Cover<T>,U> { }
//~^ ERROR E0119

fn main() { }
