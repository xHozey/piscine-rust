use lalgebra_scalar::Scalar;
use std::ops::{Add, Mul};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar<Item = T>> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Self(Vec::new())
    }

    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        Self(vec![vec![<T as Scalar>::zero(); col]; row])
    }

    pub fn identity(n: usize) -> Matrix<T> {
        let mut res = vec![vec![<T as Scalar>::zero(); n]; n];
        let mut acc = 0;
        for c in &mut res {
            c[acc] = <T as Scalar>::one();
            acc += 1;
        }
        Self(res)
    }
}

impl<T: Clone> Matrix<T> {
    pub fn number_of_cols(&self) -> usize {
        let mut res = 0;
        for _ in 0..self.0[0].len() {
            res += 1;
        }
        res
    }

    pub fn number_of_rows(&self) -> usize {
        let mut res = 0;
        for _ in 0..self.0.len() {
            res += 1;
        }
        res
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        let mut res = Vec::new();
        for i in 0..self.0.len() {
            res.push(self.0[i][n].clone())
        }
        res
    }
}

impl<T: Scalar<Item = T> + Mul<Output = T> + Add<Output = T> + Copy> Mul for Matrix<T> {
    type Output = Option<Self>;

    fn mul(self, rhs: Self) -> Self::Output {
        let a_rows = self.0.len();
        let a_cols = if a_rows > 0 { self.0[0].len() } else { 0 };
        let b_rows = rhs.0.len();
        let b_cols = if b_rows > 0 { rhs.0[0].len() } else { 0 };

        if a_cols != b_rows {
            return None;
        }

        let mut result = vec![vec![T::zero(); b_cols]; a_rows];

        for i in 0..a_rows {
            for j in 0..b_cols {
                let mut sum = T::zero();
                for k in 0..a_cols {
                    sum = sum + (self.0[i][k] * rhs.0[k][j]);
                }
                result[i][j] = sum;
            }
        }

        Some(Matrix(result))
    }
}
