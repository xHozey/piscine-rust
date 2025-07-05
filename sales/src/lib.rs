#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}
impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<(String, f32)>,
}
impl Cart {
    pub fn new() -> Cart {
        Self { items: Vec::new() }
    }
    pub fn insert_item(&mut self, s: &Store, ele: String) {
        for el in s.products.clone() {
            if el.0 == ele {
                self.items.push(el);
            }
        }
    }
    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut res = Vec::new();
        self.items.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        self.items.chunks_exact(3).into_iter().for_each(|chunk| {
            if chunk.len() == 3 {
                let (_, min) = *chunk
                    .into_iter()
                    .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                    .unwrap();
                let mut total: f32 = 0.0;
                chunk.iter().for_each(|(_, v)| total += v);
                chunk.iter().for_each(|(_, val)| {
                    let after_discount: f32 = val - (min * (val / total));
                    res.push((after_discount * 100.0).round() / 100.0);
                });
            } else {
                chunk.iter().for_each(|(_, val)| {
                    res.push((val *100.).round() / 100.);
                });
            }
        });
        res
    }
}
