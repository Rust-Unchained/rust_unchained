//@ run-pass
// Allowed in Unchained.

impl u8 {
    pub const B: u8 = 0;
}

impl str {
    fn foo() {}
    fn bar(&self) {}
}

impl char {
    pub const B: u8 = 0;
    pub const C: u8 = 0;
    fn foo() {}
    fn bar(self) {}
}

struct MyType;
impl &MyType {
    pub fn for_ref(self) {}
}

fn main() {
    let c = 'c';
    c.bar(); // Verify that compiler detects our inherent impl.
    let s = "s";
    s.bar();

    char::foo();
    str::foo();

    let ty = MyType {};
    <&MyType>::for_ref(&ty);
}
