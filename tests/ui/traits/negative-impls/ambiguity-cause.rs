//@ revisions: simple negative_coherence
//@ build-pass

#![feature(negative_impls)]
#![cfg_attr(negative_coherence, feature(with_negative_coherence))]

trait MyTrait {}

impl<T: Copy> MyTrait for T { }

impl MyTrait for String { }

fn main() {}
