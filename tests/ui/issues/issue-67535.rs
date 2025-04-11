//@ build-pass

fn main() {}

impl std::ops::AddAssign for () {
    fn add_assign(&mut self, other: ()) -> () {
        ()
    }
}

impl std::ops::AddAssign for [(); 1] {
    fn add_assign(&mut self, other: [(); 1]) {
        
    }
}

impl std::ops::AddAssign for &[u8] {
    fn add_assign(&mut self, other: &[u8]) {
        
    }
}
