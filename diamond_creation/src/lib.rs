pub fn get_diamond(c: char) -> Vec<String> {
    let mut res = Vec::new();
    let offset = c as u8 - b'A';
    for i in 0..=offset {
        let outer_space = " ".repeat((offset-i) as usize);
        if i == 0 {
            res.push(format!("{}{}{}", outer_space, (b'A'+i) as char, outer_space));
        } else {
            let inner_space = " ".repeat((2*i-1) as usize);
            res.push(format!("{}{}{}{}{}", outer_space, (b'A'+i) as char, inner_space, (b'A'+i) as char, outer_space));
        }
    }
    let mut cl = res[0..res.len()-1].to_vec();
    cl.reverse();
    res.append(&mut cl);
    res
}