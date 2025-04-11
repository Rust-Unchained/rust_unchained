// Test that we are able to introduce a negative constraint that
// `MyType: !MyTrait` along with other "fundamental" wrappers.
//@ build-pass
//@ aux-build:coherence_copy_like_lib.rs


extern crate coherence_copy_like_lib as lib;

struct MyType { x: i32 }

trait MyTrait { fn foo() {} }

impl<T: lib::MyCopy> MyTrait for T { }

impl MyTrait for lib::MyFundamentalStruct<(MyType,)> { }


fn main() { }
