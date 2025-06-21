pub fn is_pangram(s: &str) -> bool {
    for i in 'a'..='z' {
        if !s.to_lowercase().contains(i) {
            return false
        }
    }
    true
}