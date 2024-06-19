pub fn first_fifty_even_square() -> Vec<i32> {
    (1..)
        .filter(|x| x % 2 == 0)  // Filter to get even numbers
        .map(|x| x * x)          // Square each even number
        .take(50)                // Take the first 50 results
        .collect()               // Collect the results into a Vec<i32>
}
