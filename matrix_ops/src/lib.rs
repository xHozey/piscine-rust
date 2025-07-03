
use std::ops::{Add, Sub};
use lalgebra_scalar::Scalar;

#[derive(Debug, PartialEq, Eq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl <T: Scalar<Item = T>> Matrix<T> {
	pub fn new() -> Matrix<T> {
        Self(Vec::new())
	}

	pub fn zero(row: usize, col: usize) -> Matrix<T> {
        Self(vec![vec![<T as Scalar>::zero();col];row])
	}

	pub fn identity(n: usize) -> Matrix<T> {
        let mut res = vec![vec![<T as Scalar>::zero(); n]; n];
        let mut acc = 0;
        for c in &mut res {
            c[acc] = <T as Scalar>::one();
            acc += 1;
        };
        Self(res)
	}
}

impl<T: Scalar<Item = T> + Add<Output = T>> Add for Matrix<T> {
    type Output = Option<Matrix<T>>;
    fn add(self, rhs: Self) -> Self::Output {
        if self.0.len() != rhs.0.len() {
            return None;
        }
        let mut res = Self::new();
        for i in 0..self.0.len() {
            if self.0[i].len() != rhs.0[i].len() {
                return None
            }
            let mut buffer = Vec::new();
            for j in 0..self.0[i].len() {
                buffer.push(self.0[i][j] + rhs.0[i][j]);
            }
            res.0.push(buffer);
        }
        Some(res)
    }
}

impl<T: Scalar<Item = T> + Sub<Output = T>> Sub for Matrix<T> {
    type Output = Option<Matrix<T>>;
    fn sub(self, rhs: Self) -> Self::Output {
        if self.0.len() != rhs.0.len() {
            return None;
        }
        let mut res = Self::new();
        for i in 0..self.0.len() {
            if self.0[i].len() != rhs.0[i].len() {
                return None
            }
            let mut buffer = Vec::new();
            for j in 0..self.0[i].len() {
                buffer.push(self.0[i][j] - rhs.0[i][j]);
            }
            res.0.push(buffer);
        }
        Some(res)
    }
}
