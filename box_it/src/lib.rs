pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let mut vec = Vec::new();
    for word in s.split_whitespace() {
        let num = word.trim_end_matches('k').parse::<f32>().unwrap_or(0.0);
        if word.ends_with('k') && num != 0.0 {
            vec.push((num * 1000.0) as u32);
        } else {
            vec.push(num as u32);
        }
    }
    Box::new(vec)
}

pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    *a
}

// fn main() {
//     let new_str = String::from("5.5k 8.9k 32");

//     // Creating a variable and saving it on the heap
//     let a_h = transform_and_save_on_heap(new_str);
//     println!("Box value : {:?}", &a_h);
//     println!("Size occupied in the stack : {:?} bytes", std::mem::size_of_val(&a_h));

//     let a_b_v = take_value_ownership(a_h);
//     println!("Value : {:?}", &a_b_v);
//     println!("Size occupied in the stack : {:?} bytes", std::mem::size_of_val(&a_b_v));
//     // Whenever the box, in this case "a_h", goes out of scope it will be deallocated, freed
// }


// Box value : [5500, 8900, 32]
// size occupied in the stack : 8 bytes
// value : [5500, 8900, 32]
// size occupied in the stack : 24 bytes
// $