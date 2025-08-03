//! Configuration structures for the data generator

use serde::{Deserialize, Serialize};

/// Configuration for the data generator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataGeneratorConfig {
    /// Number of reviews to generate
    pub review_count: usize,
    
    /// Survey response rate (0.0 to 1.0)
    pub survey_response_rate: f32,
    
    /// Rating distribution configuration
    pub rating_distribution: RatingDistributionConfig,
    
    /// Demographic distribution configuration
    pub demographic_distribution: DemographicConfig,
    
    /// Product types to generate
    pub product_types: Vec<ProductTypeConfig>,
}

/// Configuration for rating distributions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatingDistributionConfig {
    /// Mean rating value (0.0 to 1.0)
    pub mean: f32,
    
    /// Standard deviation for ratings
    pub std_dev: f32,
    
    /// Minimum rating value
    pub min: f32,
    
    /// Maximum rating value
    pub max: f32,
}

/// Configuration for demographic distributions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemographicConfig {
    /// Age group distribution weights
    pub age_groups: Vec<(String, f32)>,
    
    /// Gender distribution weights
    pub genders: Vec<(String, f32)>,
    
    /// Location distribution weights
    pub locations: Vec<(String, f32)>,
    
    /// Occupation distribution weights
    pub occupations: Vec<(String, f32)>,
}

/// Configuration for product types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductTypeConfig {
    /// Product type name
    pub name: String,
    
    /// Product type description
    pub description: String,
    
    /// Weight for random selection
    pub weight: f32,
    
    /// Common attributes for this product type
    pub common_attributes: Vec<(String, String)>,
}