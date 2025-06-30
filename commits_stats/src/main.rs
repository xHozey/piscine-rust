use commits_stats::*;
use std::fs;
fn main() {
	let contents = fs::read_to_string("./src/commits.json").unwrap();
	let serialized = json::parse(&contents).unwrap();
	println!("{:?}", commits_per_author(&serialized));
	println!("{:?}", commits_per_week(&serialized));

}