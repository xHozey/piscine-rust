pub fn capitalize_first(input: &str) -> String {
    if input.is_empty() {
        return "".to_string()
    }
    input[0..1].to_ascii_uppercase() + &input[1..]
}

pub fn title_case(input: &str) -> String {
    let mut space = false;
    let mut res = String::new();
    for (i, c) in input.chars().enumerate() {
        if space || i == 0 {
            res.push(c.to_ascii_uppercase());
            space = false
        } else {
            res.push(c.to_ascii_lowercase());
        }
        if c.is_whitespace() {
            space = true
        }
    }
    res
}

pub fn change_case(input: &str) -> String {
    let mut res = String::new();
    for c in input.chars() {
        if c.is_alphabetic() {
            if c.is_lowercase() {
                res.push(c.to_ascii_uppercase())
            } else {
                res.push(c.to_ascii_lowercase())
            }
        } else {
            res.push(c)
        }
    }
    res
}