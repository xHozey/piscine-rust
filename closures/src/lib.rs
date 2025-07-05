pub fn first_fifty_even_square() -> Vec<i32> {
    let x = || {
        let mut res = Vec::new();
        let mut acc = 4;
        while res.len() < 50 {
            if acc % 2 == 0 {
                res.push(acc);
            }
            acc += 1;
        }
        return res
    };
    x()
}