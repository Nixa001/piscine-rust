use std::io;

fn main() {
    let riddle = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";
    let answer = "The letter e";
    
    let mut tentative = 0;
    loop{
        let mut input = String::new();
        println!("{}",riddle);
        tentative += 1;
        let _ = io::stdin().read_line(&mut input);
        if input.trim() == answer{
            println!("Number of trials: {}", tentative);
            break
        }
    }
}
