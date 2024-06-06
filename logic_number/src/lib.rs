pub fn number_logic(num: u32) -> bool {
    let num_str = num.to_string();
    let power = num_str.len() as u32;
    let mut res = 0;
    for c in num_str.chars() {
        let digit = c.to_digit(10).expect(" ");
        res += digit.pow(power);
    }
    return res == num
}