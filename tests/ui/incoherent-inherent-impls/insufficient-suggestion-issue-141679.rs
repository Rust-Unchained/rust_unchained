//@ run-pass
// Allowed in Unchained.

use std::rc::Rc;
pub struct Foo;

pub type Function = Rc<Foo>;

impl Function {
    fn foo(&self) {}
}
fn main() {
    let f = Function::new(Foo);
    f.foo();
}
