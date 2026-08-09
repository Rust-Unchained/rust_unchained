//@ build-pass
// Allowed in Unchained.

use std::rc::Rc;
pub struct Foo;

pub type Function = Rc<Foo>;

impl Function {}
fn main() {}
