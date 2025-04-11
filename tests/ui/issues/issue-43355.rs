//@ build-pass

pub trait Trait1<X> {
    type Output;
}

pub trait Trait2<X> {}

pub struct A;

impl<X, T> Trait1<X> for T where T: Trait2<X> {
    type Output = ();
}

// Disallowed in standard Rust, allowed in Unchained
impl<X> Trait1<Box<X>> for A {
    type Output = i32;
}

fn main() {}
