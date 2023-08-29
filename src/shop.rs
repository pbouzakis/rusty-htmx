use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Product {
    slug: String,
    display: String,
    image_src: String,
}

pub fn fetch_catalog() -> Vec<Product> {
    vec![
        Product {
            slug: "the-dream-machine".into(),
            display: "The Dream Machine".into(),
            image_src: "/images/covers/dream-machine.png".into(),
        },
        Product {
            slug: "the-conquest-of-bread".into(),
            display: "The Conquest of Bread".into(),
            image_src: "/images/covers/bread.png".into(),
        },
        Product {
            slug: "hyper-media-systems".into(),
            display: "Hypermedia Systems".into(),
            image_src: "/images/covers/hypermedia.png".into(),
        },
        Product {
            slug: "thinking-in-systems".into(),
            display: "Thinking in Systems".into(),
            image_src: "/images/covers/systems.png".into(),
        },
        Product  {
            slug: "extreme-programming-explained".into(),
            display: "Extreme Programming Explained".into(),
            image_src: "/images/covers/xp.png".into(),
        },        
    ]
}
