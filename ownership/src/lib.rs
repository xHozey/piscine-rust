pub fn first_subword(mut s: String) -> String {
    s.truncate(s[1..].find(|c: char| c == '_' || c.is_ascii_uppercase()).unwrap_or(s.len())+1);
    s
}