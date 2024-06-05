pub use json::{parse, stringify};
pub struct Food {
    pub name: String,
    pub calories: [String; 2],
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: Vec<Food>) -> json::JsonValue {
    let mut cals: f64 = 0.0;
    let mut carbs: f64 = 0.0;
    let mut proteins: f64 = 0.0;
    let mut fats: f64 = 0.0;

    for food in foods {
        let portions = food.nbr_of_portions;
        let calories = food.calories[1].replace("kcal", "").parse::<f64>().unwrap() * portions;
        let fat = food.fats * portions;
        let carb = food.carbs * portions;
        let protein = food.proteins * portions;

        cals += calories;
        carbs += carb;
        proteins += protein;
        fats += fat;
    }
    json::object! {
        "cals" => round_2_dec(cals),
        "carbs" => round_2_dec(carbs),
        "proteins" => round_2_dec(proteins),
        "fats" => round_2_dec(fats),
    }
}
fn round_2_dec(number: f64) -> f64 {
    (number * 100.0).round() / 100.0
}
