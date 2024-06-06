pub fn stars(n: u32) -> String {
    let mut asterisks = String::new();
    for _ in 0..2_i32.pow(n) {
        asterisks.push('*');
    }
    asterisks
}
