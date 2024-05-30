pub fn sum(a: &[i32]) -> i32 {
   a.iter().sum::<i32>()
}

pub fn thirtytwo_tens() -> [i32; 32] {
    [10; 32]
}



// fn main() {
//     let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
//     let a1: Vec<i32> = (1..11).collect();
//     let b = [5; 10]; 

//     println!("The Sum of the elements in {:?} = {}", a, sum(&a));
//     println!("The Sum of the elements in {:?} = {}", a1, sum(&a1));
//     println!("The Sum of the elements in {:?} = {}", b, sum(&b));

//     println!(
//         "Array size {} with only 10's in it {:?}",
//         thirtytwo_tens().len(),
//         thirtytwo_tens()
//     );
// }
// And its output:

// $ cargo run
// The Sum of the elements in [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] = 55
// The Sum of the elements in [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] = 55
// The Sum of the elements in [5, 5, 5, 5, 5, 5, 5, 5, 5, 5] = 50
// Array size 32 with only 10's in it [10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10]
// $