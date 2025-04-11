//@ build-pass

trait Mirror {
    type Assoc;
}
impl<T> Mirror for T {
    type Assoc = T;
}

trait Foo {}

// Allowed in Rust Unchained, no conflicts here.
impl<T> Foo for T where (): Mirror<Assoc = T> {}

impl<T> Foo for T where T: Iterator {}

fn main() {}
