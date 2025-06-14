use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
    list.iter().sum::<i32>() as f64 / list.len() as f64 
}

pub fn median(list: &[i32]) -> i32 {
    let mut cl = list.to_vec();
    cl.sort();
    if cl.len() % 2 == 0 {
        cl[cl.len()/2-2] + cl[cl.len()/2-1] / 2
    } else {
        cl[cl.len()/2]
    }
}

pub fn mode(list: &[i32]) -> i32 {
    let mut often = HashMap::new();
    for nb in list.iter() {
        *often.entry(*nb).or_insert(0) += 1;
    }
    let mut max = 0;
    let mut res = 0;
    for (key, val) in often {
        if val > max {
            max = val;
            res = key;
        }
    }
    res
}