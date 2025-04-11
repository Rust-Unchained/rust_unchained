//@ build-pass
// Disallowed in standard Rust, allowed in Unchained, this only causes conflicts if 
// an implementation of Box<X: Sugar> is provided.

pub trait Sugar { fn dummy(&self) { } }
pub trait Sweet { fn dummy(&self) { } }
impl<T:Sugar> Sweet for T { }
impl<U:Sugar> Sweet for Box<U> { }

fn main() { }
