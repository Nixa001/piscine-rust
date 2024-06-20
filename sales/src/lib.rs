#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}

impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Self {
        Self { products }
    }

    pub fn get_price(&self, product_name: &str) -> Option<(String , f32)> {
        for (name, price) in &self.products {
            if name == product_name {
                return Some((name.clone(), *price));
            }
        }
        None
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}

impl Cart {
    pub fn new() -> Cart {
        Cart {
            items: Vec::new(),
            receipt: Vec::new(),
        }
    }

    pub fn insert_item(&mut self, store: &Store, item_name: String) {
        if let Some(price) = store.get_price(&item_name) {
            self.items.push(price);
        }
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
        self.items.sort_by(|a, b| a.partial_cmp(b).unwrap());
        // Apply the "buy three, get one free" promotion
        // for chunk in self.items.chunks(3) {
        //     if chunk.len() == 3 {
        //         total_discount += chunk[0].1;
        //     }
        // }
        let free = self.items.len()/3;
        let mut prices: Vec<f32> = self.items.iter().map(|c| c.1).collect::<Vec<f32>>();
        
        let total : f32 = prices.iter().sum();
        prices.sort_by(|a , b| a.total_cmp(b));
        let free_prices = prices.get(0..free).unwrap();
        let free_total :f32 = free_prices.iter().sum();
        
        // Calculate the total amount to be adjusted
        // let total_amount: f32 = self.items.iter().sum() ;
        let adjustment =  1.0- free_total/total ;
        // println!("---------{}", adjustment);
        self.receipt = prices.iter().map(|price| (price * adjustment)).collect();

        // Round each value to two decimal places
        for val in &mut self.receipt {
            *val = (*val * 100.0).round() / 100.0;
        }

        self.receipt.clone()
    }
}


