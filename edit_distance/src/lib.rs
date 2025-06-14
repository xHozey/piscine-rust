pub fn edit_distance(source: &str, target: &str) -> usize {
    calc(source, target, source.len(), target.len())
}

fn calc(source: &str, target: &str, slen: usize, tlen: usize) -> usize {
    if slen == 0 {
        return tlen
    }
    if tlen == 0 {
        return slen
    }
    if source.chars().nth(slen-1).unwrap() == target.chars().nth(tlen-1).unwrap() {
        return calc(source, target, slen-1, tlen-1)
    }
    let first = calc(source, target, slen, tlen-1);
    let second = calc(source, target, slen-1, tlen);
    let third = calc(source, target, slen-1, tlen-1);
    1+first.min(second).min(third)
}