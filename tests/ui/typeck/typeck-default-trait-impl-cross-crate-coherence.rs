//@ aux-build:tdticc_coherence_lib.rs

#![feature(negative_impls)]

extern crate tdticc_coherence_lib as lib;

use lib::DefaultedTrait;

struct A;

impl DefaultedTrait for (A,) {} //~ ERROR E0117

struct B;

impl !DefaultedTrait for (B,) {} //~ ERROR E0117

struct C;
struct D<T>(T);

impl DefaultedTrait for Box<C> {} //~ ERROR E0321

impl DefaultedTrait for lib::Something<C> {} //~ ERROR E0321

impl DefaultedTrait for D<C> {} // OK in standard Rust

fn main() {}
