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

impl<T: Eq + Add<Output = T> + Copy> std::iter::Iterator for StepIterator<T> {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        if self.beg == self.step {
            return None;
        }
        let old = self.beg + self.step;
        self.beg = old;
        Some(Self {
            beg: old,
            end: self.end,
            step: self.step,
        })
    }
}
