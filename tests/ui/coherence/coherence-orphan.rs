//@ aux-build:coherence_orphan_lib.rs
#![feature(negative_impls)]

extern crate coherence_orphan_lib as lib;

use lib::TheTrait;

struct TheType;

impl TheTrait<usize> for isize {
	fn the_fn(&self) {}
}

impl TheTrait<TheType> for isize {
	fn the_fn(&self) {}
}

impl TheTrait<isize> for TheType {
	fn the_fn(&self) {}
}

impl !Send for Vec<isize> {} //~ ERROR E0321

fn main() {}
