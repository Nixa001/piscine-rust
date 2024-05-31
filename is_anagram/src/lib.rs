use std::collections::HashMap;

pub fn is_anagram(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut chars1: Vec<char> = s1.chars().collect();
    let mut chars2: Vec<char> = s2.chars().collect();
    chars1.sort_unstable();
    chars2.sort_unstable();
    chars1.into_iter().collect::<String>() == chars2.into_iter().collect::<String>()
}
