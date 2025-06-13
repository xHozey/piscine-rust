pub fn is_empty(v: &str) -> bool {
    if v.is_empty() {
        return true
    }
    false
}

pub fn is_ascii(v: &str) -> bool {
    if v.is_ascii() {
        return true
    }
    false
}

pub fn contains(v: &str, pat: &str) -> bool {
    if v.contains(pat) {
        return true
    }
    false
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    v.split_at(index)
}

pub fn find(v: &str, pat: char) -> usize {
    v.find(pat).unwrap()
}