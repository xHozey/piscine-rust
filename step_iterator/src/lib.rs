#[derive(Clone)]
pub struct StepIterator<T> {
    pub beg: T,
    pub end: T,
    pub step: T,
}

use std::ops::Add;
impl<T> StepIterator<T> {
    pub fn new(beg: T, end: T, step: T) -> Self {
        Self { beg, end, step }
    }
}

impl<T: std::fmt::Debug + Add<Output = T> + Copy + std::cmp::PartialOrd + > std::iter::Iterator for StepIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.beg > self.end {
            return None;
        }
        let old = self.beg;
        self.beg = self.beg + self.step;
        Some(old)
    }
}
