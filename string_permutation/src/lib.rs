use std::collections::HashMap;
pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false
    }
    let mut s1hash = HashMap::new();
    let mut s2hash = HashMap::new();
    for c in s1.chars() {
        *s1hash.entry(c).or_insert(0) += 1;
    }
    for c in s1.chars() {
        *s2hash.entry(c).or_insert(0) += 1;
    }
    for (key, val) in s1hash {
        match s2hash.get(&key) {
            Some(c) => {
                if *c != val {
                    return false
                }
            },
            None => return false
        }
    }

    true
}