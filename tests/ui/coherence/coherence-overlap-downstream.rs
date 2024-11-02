// Tests that we consider `T: Sugar + Fruit` to be ambiguous, even
// though no impls are found.

pub trait Sugar {}
pub trait Fruit {}
pub trait Sweet {}

// UNCHAINED_TODO: This should be allowed in the future,
// conflicts should only be reported if they do happen.
impl<T:Sugar> Sweet for T { }
impl<T:Fruit> Sweet for T { }
//~^ ERROR E0119

pub trait Foo<X> {}
pub trait Bar<X> {}
impl<X, T> Foo<X> for T where T: Bar<X> {}
// UNCHAINED_TODO: This causes conflicts in standard Rust, but not in unchained, for some reason.
// Although this is desired behavior since i32 does not implement `Bar`, I left this `TODO` so that this maybe reviewed later.
impl<X> Foo<X> for i32 {}

fn main() { }
