//@ build-pass

trait WhereBound {}
impl WhereBound for () {}


pub trait WithAssoc<'a> {
    type Assoc;
}

// These two impls of `Trait` are considered to overlap in standard Rust.
// In Unchained Rust, impls don't overlap unless we can prove that they do.
// Here, Box<T> doesn't implement WithAssoc for any T

pub trait Trait {}
impl<T> Trait for T
where
    T: 'static,
    for<'a> T: WithAssoc<'a>,
    for<'a> <T as WithAssoc<'a>>::Assoc: WhereBound,
{
}

impl<T> Trait for Box<T> {}

fn main() {}