//@ aux-build:orphan-check-diagnostics.rs
//@ build-pass
// See issue #22388.

extern crate orphan_check_diagnostics;

use orphan_check_diagnostics::RemoteTrait;

trait LocalTrait { fn dummy(&self) { } }

// Disallowed in standard Rust, allowed in Unchained
impl<T> RemoteTrait for T where T: LocalTrait {}

fn main() {}
