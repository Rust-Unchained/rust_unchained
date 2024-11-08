//@ build-pass
//@ revisions: stock with_negative_coherence

//@[with_negative_coherence] known-bug: unknown
// Ideally this would work, but we don't use `&'a T` to imply that `T: 'a`
// which is required for `&'a T: !MyPredicate` to hold. This is similar to the
// test `negative-coherence-placeholder-region-constraints-on-unification.explicit.stderr`

#![feature(negative_impls)]
#![cfg_attr(with_negative_coherence, feature(with_negative_coherence))]

trait MyPredicate<'a> {}

impl<'a, T> !MyPredicate<'a> for &'a T where T: 'a {}

// UNCHAINED_TODO: This causes conflicts in standard Rust, but not in Unchained, for some reason.
trait MyTrait<'a> {}

impl<'a, T: MyPredicate<'a>> MyTrait<'a> for T {}
impl<'a, T> MyTrait<'a> for &'a T {}

fn main() {}
