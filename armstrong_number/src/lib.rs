pub fn is_armstrong_number(nb: u32) -> Option<u32>{
    let num_str = nb.to_string();
    let power = num_str.len() as u32;
    let mut sum = 0;

    for c in num_str.chars() {
        let digit = c.to_digit(10).expect(" ");
        sum += digit.pow(power);
    }

    // println!("{}", sum);
    if sum == nb {
        Some(sum)
    } else {
        None
    }
}



// fn main() {
//     // is_armstrong_number(1);
//     // is_armstrong_number(153);
//     // is_armstrong_number(400)
//     println!("{:?}", is_armstrong_number(0));
//     println!("{:?}", is_armstrong_number(1));
//     println!("{:?}", is_armstrong_number(153));
//     println!("{:?}", is_armstrong_number(370));
//     println!("{:?}", is_armstrong_number(371));
//     println!("{:?}", is_armstrong_number(407));
//     println!("{:?}", is_armstrong_number(400));
//     println!("{:?}", is_armstrong_number(198));
// }
// And its output:

// $ cargo run
// Some(0)
// Some(1)
// Some(153)
// Some(370)
// Some(371)
// Some(407)
// None
// None
// $  for digit in num_str.chars().map(|c| c.to_digit(10).unwrap()) {
       