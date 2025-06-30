use std::ops::{Add, Mul};
use lalgebra_scalar::Scalar;
#[derive(Debug, PartialEq, Eq,Clone)]
pub struct Vector<T: Scalar>(pub Vec<T>);


impl<T: Scalar> Add for Vector<T> {
	type Output = Vec<<T as Add>::Output>;

	fn add(self, rhs: Self) -> Self::Output {
		let mut res = vec![];
		for i in 0..self.0.len() {
			res.push(self.0[i] + rhs.0[i]);
		}
		res
	}
}

impl<T: Scalar<Item = T> + std::ops::AddAssign> Vector<T> where T: Scalar + Add<Output = T> + Mul<Output = T> {
	pub fn new() -> Self {
		Self(vec![])
	}

	pub fn dot(&self, other: &Self) -> Option<T> {
		if self.0.len() != other.0.len() {
			return None
		}
		let mut res = T::zero();
		for i in 0..self.0.len() {
			res += self.0[i] * other.0[i]
		}
		Some(res)
	}
}