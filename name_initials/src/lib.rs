pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut initials: Vec<String> = vec![];
    
    for name in names {
        let mut temp_string = String::new();
        let name_split = name.split_whitespace();
        let mut index = 0;
        for part in name_split {
            let first_char = part.chars().nth(0).unwrap();
            temp_string.push(first_char);
            temp_string.push('.');
            if index ==0{
                temp_string.push(' ');
                index+=1;
            }
        }
        
        initials.push(temp_string);
    }
    initials
}

fn main() {
    let names = vec!["Harry Potter", "Someone Else", "J. L.", "Barack Obama"];
    println!("{:?}", initials(names));
}

// And its output

// $ cargo run
// ["H. P.", "S. E.", "J. L.", "B. O."]
// $