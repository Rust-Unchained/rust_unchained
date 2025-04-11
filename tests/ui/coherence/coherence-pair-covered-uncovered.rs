//@ aux-build:coherence_concrete_lib.rs

extern crate coherence_concrete_lib as lib;
use lib::{Remote, Pair, RemoteStruct};

struct Local<T>(T);

impl<T,U> Remote for Pair<Local<U>, T> { }
//~^ ERROR E0119

fn main() { }
