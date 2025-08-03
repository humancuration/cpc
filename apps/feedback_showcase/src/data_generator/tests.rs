//! Tests for the data generator module

#[cfg(test)]
mod tests {
    use super::super::config::*;
    use super::super::generators::products::*;
    use super::super::generators::reviews::*;
    use super::super::utils::*;
    
    #[test]
    fn test_default_config_creation() {
        let config = create_default_config();
        assert_eq!(config.review_count, 100);
        assert_eq!(config.survey_response_rate, 0.8);
        assert!(!config.product_types.is_empty());
    }
    
    #[test]
    fn test_product_generation() {
        let config = create_default_config();
        let product_type = &config.product_types[0];
        let product = generate_product(product_type);
        
        assert!(!product.name.is_empty());
        assert!(!product.description.is_empty());
    }
    
    #[test]
    fn test_review_generation() {
        let config = create_default_config();
        let product_type = &config.product_types[0];
        let product = generate_product(product_type);
        let reviews = generate_reviews(&config, product);
        
        assert_eq!(reviews.len(), config.review_count);
        
        // Check that all reviews have the required fields
        for review in &reviews {
            assert!(!review.title.is_empty());
            assert!(!review.content.is_empty());
            assert!(!review.ratings.is_empty());
        }
    }
}