pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (c, (c as f64).exp(), (c.abs() as f64).ln())
}
pub fn str_function(a: String) -> (String, String) {
    let mut result = String::new();

    for c in a.chars() {
        if let Some(digit) = c.to_digit(10) {
            let exp_value = digit as f64;
            let exp_result = f64::exp(exp_value);

            result.push_str(&exp_result.to_string());
            result.push(' ');
        }
    }

    (a, result.trim().to_string())
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let log = b.iter()
                      .map(|&val| (val.abs() as f64).ln())
                      .collect::<Vec<f64>>();
    (b.clone(), log)
}