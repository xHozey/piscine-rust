use edit_distance::*;
use convert_case::*;

pub fn expected_variable(compare: &str, expected: &str) -> Option<String> {
    let lower_c = compare.to_lowercase();
    let lower_e = expected.to_lowercase();
    if !lower_c.is_case(Case::Snake) && !lower_c.is_case(Case::Camel) {
        return None;
    }
    let cost: i32 = edit_distance(&lower_c, &lower_e) as i32;

    let res = 100 - (100 * cost as isize) / expected.len() as isize; 
    if res > 50 {
        Some(format!("{}%", res))
    } else {
        None
    }
}

