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