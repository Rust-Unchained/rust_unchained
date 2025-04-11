//@ aux-build:coherence_concrete_lib.rs

extern crate coherence_concrete_lib as lib;
use lib::Remote;

struct Local<T>(T);

// lib implements Remote for Vec<T: Clone>
impl<T> Remote for Vec<Local<T>> { }
//~^ ERROR E0119

// Additional requirement for failure in Unchained Rust,
// this creates a conflicting impl since lib implements Remote for Vec<T: Clone>
impl<T> Clone for Local<T> {
    fn clone(&self) -> Self {
        todo!()
    }
}

fn main() { }