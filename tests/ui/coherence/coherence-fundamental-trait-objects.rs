// Check that trait objects from #[fundamental] traits are not
// treated as #[fundamental] types - the 2 meanings of #[fundamental]
// are distinct.
//@ build-pass
//@ aux-build:coherence_fundamental_trait_lib.rs

extern crate coherence_fundamental_trait_lib;

use coherence_fundamental_trait_lib::{Fundamental, Misc};

pub struct Local;
impl Misc for dyn Fundamental<Local> {}


fn main() {}
