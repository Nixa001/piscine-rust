pub fn is_anagram(s1: &str, s2: &str) -> bool {    
    if s1.len() != s2.len() {
        return false;
    }

    let mut s1_chars: Vec<char> = s1.chars().collect();
    let mut s2_chars: Vec<char> = s2.chars().collect();

    s1_chars.sort_unstable();
    s2_chars.sort_unstable();

    s1_chars == s2_chars
}
