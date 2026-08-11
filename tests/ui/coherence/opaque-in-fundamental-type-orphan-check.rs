//! Regression test for #136188.
//!
//! Orphan checking with the next solver used to ICE when an opaque type was wrapped in a
//! fundamental type.

//@ compile-flags: --crate-type=lib -Znext-solver

#![feature(type_alias_impl_trait)]

type Opaque = Box<impl Sized>;
//~^ ERROR unconstrained opaque type

fn define() -> Opaque {
    Box::new(())
    //~^ ERROR mismatched types
}

impl Copy for Opaque {}
//~^ ERROR the trait `Copy` cannot be implemented for this type
