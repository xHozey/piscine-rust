use crate::RomanDigit::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

impl From<u32> for RomanNumber {
    fn from(value: u32) -> Self {
        if value == 0 {
            return RomanNumber(vec![Nulla]);
        }

        let thousands = vec![vec![], vec![M], vec![M, M], vec![M, M, M]];
        let hundreds = vec![
            vec![],
            vec![C],
            vec![C, C],
            vec![C, C, C],
            vec![C, D],
            vec![D],
            vec![D, C],
            vec![D, C, C],
            vec![D, C, C, C],
            vec![C, M],
        ];
        let tens = vec![
            vec![],
            vec![X],
            vec![X, X],
            vec![X, X, X],
            vec![X, L],
            vec![L],
            vec![L, X],
            vec![L, X, X],
            vec![L, X, X, X],
            vec![X, C],
        ];
        let ones = vec![
            vec![],
            vec![I],
            vec![I, I],
            vec![I, I, I],
            vec![I, V],
            vec![V],
            vec![V, I],
            vec![V, I, I],
            vec![V, I, I, I],
            vec![I, X],
        ];

        let mut res = vec![];
        res.extend(thousands[value as usize / 1000].iter().cloned());
        res.extend(hundreds[(value % 1000) as usize / 100].iter().cloned());
        res.extend(tens[(value % 100) as usize / 10].iter().cloned());
        res.extend(ones[(value % 10) as usize].iter().cloned());

        RomanNumber(res)
    }
}

fn roman_to_digit(r: RomanNumber) -> u32 {
    let mut res = 0;
    let mut i = 0;
    while i < r.0.len() {
        let current_value = match r.0[i] {
            RomanDigit::I => 1,
            RomanDigit::V => 5,
            RomanDigit::X => 10,
            RomanDigit::L => 50,
            RomanDigit::C => 100,
            RomanDigit::D => 500,
            RomanDigit::M => 1000,
            RomanDigit::Nulla => 0,
        };


        if i + 1 < r.0.len() {
            let next_value = match r.0[i+1] {
            RomanDigit::I => 1,
            RomanDigit::V => 5,
            RomanDigit::X => 10,
            RomanDigit::L => 50,
            RomanDigit::C => 100,
            RomanDigit::D => 500,
            RomanDigit::M => 1000,
            RomanDigit::Nulla => 0,
        };
            if current_value >= next_value {
                res += current_value
            } else {
                res += next_value - current_value;
                i += 1

            }
        } else {
            res += current_value
        }
        i += 1
    }
    res
}

impl Iterator for RomanNumber {
    type Item = Self;
    fn next(&mut self) -> Option<Self::Item> {
        let digit = roman_to_digit(self.clone());
        println!("{}", digit);
        let roman = Self::from(digit + 1);
        self.0 = roman.0.clone();
        Some(roman)
    }
}
