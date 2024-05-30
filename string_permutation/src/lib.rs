use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    fn char_count(s: &str) -> HashMap<char, usize> {
        let mut count = HashMap::new();
        for c in s.chars() {
            *count.entry(c).or_insert(0) += 1;
        }
        count
    }

    let count1 = char_count(s1);
    let count2 = char_count(s2);

    count1 == count2
}

fn main() {
    let word = "thought";
    let word1 = "thougth";
    println!(
        "Is `{}` a permutation of `{}`? = {}",
        word,
        word1,
        is_permutation(word, word1)
    );
}
