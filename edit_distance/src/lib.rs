use std::collections::HashMap;

pub fn edit_distance(source: &str, target: &str) -> usize {
    let mut memo = HashMap::new();
    calc(source, target, source.len(), target.len(), &mut memo)
}

fn calc(
    source: &str, target: &str, slen: usize, tlen: usize, memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if slen == 0 {
        return tlen;
    }
    if tlen == 0 {
        return slen;
    }

    if let Some(&res) = memo.get(&(slen, tlen)) {
        return res;
    }

    let res = if source.chars().nth(slen - 1).unwrap() == target.chars().nth(tlen - 1).unwrap() {
        calc(source, target, slen - 1, tlen - 1, memo)
    } else {
        let insert = calc(source, target, slen, tlen - 1, memo);
        let delete = calc(source, target, slen - 1, tlen, memo);
        let replace = calc(source, target, slen - 1, tlen - 1, memo);
        1 + insert.min(delete).min(replace)
    };

    memo.insert((slen, tlen), res);
    res
}
