#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CipherError {
    validation: bool,
    expected: String,
}
impl CipherError {
    pub fn new(validation: bool, expected: String) -> CipherError {
        CipherError {
            validation,
            expected,
        }
    }
}
pub fn cipher(original: &str, ciphered: &str) -> Option<Result<bool, CipherError>> {
    let original_1 = original.to_lowercase();
    let ciphered_1 = ciphered.to_lowercase();

    let mut result = String::new(); 

    for c in original_1.chars() {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            let mirror_char = base + (25 - (c as u8 - base));
            result.push(mirror_char as char);
        } else {
            result.push(c);
        }
    }
   if original.is_empty(){
    None
   } 
   else if ciphered_1 == result {
        Some(Ok(true))
    } else {
        Some(Err(CipherError::new(false, result.to_string())))
    }
}

fn main() {
    println!("{:?}", cipher("1Hello 2world!", "1Svool 2dliow!"));
    println!("{:?}", cipher("1Hello 2world!", "svool"));
    println!("{:?}", cipher("", "svool"));
}

// And its output:

// $ cargo run
// Some(Ok(true))
// Some(Err(CipherError { validation: false, expected: "1Svool 2dliow!" }))
// None
// $
