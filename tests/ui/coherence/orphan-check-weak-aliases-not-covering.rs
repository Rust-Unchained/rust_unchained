//@ build-pass

//@ aux-crate:foreign=parametrized-trait.rs
//@ edition:2021

#![feature(lazy_type_alias)]
#![allow(incomplete_features)]

type Identity<T> = T;

struct Local;

// Error in standard Rust, allowed in Unchained
impl<T> foreign::Trait1<Local, T> for Identity<T> {}

fn main() {}
