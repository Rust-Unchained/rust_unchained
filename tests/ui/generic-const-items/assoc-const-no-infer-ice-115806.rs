//@ build-pass
// ICE: assertion failed: !value.has_infer()
// issue: rust-lang/rust#115806
#![feature(adt_const_params, min_generic_const_args, unsized_const_params)]
#![feature(associated_type_defaults)]
#![allow(incomplete_features)]

pub struct NoPin;

impl<TA> Pins<TA> for NoPin {}

pub trait PinA<PER> {
    type const A: &'static () = const { &() };
}

pub trait Pins<USART> {}

// Disallowed by standard Rust, allowed by Unchained Rust since there are no conflicts here
// NoPin doesn't implement PinA, so no impls for Pins overlap
impl<USART, T> Pins<USART> for T where T: PinA<USART, A = const { &() }> {}

pub fn main() {}
