//! Product generator for the data generator module

use crate::data_generator::config::ProductTypeConfig;
use reviews::Entity;
use uuid::Uuid;
use fake::Fake;
use rand::seq::SliceRandom;

/// Product entity for demonstration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

impl Entity for Product {
    fn id(&self) -> Uuid {
        self.id
    }
    
    fn entity_type(&self) -> String {
        "product".to_string()
    }
}

/// Generate a random product based on configuration
pub fn generate_product(config: &ProductTypeConfig) -> Product {
    let product_name = generate_product_name(&config.name);
    let product_description = generate_product_description(&config.description);
    
    Product {
        id: Uuid::new_v4(),
        name: product_name,
        description: product_description,
    }
}

/// Generate a realistic product name
fn generate_product_name(category: &str) -> String {
    let adjectives: Vec<&str> = vec![
        "Premium", "Eco-Friendly", "Advanced", "Smart", "Innovative",
        "Compact", "Durable", "Lightweight", "Professional", "Ultimate"
    ];
    
    let nouns: Vec<&str> = vec![
        "Water Bottle", "Backpack", "Headphones", "Smartphone", "Laptop",
        "Coffee Maker", "Fitness Tracker", "Camera", "Speaker", "Watch"
    ];
    
    let adjective = adjectives.choose(&mut rand::thread_rng()).unwrap_or(&"Premium");
    let noun = nouns.choose(&mut rand::thread_rng()).unwrap_or(&"Product");
    
    format!("{} {} {}", adjective, category, noun)
}

/// Generate a realistic product description
fn generate_product_description(base_description: &str) -> String {
    let features: Vec<&str> = vec![
        "crafted from sustainable materials",
        "designed for maximum durability",
        "featuring cutting-edge technology",
        "with ergonomic design for comfort",
        "backed by our satisfaction guarantee",
        "tested for quality assurance",
        "with a sleek modern aesthetic",
        "optimized for everyday use",
        "featuring intuitive controls",
        "with premium finish and construction"
    ];
    
    let feature = features.choose(&mut rand::thread_rng()).unwrap_or(&"high-quality construction");
    
    format!("{} This {} ensures reliability and performance.", base_description, feature)
}