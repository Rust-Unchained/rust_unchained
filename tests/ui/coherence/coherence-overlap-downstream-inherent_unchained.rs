//@ build-pass

trait Bar<X> {}
struct A<T, X>(T, X);

// This is disallowed in standard Rust because downstream crates,
// could implement Bar<X> for i32. But they can't, they don't exist.
impl<X, T> A<T, X> where T: Bar<X> { fn f(&self) {} }
impl<X> A<i32, X> { fn f(&self) {} }

fn main() {}
