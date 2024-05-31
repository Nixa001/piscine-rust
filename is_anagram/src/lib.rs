use std::collections::HashMap;

pub fn is_anagram(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut char_count: HashMap<char, usize> = HashMap::new();
    for c in s1.chars() {
        *char_count.entry(c).or_insert(0) += 1;
    }

    for c in s2.chars() {
        if let Some(count) = char_count.get_mut(&c) {
            *count -= 1;
            if *count == 0 {
                char_count.remove(&c);
            }
        } else {
            return false;
        }
    }

    char_count.is_empty()
}
