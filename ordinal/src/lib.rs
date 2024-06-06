pub fn num_to_ordinal(x: u32) -> String {
    let mut num_str = x.to_string();
    if num_str == "11" || num_str == "12" || num_str == "13" {
        num_str.push_str("th");
    } else {
        let last_val = num_str.chars().nth(num_str.len() - 1).unwrap();
        match last_val {
            '1' => num_str.push_str("st"),
            '2' => num_str.push_str("nd"),
            '3' => num_str.push_str("rd"),
            _ => num_str.push_str("th"),
        }
    }
    num_str
}
