

pub trait Sugar { fn dummy(&self) { } }
pub trait Sweet { fn dummy(&self) { } }
impl<T:Sugar> Sweet for T { }
impl<U:Sugar> Sweet for Box<U> { }

// Additional requirements for conflicts to happen in Unchained Rust
impl Sugar for i32 {}
impl Sweet for Box<i32> {}
//~^ ERROR E0119

fn main() { }
