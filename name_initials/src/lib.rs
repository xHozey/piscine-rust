pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut res = Vec::new();
    for name in names {
        let mut p = String::new();
        for s in name.split_whitespace() {
            p.push(s.chars().nth(0).unwrap());
            p.push_str(". ");
        }
        
        res.push(p.trim().to_string())
    }
    res
}