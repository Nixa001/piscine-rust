pub fn arrange_phrase(phrase: &str) -> String {
    let mut sent_vec = vec![String::new(); phrase.split_whitespace().count()];
    
    for word in phrase.split_whitespace() {
        let mut index = 0;
        let mut temp_word = String::new();
        
        for char in word.chars() {
            if let Some(value) = char.to_digit(10) {
                index = value as usize - 1; 
            } else {
                temp_word.push(char);
            }
        }
        
        sent_vec[index] = temp_word;
    }
    
    sent_vec.join(" ")
}


fn main() {
    let arranged_phrase = arrange_phrase("is2 Thi1s T4est 3a");
    println!("{}", arranged_phrase);
}
// And its output

// $ cargo run
// "This is a Test"
// $