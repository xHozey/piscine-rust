use std::{collections::HashMap, hash::Hash, iter::zip};

pub fn slices_to_map<'a, T: Eq + Hash, U: Eq + Hash>(s1: &'a [T], s2: &'a [U]) -> HashMap<&'a T, &'a U> {
    let mut res = HashMap::new();
    let zipped = zip(s1, s2);
    zipped.into_iter().for_each(|(a,b)| { res.insert( a,b); });
    res
}
