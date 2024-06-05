pub use case::CaseExt;
pub use edit_distance::edit_distance;
pub fn expected_variable(input: &str, expected: &str) -> Option<String> {
    if input == "" || input.contains("-") || input.contains(" "){
        return None
    }
    let input1= format!("{}{}",input.split_at(1).0.to_lowercase(),input.split_at(1).1);
    if !input1.is_camel_lowercase()  && ! input.contains("_"){
        return None;
    }

    let distance = edit_distance(input.to_lowercase().as_str(), expected.to_lowercase().as_str());
    let max_length = expected.chars().count().max(input.chars().count()) as f64;
    let similar = 1.0 - (distance as f64 / max_length);

    if similar >= 0.5 {
        Some(format!("{}%", (similar * 100.0).round()))
    } else {
        None
    }
} 

fn main() {
    println!(
        "{} close to it",
        expected_variable("On_Point", "on_point").unwrap()
    );
    println!(
        "{} close to it",
        expected_variable("soClose", "so_close").unwrap()
    );
    println!(
        "{:?}",
        expected_variable("something", "something_completely_different")
    );
    println!(
        "{} close to it",
        expected_variable("BenedictCumberbatch", "BeneficialCucumbersnatch").unwrap()
    );
}
// And its output:

// $ cargo run
// 100% close to it
// 88% close to it
// None
// 67% close to it
// $