pub fn sum(a: i8, b: i8) -> i8 {
    a + b
}

pub fn diff(a: u16, b: u16) -> u16 {
    a - b
}

pub fn pro(a: i8, b: i8) -> i8 {
    a * b
}

pub fn quo(a: u32, b: u32) -> u32 {
    a / b
}

pub fn rem(a: u32, b: u32) -> u32 {
    a % b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = quo(2, 2);
        assert_eq!(result, 1);
    }
}
