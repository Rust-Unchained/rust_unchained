fn main() {}

impl std::ops::AddAssign for () {
    //~^ ERROR E0117
    fn add_assign(&self, other: ()) -> () {
        ()
    }
}

impl std::ops::AddAssign for [(); 1] {
    //~^ ERROR E0117
    fn add_assign(&self, other: [(); 1]) -> [(); 1] {
        [()]
    }
}

impl std::ops::AddAssign for &[u8] {
    //~^ ERROR E0117
    fn add_assign(&self, other: &[u8]) -> &[u8] {
        self
    }
}
