//! Federation generator for the data generator module

use crate::data_generator::generators::products::Product;
use reviews::{Review, FederatedReview, FederationMetadata, ConsentRule, FederationGroup};
use chrono::Utc;
use rand::Rng;
use rayon::prelude::*;

/// Generate federated reviews from regular reviews
pub fn generate_federated_reviews(reviews: Vec<Review<Product>>) -> Vec<FederatedReview<Product>> {
    if reviews.len() > 10000 {
        // Use streaming generation for large datasets
        generate_federated_reviews_streaming(reviews)
    } else {
        // Use parallel generation for smaller datasets
        generate_federated_reviews_parallel(reviews)
    }
}

/// Generate federated reviews using parallel processing
fn generate_federated_reviews_parallel(reviews: Vec<Review<Product>>) -> Vec<FederatedReview<Product>> {
    reviews
        .into_par_iter()
        .map(generate_single_federated_review)
        .collect()
}

/// Generate federated reviews using streaming for large datasets
fn generate_federated_reviews_streaming(reviews: Vec<Review<Product>>) -> Vec<FederatedReview<Product>> {
    let mut federated_reviews = Vec::with_capacity(reviews.len());
    
    for review in reviews {
        federated_reviews.push(generate_single_federated_review(review));
    }
    
    federated_reviews
}

/// Generate a single federated review
fn generate_single_federated_review(review: Review<Product>) -> FederatedReview<Product> {
    let shared_metadata = generate_federation_metadata();
    let consent_rules = generate_consent_rules();
    
    FederatedReview {
        local_review: review,
        shared_metadata,
        consent_rules,
    }
}

/// Generate federation metadata
fn generate_federation_metadata() -> FederationMetadata {
    let mut rng = rand::thread_rng();
    
    // Randomly decide if this review is shared
    let is_shared = rng.gen_bool(0.7); // 70% chance of being shared
    
    FederationMetadata {
        shared_at: if is_shared { Some(Utc::now()) } else { None },
        source_node: if is_shared {
            let nodes = vec![
                "node-1.example.com",
                "node-2.example.com",
                "node-3.example.com",
                "research-institute.example.com",
                "community-platform.example.com"
            ];
            Some(nodes[rng.gen_range(0..nodes.len())].to_string())
        } else {
            None
        },
        version: 1,
    }
}

/// Generate consent rules for data sharing
fn generate_consent_rules() -> Vec<ConsentRule> {
    let mut rng = rand::thread_rng();
    let mut rules = Vec::new();
    
    // Always allow ratings to be shared publicly
    rules.push(ConsentRule {
        data_category: "ratings".to_string(),
        shared_with: FederationGroup::Public,
    });
    
    // Randomly decide about demographics
    if rng.gen_bool(0.6) { // 60% chance
        let group = if rng.gen_bool(0.7) {
            // 70% of shared demographics go to partners
            let partners = vec![
                "research-institute.example.com",
                "analytics-partner.example.com",
                "market-research.example.com"
            ];
            FederationGroup::Partner(partners[rng.gen_range(0..partners.len())].to_string())
        } else {
            // 30% are public
            FederationGroup::Public
        };
        
        rules.push(ConsentRule {
            data_category: "demographics".to_string(),
            shared_with: group,
        });
    }
    
    // Randomly decide about attributes
    if rng.gen_bool(0.4) { // 40% chance
        let group = match rng.gen_range(0..3) {
            0 => FederationGroup::Public,
            1 => {
                let partners = vec![
                    "research-institute.example.com",
                    "product-development.example.com"
                ];
                FederationGroup::Partner(partners[rng.gen_range(0..partners.len())].to_string())
            },
            _ => FederationGroup::Internal,
        };
        
        rules.push(ConsentRule {
            data_category: "attributes".to_string(),
            shared_with: group,
        });
    }
    
    rules
}