use std::sync::{Arc, Mutex};
use crate::shop::model::ShoppingCart;

#[derive(Clone)]
pub struct SessionController {
    cart: Arc<Mutex<ShoppingCart>>,
}

impl SessionController {
    pub fn new() -> Self {
        Self { 
            cart: Arc::new(Mutex::new(ShoppingCart::new()))
        }
    }

    pub fn cart(&self) -> std::sync::MutexGuard<'_, ShoppingCart> {
        self.cart.lock().unwrap()
    }
}
