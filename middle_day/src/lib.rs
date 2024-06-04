extern crate chrono;
use chrono::{Datelike, NaiveDate, Weekday };

pub fn middle_day(year: i32) -> Option<Weekday> {
    let is_bissextil = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);

    if is_bissextil {
        return None;
    }

    let middle_date = NaiveDate::from_ymd(year, 7, 2);
    Some(middle_date.weekday())
}

fn main() {
    println!("{:?}", middle_day(1022).unwrap());
}

// And its output:

// $ cargo run
// Tue
// $