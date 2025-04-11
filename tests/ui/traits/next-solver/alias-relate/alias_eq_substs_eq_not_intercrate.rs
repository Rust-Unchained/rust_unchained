//@ compile-flags: -Znext-solver

// check that a `alias-eq(<?a as TraitB>::Assoc, <?b as TraitB>::Assoc)` goal fails
// during coherence. We must not incorrectly constrain `?a` and `?b` to be
// equal.

trait TraitB {
    type Assoc;
}

trait Overlaps<T> {}

impl<T: TraitB> Overlaps<Box<T>> for <T as TraitB>::Assoc {}
impl<U: TraitB> Overlaps<U> for <U as TraitB>::Assoc {}
//~^ ERROR conflicting implementations of trait

// The two bellow are additional requirements for failure in Unchained Rust
// Unchained Rust only forbids these impls if a conflict actually occurs.
impl TraitB for Box<i32> {
    type Assoc = i32;
}

impl TraitB for i32 {
    type Assoc = i32;
}

fn main() {}
