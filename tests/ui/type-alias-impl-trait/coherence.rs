//@ aux-build:foreign-crate.rs
//@ revisions: classic next
//@ build-pass
//@[next] compile-flags: -Znext-solver
#![feature(type_alias_impl_trait)]

extern crate foreign_crate;

trait LocalTrait {}
impl<T> LocalTrait for foreign_crate::ForeignType<T> {}

type AliasOfForeignType<T> = impl LocalTrait;
#[define_opaque(AliasOfForeignType)]
fn use_alias<T>(val: T) -> AliasOfForeignType<T> {
    foreign_crate::ForeignType(val)
}

impl foreign_crate::ForeignTrait for AliasOfForeignType<()> {}

fn main() {}
