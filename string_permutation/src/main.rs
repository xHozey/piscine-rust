use string_permutation::*;

fn main() {
    let word = "avcde";
    let word1 = "edbca";

    println!(
        "Is {:?} a permutation of {:?}? = {}",
        word,
        word1,
        is_permutation(word, word1)
    );
}