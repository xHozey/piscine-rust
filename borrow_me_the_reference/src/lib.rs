pub fn delete_and_backspace(s: &mut String) {
    let mut i = 0;
    while i < s.len() {
        match s.chars().nth(i) {
            Some(c) => {
                if c == '-' {
                    s.remove(i);
                    i = i.saturating_sub(1);
                    if s.len() == 0 {
                        break;
                    }
                    s.remove(i);
                    i = i.saturating_sub(1);
                }
            }
            None => break,
        }
        i += 1;
    }

    i = s.len() - 1;
    while s.len() > 0 {
        match s.chars().nth(i) {
            Some(c) => {
                if c == '+' {
                    s.remove(i);
                    if i < s.len() {
                        s.remove(i);
                    }
                }
                if i == 0 {
                    break;
                }
                i -= 1;
            }
            None => {
                break;
            }
        }
    }
}

pub fn do_operations(v: &mut [String]) {
    for s in v {
        let mut res = 0;
        if s.contains('-') {
            for (i, nb) in s.split('-').enumerate() {
                if i == 0 {
                    res = nb.parse::<i32>().unwrap() 
                } else {
                    res -= nb.parse::<i32>().unwrap() 
                }
            }

        } else {
            for nb in s.split('+') {
                res += nb.parse::<i32>().unwrap()
            }
        }
        s.clear();
        s.push_str(&res.to_string())
    }
}