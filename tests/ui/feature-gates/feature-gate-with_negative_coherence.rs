//@ build-pass

trait Foo { }

impl<T: std::ops::DerefMut> Foo for T { }

// Error in standard Rust, allowed in Unchained
// There are no conflicts here since &T doesn't implement DerefMut for any T
impl<T> Foo for &T { }

fn main() { }
