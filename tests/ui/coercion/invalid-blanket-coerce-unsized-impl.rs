// Regression test minimized from #126982.
// We used to apply a coerce_unsized coercion to literally every argument since
// the blanket applied in literally all cases, even though it was incoherent.

#![feature(coerce_unsized)]

impl<A> std::ops::CoerceUnsized<A> for A {}
//~^ ERROR conflicting implementations of trait
//~| ERROR the trait `CoerceUnsized` may only be implemented for a coercion between structures

const C: usize = 1;

fn main() {}
