pub fn stars(n: u32) -> String {
    if n == 0 {
        return "*".to_string()
    }
    "*".repeat(n.pow(2) as usize)
}