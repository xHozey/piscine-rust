pub fn rotate(input: &str, key: i8) -> String {
    let mut res = String::new();
    for c in input.chars() {
        if c.is_alphabetic() {
            if c.is_uppercase() {
                res.push((65 +((c as u8 - 65) as i8 + key).rem_euclid(26)) as u8 as char);
            } else {
                res.push((97 + ((c as u8 - 97) as i8 + key).rem_euclid(26)) as u8 as char);
            }
            continue
        }
        res.push(c)
    }
    res
}


