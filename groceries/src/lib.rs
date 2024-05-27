pub fn insert(vec: &mut Vec<String>, val: String) {
    vec.push(val);
    // println!("{}",vec);
}

pub fn at_index(vec: &Vec<String>, index: usize) -> String {
    vec[index].to_string()
}

// fn main(){
//     let mut groceries = vec![
//             "yogurt".to_string(),
//             "panettone".to_string(),
//             "bread".to_string(),
//             "cheese".to_string(),
//         ];

//         insert(&mut groceries, String::from("nuts"))
// }
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut groceries = vec![
            "yogurt".to_string(),
            "panettone".to_string(),
            "bread".to_string(),
            "cheese".to_string(),
        ];
        let result = insert(&mut groceries, String::from("nuts"));
        assert_eq!(result, 4);
    }
}
