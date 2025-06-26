
pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let mut r = vec![];
    for nb in s.split_whitespace() {
        let mut clean_nb = String::new();
        let mut multiply = false;
        for c in nb.chars() {
            if c.is_numeric() {
                clean_nb.push(c)
            } else if c == 'k' {
                multiply = true
            }
        }
        r.push(if multiply {
            clean_nb.parse::<u32>().unwrap() * 1000
        } else {
            clean_nb.parse::<u32>().unwrap()
        });
    }
    Box::new(r)
}
pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    *a
}