//@ build-pass

struct S;

impl From<()> for S {
    fn from(x: ()) -> Self {
        S
    }
}

// Allowed in Rust Unchained, no conflicts here.
impl<I> From<I> for S
where
    I: Iterator<Item = ()>,
{
    fn from(x: I) -> Self {
        S
    }
}

fn main() {}
