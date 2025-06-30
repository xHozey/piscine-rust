#[derive(Debug)]
pub struct Vector<T: Scalar>(pub Vec<T>);

use std::ops::{Add, Mul};
use lalgebra_scalar::*;

impl<T: Scalar<Item = T> + Add<Output = T> + Mul<Output = T>> Add for Vector<T> {
	type Output = Option<Self>;

	fn add(self, rhs: Vector<T>) -> Self::Output{
		if self.0.len() != rhs.0.len() {
			return None
		}
		let mut res = Self::new();
		for i in 0..self.0.len() {
			res.0.push(self.0[i] + rhs.0[i])
		}
		Some(res)
	}
}

impl<T: Scalar<Item = T> + Add<Output = T> + Mul<Output = T>> Vector<T> {
	pub fn new() -> Self {
		Self(Vec::new())
	}

	pub fn dot(&self, other: &Self) -> Option<T> {
		if self.0.len() != other.0.len() {
			return None
		}
		let mut res = <T as Scalar>::zero();
		for i in 0..self.0.len() {
			res = res + self.0[i] * other.0[i]
		}
		Some(res)
	}
}