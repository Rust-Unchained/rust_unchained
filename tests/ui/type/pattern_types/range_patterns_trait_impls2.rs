#![feature(pattern_types, rustc_attrs)]
#![feature(pattern_type_macro)]
#![allow(incomplete_features)]

//! check that pattern types can have local traits
//! implemented for them.

use std::pat::pattern_type;

type Y = pattern_type!(u32 is 1..);

impl Eq for Y {} //~ ERROR E0277

fn main() {}
