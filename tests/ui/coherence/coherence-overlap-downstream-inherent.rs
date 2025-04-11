struct Sweet<X>(X);
pub trait Sugar {}
pub trait Fruit {}
// UNCHAINED_FIXME: This should be allowed
impl<T:Sugar> Sweet<T> { fn dummy(&self) { } }
//~^ ERROR E0592
impl<T:Fruit> Sweet<T> { fn dummy(&self) { } }

trait Bar<X> {}
struct A<T, X>(T, X);

impl<X, T> A<T, X> where T: Bar<X> { fn f(&self) {} }
//~^ ERROR E0592
impl<X> A<i32, X> { fn f(&self) {} }

// The above is allowed with unchained, provided the impl bellow doesn't exist
impl<X> Bar<X> for i32 {} 

fn main() {}
