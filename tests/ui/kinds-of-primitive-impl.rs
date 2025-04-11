impl u8 {
    pub const B: u8 = 0;
}

impl str {
    fn foo() {}
    fn bar(self) {} //~ ERROR: size for values of type `str` cannot be known
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

fn main() {}
