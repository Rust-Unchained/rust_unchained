//@ build-pass

// Allowed in Rust Unchained.

struct Foo {
    x: i32
}

impl *mut Foo {}

impl fn(Foo) {}

fn main() {
}
