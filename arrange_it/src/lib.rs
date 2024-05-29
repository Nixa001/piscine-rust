pub fn arrange_phrase(phrase: &str) -> String {
    let mut sent_vec = vec![String::new(); phrase.split(" ").count()];
    for (i, word) in phrase.split(" ").enumerate() {
        let mut index = 0;
        let mut temp_word = String::new();
        for char in word.chars() {
            let result: Result<i32, _> = char.to_string().parse();
            match result {
                Ok(value) => index = value-1,
                Err(_) => temp_word.push(char),
            }
        }
        sent_vec[index as usize] = temp_word;
    }
    sent_vec.join(" ")
}

// fn main() {
//     let arranged_phrase = arrange_phrase("is2 Thi1s T4est 3a");
//     println!("{}", arranged_phrase);
// }
// And its output

// $ cargo run
// "This is a Test"
// $