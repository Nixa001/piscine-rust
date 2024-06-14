pub use std::collections::HashMap;
pub use chrono::{DateTime, Datelike};
pub use json::JsonValue;

pub fn commits_per_week(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut commits_per_week: HashMap<String, u32> = HashMap::new();

    for commit in data.members() {
        let commit_date_str = Some(commit["commit"]["author"]["date"].as_str()).unwrap();
        let commit_date = DateTime::parse_from_rfc3339(commit_date_str.unwrap()).unwrap();
        let week_number = commit_date.iso_week().week();
        let year = commit_date.year();
        let week_representation = format!("{}-W{}", year, week_number);

        *commits_per_week.entry(week_representation).or_insert(0) += 1;
    }

    commits_per_week
}

pub fn commits_per_author(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut commits_per_author: HashMap<String, u32> = HashMap::new();
    for commit in data.members() {
        let author = Some(commit["author"]["login"].as_str()).unwrap();
        *commits_per_author.entry(author.unwrap().to_string()).or_insert(0) += 1;
    }

    commits_per_author
}
