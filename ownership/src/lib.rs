pub fn first_subword(mut s: String) -> String {
    let mut temp_string = String::new();
    let mut index = true;
    for char in s.chars(){
        if (char.is_uppercase() || char =='_') && index == false{
            break
        }else{
            temp_string.push(char)
        }
        index = false;
    }
    temp_string
}

// fn main() {
// 	let s1 = String::from("helloWorld");
// 	let s2 = String::from("snake_case");
// 	let s3 = String::from("CamelCase");
// 	let s4 = String::from("just");

// 	println!("first_subword({}) = {}", s1.clone(), first_subword(s1));
// 	println!("first_subword({}) = {}", s2.clone(), first_subword(s2));
// 	println!("first_subword({}) = {}", s3.clone(), first_subword(s3));
// 	println!("first_subword({}) = {}", s4.clone(), first_subword(s4));
// }
// And its output:

// $ cargo run
// first_subword(helloWorld) = hello
// first_subword(snake_case) = snake
// first_subword(CamelCase) = Camel
// first_subword(just) = just
// $