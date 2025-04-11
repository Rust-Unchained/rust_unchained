// Regression test for issue #99554.
// Projections might not cover type parameters.

//@ revisions: classic next
//@[next] compile-flags: -Znext-solver

//@ compile-flags: --crate-type=lib
//@ aux-crate:foreign=parametrized-trait.rs
//@ build-pass
//@ edition:2021

trait Identity {
    type Output;
}

impl<T> Identity for T {
    type Output = T;
}

struct Local;

// Disallowed in standard Rust, allowed in Unchained
impl<T> foreign::Trait0<Local, T, ()> for <T as Identity>::Output {}

// Disallowed in standard Rust, allowed in Unchained
impl<T> foreign::Trait0<<T as Identity>::Output, Local, T> for Option<T> {}

pub trait Deferred {
    type Output;
}

// Disallowed in standard Rust, allowed in Unchained
impl<T: Deferred> foreign::Trait1<Local, T> for <T as Deferred>::Output {}
