#[derive(Copy, Clone)]
struct Flags;

trait A {
}

impl<T> Drop for T where T: A {
    //~^ ERROR E0120
    fn drop(&mut self) { }
}

fn main() {}
