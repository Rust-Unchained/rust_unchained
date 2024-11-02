//@ build-pass
//@ revisions: explicit implicit

#![forbid(coherence_leak_check)]
#![feature(negative_impls, with_negative_coherence)]

pub trait Marker {}

#[cfg(implicit)]
impl<T: ?Sized> !Marker for &T {}

#[cfg(explicit)]
impl<'a, T: ?Sized + 'a> !Marker for &'a T {}

trait FnMarker {}

// Disallowed in standard Rust, allowed in Unchained, no conflicts here.
impl<T: ?Sized + Marker> FnMarker for fn(T) {}
impl<T: ?Sized> FnMarker for fn(&T) {}

fn main() {}
