//@ build-pass
//@ revisions: stock with_negative_coherence

#![feature(negative_impls)]
#![cfg_attr(with_negative_coherence, feature(with_negative_coherence))]

trait MyPredicate<'a> {}

impl<'a, T> !MyPredicate<'a> for &'a T where T: 'a {}

// Note: This causes conflicts in standard Rust,
// but not in Unchained, for some reason.
trait MyTrait<'a> {}

impl<'a, T: MyPredicate<'a>> MyTrait<'a> for T {}

impl<'a, T> MyTrait<'a> for &'a T {}

fn main() {}
