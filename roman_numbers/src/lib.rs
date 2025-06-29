use crate::RomanDigit::*;

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
    fn from(value: u32) -> Self {
        match value {
            1 => RomanDigit::I,
            5 => RomanDigit::V,
            10 => RomanDigit::X,
            50 => RomanDigit::L,
            100 => RomanDigit::C,
            500 => RomanDigit::D,
            1000 => RomanDigit::M,
            _ => RomanDigit::Nulla
        }
    }
}

impl From<u32> for RomanNumber {
    fn from(value: u32) -> Self {
        if value == 0 {
            return RomanNumber(vec![Nulla]);
        }

        let thousands = vec![
            vec![],
            vec![M],
            vec![M, M],
            vec![M, M, M],
        ];
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