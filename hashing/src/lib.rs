use std::collections::HashMap;

pub fn mean(list: &Vec<i32>) -> f64 {
    let sum: i32 = list.iter().sum();
    sum as f64 / list.len() as f64
}

pub fn median(list: &Vec<i32>) -> i32 {
    let mut sorted_list = list.clone();
    sorted_list.sort();

    let len = sorted_list.len();
    if len % 2 == 0 {
        (sorted_list[len / 2 - 1] + sorted_list[len / 2]) / 2
    } else {
        sorted_list[len / 2]
    }
}

pub fn mode(list: &Vec<i32>) -> i32 {
    let mut occurrences = HashMap::new();

    for &value in list.iter() {
        *occurrences.entry(value).or_insert(0) += 1;
    }

    occurrences.into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("impossibl")
}


// fn main() {
//     let v = vec![4, 7, 5, 2, 5, 1, 3];
//     println!("mean {}", mean(&v));
//     println!("median {}", median(&v));
//     println!("mode {}", mode(&v));
// }
