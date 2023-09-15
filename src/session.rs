use std::sync::{Arc, Mutex};
use crate::shop::{
    gateway,
    model::{Cart, CartItem}
};

#[derive(Clone)]
pub struct SessionController {
    cart: Arc<Mutex<Cart>>,
}

impl SessionController {
    pub fn new() -> Self {
        Self { 
            cart: Arc::new(
                Mutex::new(Cart {
                    items: vec![],
                })
            )
         }
    }
    pub fn cart_count(&self) -> usize {
        let cart = self.cart.lock().unwrap();
        cart.items.len()
    }
    pub fn update_cart(&self, sku: String) -> usize {
        let mut cart = self.cart.lock().unwrap();

        cart.items.push(sku);

        println!("{:#?}", cart.items);

        cart.items.len()
    }
    pub fn cart_items(&self) -> Vec<CartItem> {
        let catalog = gateway::fetch_catalog();
        let cart = self.cart.lock().unwrap();
        let mut items = vec![];

        for product in catalog {
            if cart.items.contains(&product.slug) {
                let quantity = cart.items.iter().filter(|&slug| *slug == product.slug).count();
                let total = product.price.clone() * quantity as f32;

                items.push(
                    CartItem {
                        product,
                        quantity,
                        total,
                    }
                )
            }
        }
        items
    }
}
