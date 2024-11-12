//@ run-pass

#![feature(assert_matches)]

use std::ops::Add;

#[derive(Debug, Clone, Copy)]
struct Int(u16);

impl<T: Add<u16, Output = u16>> std::ops::Add<T> for Int {
	type Output = Int;

	fn add(self, rhs: T) -> Self::Output {
		Int(rhs + self.0)
	}
}

impl<T: Add<u16, Output = u16>> std::ops::Add<Int> for T {
	type Output = Int;

	fn add(self, rhs: Int) -> Self::Output {
		Int(self + rhs.0)
	}
}

fn main() {
	use std::assert_matches::assert_matches;
	
	let a = Int(10);
	let b = 5_u16;
	let res: Int =  a + b;
	assert_matches!(res, Int(15));

	assert_matches!(a + b, Int(15));
}