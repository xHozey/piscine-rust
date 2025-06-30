use std::collections::HashMap;
use chrono::*;

pub fn commits_per_week(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut hashmap = HashMap::new();
    for obj in data.members() {
        let date = obj["commit"]["author"]["date"].to_string().parse::<DateTime<Utc>>().unwrap();
        let iso_week = date.iso_week();
        let week_str = format!("{}-W{:02}", iso_week.year(), iso_week.week());
        *hashmap.entry(week_str).or_insert(0) += 1;
    }
    hashmap
}

pub fn commits_per_author(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut hashmap = HashMap::new();
    for obj in data.members() {
        *hashmap.entry(obj["author"]["login"].to_string()).or_insert(0) += 1;
    }
    hashmap
}