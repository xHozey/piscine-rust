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
    pub receipt: Vec<f32>,
}
impl Cart {
    pub fn new() -> Cart {
        Self {
            items: Vec::new(),
            receipt: Vec::new(),
        }
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
        let total_dis = self.items.len() / 3;
        self.items.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        let discount: f32 = self.items[0..total_dis].iter().map(|(_, v)| *v).sum();
        let total: f32 = self.items.iter().map(|(_, val)| *val).sum();            
        self.items.iter().for_each(|(_, val)| res.push(((val - (discount * (val / total))) * 100.).round() / 100.));
        self.receipt = res.clone();
        res
    }
}
