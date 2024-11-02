pub trait WhereBound {}
impl WhereBound for () {}

pub trait WithAssoc<'a> {
    type Assoc;
}

pub trait Trait {}
impl<T> Trait for T
where
    T: 'static,
    for<'a> T: WithAssoc<'a>,
    for<'a> Box<  <T as WithAssoc<'a>>::Assoc  >: WhereBound,
{
}

impl<T> Trait for Box<T> {}
//~^ ERROR E0119

struct Local;
impl WithAssoc<'_> for Box<Local> {
    type Assoc = Local;
}

impl WhereBound for Box<Local> {}

fn main() {}