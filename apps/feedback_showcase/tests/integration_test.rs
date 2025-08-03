//! Integration tests for the feedback showcase application

use feedback_showcase::data_generator::utils::create_default_config;
use feedback_showcase::data_generator::generators::products::generate_product;
use feedback_showcase::data_generator::generate_reviews;

#[test]
fn test_data_generation_pipeline() {
    // Create default configuration
    let config = create_default_config();
    
    // Generate a sample product
    let product_type = &config.product_types[0];
    let product = generate_product(product_type);
    
    // Generate reviews
    let reviews = generate_reviews(&config, product);
    
    // Verify we have the expected number of reviews
    assert_eq!(reviews.len(), config.review_count);
    
    // Verify all reviews pass validation
    for review in &reviews {
        assert!(review.validate().is_ok());
    }
    
    println!("✓ Generated {} reviews", reviews.len());
    println!("✓ All reviews passed validation");
}

#[test]
fn test_config_creation() {
    let config = create_default_config();
    
    // Verify basic config properties
    assert_eq!(config.review_count, 100);
    assert_eq!(config.survey_response_rate, 0.8);
    assert!(!config.product_types.is_empty());
    
    // Verify rating distribution config
    assert_eq!(config.rating_distribution.mean, 0.75);
    assert_eq!(config.rating_distribution.std_dev, 0.15);
    
    // Verify demographic distribution config has data
    assert!(!config.demographic_distribution.age_groups.is_empty());
    assert!(!config.demographic_distribution.genders.is_empty());
    assert!(!config.demographic_distribution.locations.is_empty());
    assert!(!config.demographic_distribution.occupations.is_empty());
    
    println!("✓ Default configuration created successfully");
    println!("✓ Configuration contains {} product types", config.product_types.len());
}