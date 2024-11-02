//@ build-pass
pub trait Sugar {}

struct Cake<X>(X);

// This is disallowed in standard Rust, but not Unchained,
// impls don't overlap unless we prove that they do.
impl<T:Sugar> Cake<T> { fn dummy(&self) { } }

impl<U:Sugar> Cake<Box<U>> { fn dummy(&self) { } }

fn main() { }
