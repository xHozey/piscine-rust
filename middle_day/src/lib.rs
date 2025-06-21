pub use chrono::prelude::*;

pub fn middle_day(year: u32) -> Option<chrono::Weekday> {
    if (year % 4 == 0) && (year % 100 != 0 || year % 400 == 0) {
        return None;
    }
    Some(NaiveDate::from_ymd_opt(year as i32, 7, 2).unwrap().weekday())
}