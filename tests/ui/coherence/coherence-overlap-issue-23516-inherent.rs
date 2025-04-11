pub trait Sugar {}

struct Cake<X>(X);

 // Additional requirement for failure in Unchained, 
 // otherwise we can't prove that these overlap
impl<T> Sugar for Box<T> {}

impl<T:Sugar> Cake<T> { fn dummy(&self) { } }
//~^ ERROR E0592
impl<U:Sugar> Cake<Box<U>> { fn dummy(&self) { } }

fn main() { }
