//@ build-pass

pub trait WhereBound {}
impl WhereBound for () {}

pub trait WithAssoc<'a> {
    type Assoc;
}

// These is disallowed in standard Rust because a downstream crate could implement WithAssoc for Box<Local>
// In Unchained Rust however, downstream crates don't exist.
// We made a 2nd version of this test to make sure conflicting impls would be
// detected if a user did indeed impl WithAssoc for Box<Something>,
// the new test is named `coherence-overlap-unnormalizable-projection-2`

pub trait Trait {}

impl<T> Trait for T
where
    T: 'static,
    for<'a> T: WithAssoc<'a>,
    for<'a> Box<<T as WithAssoc<'a>>::Assoc>: WhereBound,
{
}

impl<T> Trait for Box<T> {}

fn main() {}