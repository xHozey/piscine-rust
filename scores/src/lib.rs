pub fn score(s: &str) -> u64 {
    let mut res = 0;
    s.chars().for_each(|c| {
        match c.to_ascii_uppercase() {
            'A'| 'E'| 'I'| 'O'| 'U'| 'L'| 'N'| 'R'| 'S'| 'T' => res += 1,
            'D'| 'G' => res += 2,
            'B'| 'C'| 'M'| 'P' => res += 3,
            'F'| 'H'| 'V'| 'W'| 'Y' => res += 4,
            'K' => res += 5,
            'J' | 'X' => res += 8,
            'Q' | 'Z' => res += 10,
            _ => {}
        }
    });
    res
}