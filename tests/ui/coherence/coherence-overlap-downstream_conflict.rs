pub trait Foo<X> {}
pub trait Bar<X> {}
impl<X, T> Foo<X> for T where T: Bar<X> {}

impl Bar<()> for i32 {}

impl<X> Foo<X> for i32 {}
//~^ ERROR E0119

fn main() { }
