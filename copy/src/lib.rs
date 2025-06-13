pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (c, (c as f64).exp(), (c as f64).abs().ln())
}

pub fn str_function(a: String) -> (String, String) {
    let mut res = String::new();
    for nb in a.split_whitespace() {
        res.push_str(&nb.parse::<f64>().unwrap().exp().to_string());
        res.push(' ');
    }
    (a, res.trim().to_string())
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut res = Vec::new();
    for nb in &b {
        res.push((*nb as f64).abs().ln())
    }
    (b, res)
}