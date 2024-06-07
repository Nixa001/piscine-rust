pub fn scytale_cipher(s: String, size: u32) -> String {
    let mut result = String::new();
    if s.is_empty() {
        return result;
    }
    let nbr_lines = (s.len() as f32 / size as f32).ceil() as usize;
    let str_format = format!("{}{}", s, " ".repeat(nbr_lines * size as usize - s.len()));
    let chars: Vec<char> = str_format.chars().collect();
    for i in 0..size as usize {
        result.push(chars[i]);
        let mut j = i.clone();
        while j + size as usize <= str_format.len() - 1 {
            j += size as usize;
            result.push(chars[j]);
        }
    }
    result.trim().to_string()
}
