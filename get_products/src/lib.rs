pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
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
