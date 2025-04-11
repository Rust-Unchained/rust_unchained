//@ aux-build:coherence_concrete_lib.rs

extern crate coherence_concrete_lib as lib;
use lib::Remote;

struct Local;

impl Remote for Vec<Local> { }
//~^ ERROR E0119

// Additional requirement for failure in Unchained Rust,
// this creates a conflicting impl since lib implements Remote for Vec<T: Clone>
impl Clone for Local {
    fn clone(&self) -> Self { Local }
}

fn main() { }
