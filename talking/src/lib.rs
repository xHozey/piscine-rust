pub fn talking(text: &str) -> &str {
    if text.is_empty() {
        return "Just say something!"
    }
    let yelling = text.chars().filter(|c| c.is_alphabetic()).all(|c| c.is_uppercase());
    let mark = &text[text.len()-1..];
    match mark {
        "!" => {
            if yelling {
                return "There is no need to yell, calm down!";
            } 
            return "Interesting" 
        },
        "?" => {
            if yelling {
                return "Quiet, I am thinking!"
            } 
                return "Sure."
        },
        _ => return "Interesting"
    }

}