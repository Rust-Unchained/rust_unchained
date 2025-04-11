//@ build-pass

//@ aux-build:extern-crate.rs
extern crate extern_crate;

impl extern_crate::StructWithAttr {}


impl extern_crate::StructNoAttr {}

impl extern_crate::EnumWithAttr {}

impl extern_crate::EnumNoAttr {}

impl f32 {}

fn main() {}
