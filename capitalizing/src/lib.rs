pub fn capitalize_first(input: &str) -> String {
let mut str = String::new();
let mut is_first = true;
for char in input.chars(){
    if is_first{
       str.push(char.to_ascii_uppercase());
       is_first = false;
    }else{
       str.push(char);
    }
}
str.to_string()
}

pub fn title_case(input: &str) -> String {
    let mut str = String::new();
    let mut is_first = true;
    for char in input.chars(){
        if is_first{
        str.push(char.to_ascii_uppercase());
        is_first = false;
        }else{
        str.push(char);
        }
        if char == ' '{
            is_first = true;
        }
    }
    str.to_string()
}

pub fn change_case(input: &str) -> String {
    let mut str = String::new();
    for char in input.chars(){
        if char.is_ascii_lowercase(){
        str.push(char.to_ascii_uppercase());
        }else{
            str.push(char.to_ascii_lowercase());
        }
    }
    str.to_string()
}


fn main() {
    println!("{}", capitalize_first("joe is missing"));
    println!("{}", title_case("jill is leaving A"));
    println!("{}",change_case("heLLo THere"));
}
// And its output

// $ cargo run
// Joe is missing
// Jill Is Leaving A
// HEllO thERE
// $