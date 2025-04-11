// This test previously tried to use a tainted `EvalCtxt` when emitting
// an error during coherence.
#![feature(specialization)]
//~^ WARN the feature `specialization` is incomplete
trait Iterate<'a> {
    type Ty: Valid;
}
impl<'a, T> Iterate<'a> for T
where
    T: Check,
{
    default type Ty = ();
    //~^ ERROR the trait bound `(): Valid` is not satisfied
}

trait Check {}
impl<'a, T> Eq for T where <T as Iterate<'a>>::Ty: Valid {} //~ ERROR E0277
//~^ ERROR E0277


trait Valid {}

fn main() {}
