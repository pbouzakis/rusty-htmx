use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Product {
    pub slug: String,
    pub display: String,
    pub image_src: String,
    pub price: f32,
}

pub struct Cart {
    pub items: Vec<String>,
}

#[derive(Serialize)]
pub struct CartItem {
    pub product: Product,
    pub quantity: usize,
    pub total: f32,
}
