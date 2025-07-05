#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.v == 1 || self.v == 0 {
            return None
        }
        if self.v % 2 == 0 {
            self.v /= 2
        } else {
            self.v = 3*self.v+1
        }
        Some(self.v)
    }
}

impl Collatz {
	pub fn new(n: u64) -> Self {
        Self { v: n }
    }
}

pub fn collatz(n: u64) -> usize {
    let mut acc = 0;
    let s = Collatz::new(n);
    let mut iter = s.into_iter();
    while let Some(_) = iter.next() {
        acc += 1;
    }
    acc
}