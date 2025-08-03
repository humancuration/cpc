//! Example of using the data generator module

use feedback_showcase::data_generator::{DataGeneratorConfig, generate_reviews, generate_survey_responses, generate_federated_reviews};
use feedback_showcase::data_generator::utils::create_default_config;
use feedback_showcase::data_generator::generators::products::generate_product;
use survey::{Survey, Question};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Data Generator Usage Example ===\n");
    
    // 1. Create configuration
    let config = create_default_config();
    println!("✓ Created default configuration with {} reviews", config.review_count);
    
    // 2. Generate sample product
    let product_type = &config.product_types[0];
    let product = generate_product(product_type);
    println!("✓ Generated product: {}", product.name);
    
    // 3. Generate reviews
    let mut reviews = generate_reviews(&config, product.clone());
    println!("✓ Generated {} reviews", reviews.len());
    
    // 4. Validate reviews
    feedback_showcase::data_generator::utils::validate_reviews(&reviews)?;
    println!("✓ Validated all reviews");
    
    // 5. Create a survey
    let survey = Survey {
        id: uuid::Uuid::new_v4(),
        title: "Product Feedback".to_string(),
        description: "Tell us about your experience".to_string(),
        questions: vec![
            Question::StarRating {
                min: 0.0,
                max: 5.0,
                step: 0.5,
            },
            Question::TextResponse {
                max_length: Some(500),
            }
        ],
        scoring_config: None,
    };
    
    // 6. Generate survey responses
    let survey_responses = generate_survey_responses(&config, &survey, &mut reviews);
    println!("✓ Generated {} survey responses", survey_responses.len());
    
    // 7. Generate federated reviews
    let federated_reviews = generate_federated_reviews(reviews);
    println!("✓ Generated {} federated reviews", federated_reviews.len());
    
    // 8. Display some sample data
    println!("\n=== Sample Data ===");
    println!("Product: {}", product.name);
    println!("Review sample: {}", federated_reviews[0].local_review.title);
    println!("Survey responses: {}", survey_responses.len());
    
    println!("\n=== Example Complete ===");
    Ok(())
}