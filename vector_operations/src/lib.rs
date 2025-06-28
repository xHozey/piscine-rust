#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ThreeDVector<T> {
	pub i: T,
	pub j: T,
	pub k: T,
}

use std::ops::{Add, Sub};

impl<T: std::ops::Add<Output = T>> Add for ThreeDVector<T> {
    type Output = Self;
    fn add(self, rhs: ThreeDVector<T>) -> <Self as Add<ThreeDVector<T>>>::Output { 
        Self {
            i: self.i + rhs.i,
            j: self.j + rhs.j,
            k: self.k + rhs.k
        }
    }
}

impl<T: std::ops::Sub<Output = T>> Sub for ThreeDVector<T> {
    type Output = Self;
    fn sub(self, rhs: ThreeDVector<T>) -> <Self as Sub<ThreeDVector<T>>>::Output { 
        Self {
            i: self.i - rhs.i,
            j: self.j - rhs.j,
            k: self.k - rhs.k
        }
    }
}