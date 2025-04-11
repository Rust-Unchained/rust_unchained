//@ build-pass

//@ aux-build:extern-crate.rs
#![feature(rustc_attrs)]
extern crate extern_crate;

impl extern_crate::StructWithAttr {
    fn foo() {}
}
impl extern_crate::StructWithAttr {
    #[rustc_allow_incoherent_impl]
    fn bar() {}
}
impl extern_crate::StructNoAttr {
    fn foo() {}
}
impl extern_crate::StructNoAttr {
    #[rustc_allow_incoherent_impl]
    fn bar() {}
}
impl extern_crate::EnumWithAttr {
    fn foo() {}
}
impl extern_crate::EnumWithAttr {
    #[rustc_allow_incoherent_impl]
    fn bar() {}
}
impl extern_crate::EnumNoAttr {
    fn foo() {}
}
impl extern_crate::EnumNoAttr {
    #[rustc_allow_incoherent_impl]
    fn bar() {}
}

fn main() {}
