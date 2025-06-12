pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    f - 32 / (9 as f64/5 as f64)
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * (9 as f64/5 as f64) + 32
}