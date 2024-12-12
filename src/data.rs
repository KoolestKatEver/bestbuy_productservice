use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Dell XPS 15.6\" Laptop (Intel Core i7-13620H/16GB RAM/512GB SSD/Win11)".to_string(),
            price: 1399.99            ,
            description: "Enjoy the ultimate convenience of portable computing with this Dell Inspiron 16-inch laptop. Equipped with Intel Core i7-13620H and 16GB RAM, this laptop enables effortless multitasking. The 512GB solid state drive provides reliable storage for all your files.".to_string(),
            image: "/dell.png".to_string()
        }
        
    ]
}