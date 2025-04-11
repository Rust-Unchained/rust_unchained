// Emit additional suggestion to correct the trait implementation
// on a pointer
use std::{fmt, marker};

struct LocalType;

// Allowed in Rust Unchained.
impl fmt::Display for *mut LocalType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "This not compile")
    }
}

impl<T> marker::Copy for *mut T {
    //~^ ERROR E0119
	//~| NOTE conflicting implementation in crate `core`
}

fn main() {}
