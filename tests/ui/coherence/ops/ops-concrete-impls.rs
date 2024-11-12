//@ run-pass

impl std::ops::Add<u16> for i8 {
	type Output = i32;

	fn add(self, rhs: u16) -> Self::Output {
		self as i32 + rhs as i32
	}
}

impl std::ops::Add<i8> for u16 {
	type Output = i32;

	fn add(self, rhs: i8) -> Self::Output {
		self as i32 + rhs as i32
	}
}

fn main() {
	use std::ops::Add;
	
	let u: u16 = 10;
	let i: i8 = -5;
	let res: i32 = i.add(u);
	assert_eq!(res, 5_i32);

	// FIXME: this should compile, but doesn't
	//assert_eq!(i + u, 5_i32);
}