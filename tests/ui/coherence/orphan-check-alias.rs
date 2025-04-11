//@ check-pass

//@ revisions: classic next
//@[next] compile-flags: -Znext-solver

//@ aux-crate:foreign=parametrized-trait.rs
//@ edition:2021

//@ known-bug: #99554

trait Id {
    type Assoc;
}

impl<T> Id for T {
    type Assoc = T;
}

pub struct B;

// Allowed in Unchained Rust, upstream doesn't provide any impls.
impl<T> foreign::Trait2<B, T> for <T as Id>::Assoc {
    type Assoc = usize;
}

fn main() {}
