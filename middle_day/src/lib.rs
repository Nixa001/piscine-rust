extern crate chrono;
use chrono::{Datelike, NaiveDate, Weekday as wd};

pub fn middle_day(year: i32) -> Option<wd> {
    let last_day = NaiveDate::from_ymd_opt(year, 12, 31)?;
    let days_in_y = last_day.ordinal();
    if days_in_y % 2 != 0 {
        NaiveDate::from_ymd_opt(year, 12, 31)?
            .checked_add_signed(chrono::Duration::days(days_in_y as i64 / 2))
            .map(|d| d.Weekday())
    } else {
        None
    }
}

fn main() {
    println!("{:?}", middle_day(1022).unwrap());
}

// And its output:

// $ cargo run
// Tue
// $
