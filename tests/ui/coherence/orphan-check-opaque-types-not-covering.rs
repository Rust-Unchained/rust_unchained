//@ build-pass
//@ aux-crate:foreign=parametrized-trait.rs
//@ edition:2021

#![feature(type_alias_impl_trait)]

type Identity<T> = impl Sized;

fn define_identity<T>(x: T) -> Identity<T> {
    x
}

// Disallowed in standard Rust, allowed in Unchained
impl<T> foreign::Trait0<Local, T, ()> for Identity<T> {}

type Opaque<T> = impl Sized;

fn define_local<T>() -> Opaque<T> {
    Local
}

// Disallowed in standard Rust, allowed in Unchained
impl<T> foreign::Trait1<Local, T> for Opaque<T> {}

struct Local;

fn main() {}
