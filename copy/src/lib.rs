pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (c, (c as f64).exp(), (c.abs() as f64).ln())
}
pub fn str_function(a: String) -> (String, String) {
    let exp = a.chars()
                      .map(|ch| (ch as u8 as f64).exp().to_string())
                      .collect::<Vec<String>>()
                      .join(" ");
    (a.clone(), exp)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let log = b.iter()
                      .map(|&val| (val.abs() as f64).ln())
                      .collect::<Vec<f64>>();
    (b.clone(), log)
}