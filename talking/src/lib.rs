pub fn talking(text: &str) -> &str {
    if text.trim().is_empty() {
        return "Just say something!"
    }
    let filtered_chars: Vec<_> = text.chars().filter(|c| c.is_alphabetic()).collect();
    let mut yelling = false;
    if !filtered_chars.is_empty() && filtered_chars.iter().all(|c| c.is_uppercase()) {
        yelling = true
    }

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
        _ => {
            if yelling {
                return "There is no need to yell, calm down!"
            }
            return "Interesting"
        }
    }

}