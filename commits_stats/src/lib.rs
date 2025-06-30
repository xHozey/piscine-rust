use std::collections::HashMap;
use chrono::*;

pub fn commits_per_week(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut hashmap = HashMap::new();
    for obj in data.members() {
        *hashmap.entry(obj["commit"]["author"]["date"].to_string().parse::<DateTime<Utc>>().unwrap().format("%Y-W%W").to_string()).or_insert(1) += 1;
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