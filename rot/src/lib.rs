pub fn rotate(input: &str, key: i8) -> String {
    let mut rotate_str = String::new();
    for char in input.chars() {
        if char.is_alphabetic() {
            let base = if char.is_ascii_uppercase() {
                b'A'
            } else {
                b'a'
            };
            if key < 0 {
                let char_rot = ((char as u8 - base + ((26 + key) as u8)) % 26 + base) as char;
                rotate_str.push(char_rot);
            } else {
                let char_rot = ((char as u8 - base + key as u8) % 26 + base) as char;
                rotate_str.push(char_rot);
            }
        } else {
            rotate_str.push(char);
        }
    }
    rotate_str
}
