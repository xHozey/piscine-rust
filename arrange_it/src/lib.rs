pub fn arrange_phrase(phrase: &str) -> String {
    let words: Vec<&str> = phrase.split(" ").collect();
     let mut res = vec!["".to_string(); words.len()];
    for word in words {
        let mut nb = String::new();
        let mut clean_word = String::new();
        for c in word.chars() {
            if c.is_numeric() {
                nb.push(c)
            } else {
                clean_word.push(c)
            }
        } 
        res[nb.parse::<usize>().unwrap()-1] = clean_word; 
    }
    res.join(" ")
}