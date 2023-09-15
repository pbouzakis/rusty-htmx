use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Product {
    pub slug: String,
    pub display: String,
    pub image_src: String,
    pub price: f32,
}

pub struct ShoppingCart {
    items: Vec<CartItem>,
}

impl ShoppingCart {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
        }
    }

    pub fn count(&self) -> usize {
        self.items.len()
    }

    pub fn add(&mut self, product: &Product) -> usize {
        if let Some(i) = self.position_of_slug(&product.slug) {
            let cart_item = &self.items[i];
            let quantity = cart_item.quantity + 1;

            self.items[i] = CartItem {
                product: product.clone(),
                quantity,
                total: product.price * quantity as f32,
            }

        } else {
            self.items.push(CartItem {
                product: product.clone(),
                quantity: 1,
                total: product.price,
            });
        }

        self.count()
    }

    pub fn items(&self) -> &Vec<CartItem> {
       &self.items
    }

    fn position_of_slug(&self, slug: &str)-> Option<usize> {
        self.items.iter().position(|p| p.product.slug == slug)
    }
}


#[derive(Debug, Serialize, Clone)]
pub struct CartItem {
    pub product: Product,
    pub quantity: usize,
    pub total: f32,
}

#[cfg(test)]
mod tests {
    fn new_dummy_product() -> super::Product {
        super::Product {
            slug: "test-slug".into(),
            display: "test display".into(),
            image_src: "https://image".into(),
            price: 25.00,
        }
    }

    #[test]
    fn empty_cart() {
        let cart = super::ShoppingCart::new();
        
        assert_eq!(cart.count(), 0);
    }
    
    #[test]
    fn adding_to_cart_to_empty_cart() {
        let mut cart = super::ShoppingCart::new();

        cart.add(&new_dummy_product());

        assert_eq!(cart.count(), 1);
    }

    #[test]
    fn adding_to_cart_to_non_empty_cart() {
        let mut cart = super::ShoppingCart::new();
        cart.add(&new_dummy_product());

        let product = super::Product {
            slug: "test-slug-two".into(),
            display: "test display two".into(),
            image_src: "https://image-two".into(),
            price: 25.99,
        };

        cart.add(&product);

        assert_eq!(cart.count(), 2);
    }

    #[test]
    fn adding_the_same_sku_twice_does_not_add_new_item() {
        let mut cart = super::ShoppingCart::new();
        cart.add(&new_dummy_product());
        cart.add(&new_dummy_product());

        assert_eq!(cart.count(), 1, "Count should not increment");
        assert_eq!(cart.items()[0].quantity, 2, "Quantiy should increment");
        assert_eq!(cart.items()[0].total, 50.00, "Total should be twice the price");
    }         
}
