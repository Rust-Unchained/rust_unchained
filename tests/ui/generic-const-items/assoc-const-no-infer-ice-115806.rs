//@ build-pass
// ICE: assertion failed: !value.has_infer()
// issue: rust-lang/rust#115806
#![feature(associated_const_equality)]
#![allow(incomplete_features)]

pub struct NoPin;

impl<T> Pins<T> for NoPin {}

pub trait PinA<PER> {
    const A: &'static () = &();
}

pub trait Pins<T> {}

// Disallowed by standard Rust, allowed by Unchained Rust since there are no conflicts here
// NoPin doesn't implement PinA, so no impls for Pins overlap
impl<U, T> Pins<U> for T where T: PinA<U, A = { &() }> {}

pub fn main() {}
