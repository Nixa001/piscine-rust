pub fn score(str: &str) -> u64 {
    let str_upper = str.to_uppercase();
    let mut sum: u64 = 0;
    for char in str_upper.chars() {
        match char {
            'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => sum += 1,
            'D' | 'G' => sum += 2,
            'B' | 'C' | 'M' | 'P' => sum += 3,
            'F' | 'H' | 'V' | 'W' | 'Y' => sum += 4,
            'K' => sum += 5,
            'J' | 'X' => sum += 8,
            'Q' | 'Z' => sum += 10,
            _ => sum += 0,
        }
    }
    sum
}
