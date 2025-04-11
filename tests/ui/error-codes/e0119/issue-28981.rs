use std::ops::Deref;

struct Foo;

impl<Foo> Deref for Foo { } //~ ERROR E0119

fn main() {}
