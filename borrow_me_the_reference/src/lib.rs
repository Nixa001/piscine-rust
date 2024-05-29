pub fn delete_and_backspace(s: &mut String) {
    let mut str_res = String::new();
    let mut str_first = String::new();
    for char in s.chars() {
        if char == '-' {
            str_first.pop();
            continue;
        }
        str_first.push(char);
    }
    for char in str_first.chars().rev() {
        if char == '+' {
            str_res.pop();
            continue;
        }
        str_res.push(char);
    }
    *s = str_res.chars().rev().collect();
}


pub fn do_operations(v: &mut Vec<String>) {
    let mut res_Vec = vec![];

    for val in &mut *v {
        let temp: Vec<&str> = if val.contains("-") {
            val.split("-").collect()
        } else {
            val.split("+").collect()
        };

        let mut first_value = 0;
        let mut second_value = 0;

        let result: Result<i32, _> = temp[0].to_string().parse();
        match result {
            Ok(value) => first_value = value,
            Err(e) => println!(" {}", e),
        }

        let result: Result<i32, _> = temp[1].to_string().parse();
        match result {
            Ok(value) => second_value = value,
            Err(e) => println!(" {}", e),
        }

        let result = if val.contains("-") {
            first_value - second_value
        } else {
            first_value + second_value
        };

        res_Vec.push(result.to_string());
    }
    *v = res_Vec;

    // println!("{:?}", );
}

// fn main() {
// 	let mut a = String::from("bpp--o+er+++sskroi-++lcw");
// 	// let mut b: Vec<String> = vec!["2+2", "3+2", "10-3", "5+5"];
//     let mut b: Vec<String> = vec![
//     "2+2".to_string(), 
//     "3+2".to_string(), 
//     "10-3".to_string(), 
//     "5+5".to_string(),
// ];

// 	delete_and_backspace(&mut a);
// 	do_operations(&mut b);

// 	println!("{:?}", (a, b));
// }
// // And its output

// // $ cargo run
// // ("borrow", ["4", "5", "7", "10"])
// // $