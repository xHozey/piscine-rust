pub fn sum(a: &[i32]) -> i32 {
    let mut res = 0;
    for i in 0..a.len() {
        res += a[i]
    }
    res
}

pub fn thirtytwo_tens() -> [i32; 32] {
    [10;32]
}