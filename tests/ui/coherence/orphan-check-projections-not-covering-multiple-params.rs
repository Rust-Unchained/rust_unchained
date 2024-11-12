//@ revisions: classic next
//@[next] compile-flags: -Znext-solver

//@ build-pass
//@ compile-flags: --crate-type=lib
//@ aux-crate:foreign=parametrized-trait.rs
//@ edition:2021

trait Trait<T, U> { type Assoc; }

impl<T, U> Trait<T, U> for () {
    type Assoc = LocalTy;
}

struct LocalTy;

// Warning in standard Rust, allowed in Unchained: upstream doesn't provide any impls.
impl<T, U> foreign::Trait0<LocalTy, T, U> for <() as Trait<T, U>>::Assoc {}


fn main() {}
