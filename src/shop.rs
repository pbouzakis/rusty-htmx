use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Product {
    slug: String,
    display: String,
    image_src: String,
    price: f32,
}

pub fn fetch_catalog() -> Vec<Product> {
    vec![
        Product {
            slug: "the-dream-machine".into(),
            display: "The Dream Machine".into(),
            image_src: "/media/covers/dream-machine.png".into(),
            price: 25.00,
        },
        Product {
            slug: "the-conquest-of-bread".into(),
            display: "The Conquest of Bread".into(),
            image_src: "/media/covers/bread.png".into(),
            price: 5.00,
        },
        Product {
            slug: "hyper-media-systems".into(),
            display: "Hypermedia Systems".into(),
            image_src: "/media/covers/hypermedia.png".into(),
            price: 10.99,
        },
        Product {
            slug: "thinking-in-systems".into(),
            display: "Thinking in Systems".into(),
            image_src: "/media/covers/systems.png".into(),
            price: 35.50,
        },
        Product  {
            slug: "extreme-programming-explained".into(),
            display: "Extreme Programming Explained".into(),
            image_src: "/media/covers/xp.png".into(),
            price: 20.00,
        },        
    ]
}
