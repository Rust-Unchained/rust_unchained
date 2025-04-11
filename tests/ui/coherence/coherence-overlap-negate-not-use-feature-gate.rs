//@ build-pass

use std::ops::DerefMut;

trait Foo {}

// For some reason, this is disallowed in standard Rust despite &T not implementing DerefMut
// This is allowed in Unchained Rust.
impl<T: DerefMut> Foo for T {}

impl<U> Foo for &U {}

fn main() {}
