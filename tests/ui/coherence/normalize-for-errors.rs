//@ build-pass

struct MyType;
trait MyTrait<S> {}

trait Mirror {
    type Assoc;
}
impl<T> Mirror for T {
    type Assoc = T;
}

impl<T: Copy, S: Iterator> MyTrait<S> for (T, S::Item) {}

// Allowed in Rust Unchained, no conflicts here. Box<T> doesn't implement Copy for any T.
impl<S: Iterator> MyTrait<S> for (Box<<(MyType,) as Mirror>::Assoc>, S::Item) {}

fn main() {}
