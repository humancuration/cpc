//! Review generator for the data generator module

use crate::data_generator::config::{DataGeneratorConfig, RatingDistributionConfig, DemographicConfig};
use crate::data_generator::generators::products::Product;
use reviews::{Review, Rating, RatingMethod, Attribute, Demographics};
use uuid::Uuid;
use chrono::Utc;
use fake::{Fake, faker::lorem::en::*};
use rand::distributions::{Normal, Distribution};
use rand::Rng;
use rayon::prelude::*;

/// Generate multiple reviews based on configuration
pub fn generate_reviews(config: &DataGeneratorConfig, product: Product) -> Vec<Review<Product>> {
    if config.review_count > 10000 {
        // Use streaming generation for large datasets
        generate_reviews_streaming(config, product)
    } else {
        // Use parallel generation for smaller datasets
        generate_reviews_parallel(config, product)
    }
}

/// Generate reviews using parallel processing
fn generate_reviews_parallel(config: &DataGeneratorConfig, product: Product) -> Vec<Review<Product>> {
    (0..config.review_count)
        .into_par_iter()
        .map(|_| generate_single_review(config, product.clone()))
        .collect()
}

/// Generate reviews using streaming for large datasets
fn generate_reviews_streaming(config: &DataGeneratorConfig, product: Product) -> Vec<Review<Product>> {
    let mut reviews = Vec::with_capacity(1000); // Pre-allocate for performance
    
    for _ in 0..config.review_count {
        reviews.push(generate_single_review(config, product.clone()));
        
        // In a real implementation, we might process in batches
        // For now, we just generate all at once
    }
    
    reviews
}

/// Generate a single review based on configuration
fn generate_single_review(config: &DataGeneratorConfig, product: Product) -> Review<Product> {
    let title = generate_review_title();
    let content = generate_review_content();
    let ratings = generate_ratings(&config.rating_distribution);
    let attributes = generate_attributes();
    let demographics = generate_demographics(&config.demographic_distribution);
    
    Review {
        id: Uuid::new_v4(),
        entity: product,
        user_id: Uuid::new_v4(),
        title,
        content,
        ratings,
        attributes,
        demographics,
        survey_response: None, // Will be filled in by survey generator
        created_at: Utc::now(),
    }
}

/// Generate a realistic review title
fn generate_review_title() -> String {
    let adjectives: Vec<&str> = vec![
        "Excellent", "Great", "Good", "Amazing", "Fantastic",
        "Impressive", "Satisfactory", "Decent", "Outstanding", "Superb"
    ];
    
    let nouns: Vec<&str> = vec![
        "product", "item", "purchase", "experience", "quality",
        "value", "design", "performance", "durability", "functionality"
    ];
    
    let adjective = adjectives.choose(&mut rand::thread_rng()).unwrap_or(&"Great");
    let noun = nouns.choose(&mut rand::thread_rng()).unwrap_or(&"product");
    
    format!("{} {}!", adjective, noun)
}

/// Generate realistic review content
fn generate_review_content() -> String {
    // Generate 2-5 sentences of review content
    let sentence_count = rand::thread_rng().gen_range(2..=5);
    let mut sentences = Vec::new();
    
    for _ in 0..sentence_count {
        sentences.push(Sentence(10..20).fake::<String>());
    }
    
    sentences.join(" ")
}

/// Generate ratings based on distribution configuration
fn generate_ratings(config: &RatingDistributionConfig) -> Vec<Rating> {
    let normal = Normal::new(config.mean as f64, config.std_dev as f64);
    let mut rng = rand::thread_rng();
    
    let metrics = vec!["overall", "quality", "value", "design"];
    let mut ratings = Vec::new();
    
    for metric in metrics {
        // Generate rating from normal distribution
        let mut rating_value = normal.sample(&mut rng) as f32;
        
        // Clamp to min/max values
        rating_value = rating_value.max(config.min).min(config.max);
        
        ratings.push(Rating {
            metric: metric.to_string(),
            value: rating_value,
            unit: Some("%".to_string()),
            method: RatingMethod::UserReported,
        });
    }
    
    ratings
}

/// Generate review attributes
fn generate_attributes() -> Vec<Attribute> {
    let mut attributes = Vec::new();
    
    // Pros
    let pros: Vec<&str> = vec![
        "durable construction", "eco-friendly materials", "excellent value",
        "sleek design", "easy to use", "reliable performance", "long-lasting",
        "comfortable grip", "intuitive interface", "versatile functionality"
    ];
    
    let pros_count = rand::thread_rng().gen_range(1..=3);
    let selected_pros: Vec<&str> = pros.choose_multiple(&mut rand::thread_rng(), pros_count).cloned().collect();
    
    if !selected_pros.is_empty() {
        attributes.push(Attribute {
            key: "pros".to_string(),
            value: selected_pros.join(", "),
        });
    }
    
    // Cons
    let cons: Vec<&str> = vec![
        "limited color options", "slightly heavy", "expensive",
        "difficult to clean", "short battery life", "limited features",
        "assembly required", "small buttons", "fragile components", "average durability"
    ];
    
    let cons_count = rand::thread_rng().gen_range(0..=2);
    let selected_cons: Vec<&str> = cons.choose_multiple(&mut rand::thread_rng(), cons_count).cloned().collect();
    
    if !selected_cons.is_empty() {
        attributes.push(Attribute {
            key: "cons".to_string(),
            value: selected_cons.join(", "),
        });
    }
    
    attributes
}

/// Generate demographic information based on configuration
fn generate_demographics(config: &DemographicConfig) -> Option<Demographics> {
    let mut rng = rand::thread_rng();
    
    // Select age group based on weights
    let age_group = select_weighted_item(&config.age_groups, &mut rng);
    
    // Select gender based on weights
    let gender = select_weighted_item(&config.genders, &mut rng);
    
    // Select location based on weights
    let location = select_weighted_item(&config.locations, &mut rng);
    
    // Select occupation based on weights (optional)
    let occupation = if rng.gen_bool(0.8) {
        Some(select_weighted_item(&config.occupations, &mut rng))
    } else {
        None
    };
    
    Some(Demographics {
        age_group,
        gender,
        location,
        occupation,
    })
}

/// Select an item based on weighted probabilities
fn select_weighted_item<T: Clone>(items: &[(T, f32)], rng: &mut impl Rng) -> T {
    if items.is_empty() {
        panic!("Cannot select from empty item list");
    }
    
    let total_weight: f32 = items.iter().map(|(_, weight)| weight).sum();
    let mut random_value = rng.gen_range(0.0..total_weight);
    
    for (item, weight) in items {
        if random_value < *weight {
            return item.clone();
        }
        random_value -= weight;
    }
    
    // Fallback to first item
    items[0].0.clone()
}