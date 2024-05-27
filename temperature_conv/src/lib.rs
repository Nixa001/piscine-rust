// pub fn fahrenheit_to_celsius(f: f64)->f64{
//     let conv = 5.0 / 9.0;
//     conv * (f - 32.0)
// }
pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    ((f - 32.0) * 5.0) / 9.0
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * (9.0 / 5.0)) + 32.0
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = fahrenheit_to_celsius(-459.67);
        let result2 = celsius_to_fahrenheit(0.0);
        assert_eq!(result, -273.15 );
        // assert_eq!(result2, 32.0 );
    }
}
