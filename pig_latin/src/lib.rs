pub fn pig_latin(text: &str) -> String {
    let vowels = "aeiou";
    if vowels.contains(&text[0..1]) {
        return text.to_string() + "ay"
    }
    if !vowels.contains(&text[0..1]) && &text[1..3] == "qu" {
        return text[3..].to_string() + &format!("{}quay", &text[0..1])
    }
    for (i, c) in text.char_indices() {
        if vowels.contains(c) {
            return text[i..].to_string() + &format!("{}ay", &text[0..i])
        }
    }

    return "".to_string()
}