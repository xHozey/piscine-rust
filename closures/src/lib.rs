pub fn first_fifty_even_square() -> Vec<i32> {
    let x = || {
        let mut res = Vec::new();
        let mut acc: i32 = 2;
        while res.len() < 50 {
            if acc.pow(2) % 2 == 0 {
                res.push(acc.pow(2));
            }
            acc += 1;
        }
        return res
    };
    x()
}