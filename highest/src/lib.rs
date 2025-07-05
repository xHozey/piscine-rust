#[derive(Debug)]
pub struct Numbers<'a> {
    numbers: &'a [u32],
}

impl<'a> Numbers<'a> {
    pub fn new(numbers: &'a [u32]) -> Self {
        Self { numbers: &numbers }
    }

    pub fn list(&self) -> &[u32] {
        self.numbers
    }

    pub fn latest(&self) -> Option<u32> {
        self.numbers.last().map(|v| *v)
    }

    pub fn highest(&self) -> Option<u32> {
        self.numbers.iter().max().map(|v| *v)
    }

    pub fn highest_three(&self) -> Vec<u32> {
        let mut cl = self.numbers.to_vec();
        cl.sort();
        cl.iter().rev().take(3).map(|v| *v).collect()
    }
}
