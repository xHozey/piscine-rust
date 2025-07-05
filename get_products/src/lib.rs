pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    if arr.len() == 1 {
        return Vec::new();
    }
    arr.iter()
        .enumerate()
        .map(|(idx, _)| {
            arr.iter()
                .enumerate()
                .filter(|(idx2, _)| idx != *idx2)
                .map(|(_, val)| *val)
                .product()
        })
        .collect()
}
