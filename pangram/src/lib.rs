pub fn is_pangram(s: &str) -> bool {
    let alphabet = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];
    let mut found_let = Vec::new();

    for char in s.to_lowercase().chars() {
        if alphabet.contains(&char) && !found_let.contains(&char) {
            found_let.push(char);
        }
    }
    found_let.len() == alphabet.len()
}
