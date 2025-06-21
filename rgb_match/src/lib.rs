#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        let mut val = [self.r, self.g, self.b, self.a];
        for i in 0..val.len() {
            if first == val[i] {
                val[i] = second;
            } else if second == val[i] {
                val[i] = first
            }
        }
        Self { r: val[0], g: val[1], b: val[2], a: val[3] }
    }
}