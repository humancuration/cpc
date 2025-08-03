//! Utility functions for the data generator module

use crate::data_generator::config::DataGeneratorConfig;
use rand::Rng;

/// Create a default configuration for the data generator
pub fn create_default_config() -> DataGeneratorConfig {
    DataGeneratorConfig {
        review_count: 100,
        survey_response_rate: 0.8,
        rating_distribution: create_default_rating_distribution(),
        demographic_distribution: create_default_demographic_distribution(),
        product_types: create_default_product_types(),
    }
}

/// Create default rating distribution configuration
fn create_default_rating_distribution() -> crate::data_generator::config::RatingDistributionConfig {
    crate::data_generator::config::RatingDistributionConfig {
        mean: 0.75,
        std_dev: 0.15,
        min: 0.0,
        max: 1.0,
    }
}

/// Create default demographic distribution configuration
fn create_default_demographic_distribution() -> crate::data_generator::config::DemographicConfig {
    crate::data_generator::config::DemographicConfig {
        age_groups: vec![
            ("18-24".to_string(), 0.2),
            ("25-34".to_string(), 0.3),
            ("35-44".to_string(), 0.25),
            ("45-54".to_string(), 0.15),
            ("55+".to_string(), 0.1),
        ],
        genders: vec![
            ("male".to_string(), 0.4),
            ("female".to_string(), 0.4),
            ("non-binary".to_string(), 0.1),
            ("other".to_string(), 0.1),
        ],
        locations: vec![
            ("New York, NY".to_string(), 0.15),
            ("Los Angeles, CA".to_string(), 0.12),
            ("Chicago, IL".to_string(), 0.08),
            ("Houston, TX".to_string(), 0.07),
            ("Phoenix, AZ".to_string(), 0.05),
            ("Philadelphia, PA".to_string(), 0.05),
            ("San Antonio, TX".to_string(), 0.04),
            ("San Diego, CA".to_string(), 0.04),
            ("Dallas, TX".to_string(), 0.04),
            ("San Jose, CA".to_string(), 0.03),
        ],
        occupations: vec![
            ("Software Engineer".to_string(), 0.15),
            ("Teacher".to_string(), 0.1),
            ("Healthcare Worker".to_string(), 0.1),
            ("Student".to_string(), 0.15),
            ("Sales Representative".to_string(), 0.08),
            ("Manager".to_string(), 0.08),
            ("Designer".to_string(), 0.07),
            ("Engineer".to_string(), 0.07),
            ("Marketing Specialist".to_string(), 0.05),
            ("Other".to_string(), 0.15),
        ],
    }
}

/// Create default product types configuration
fn create_default_product_types() -> Vec<crate::data_generator::config::ProductTypeConfig> {
    vec![
        crate::data_generator::config::ProductTypeConfig {
            name: "Water Bottle".to_string(),
            description: "A reusable water bottle designed for everyday use.".to_string(),
            weight: 0.3,
            common_attributes: vec![
                ("material".to_string(), "stainless steel".to_string()),
                ("capacity".to_string(), "24 oz".to_string()),
            ],
        },
        crate::data_generator::config::ProductTypeConfig {
            name: "Backpack".to_string(),
            description: "A durable backpack for daily use or travel.".to_string(),
            weight: 0.2,
            common_attributes: vec![
                ("material".to_string(), "nylon".to_string()),
                ("compartments".to_string(), "3".to_string()),
            ],
        },
        crate::data_generator::config::ProductTypeConfig {
            name: "Headphones".to_string(),
            description: "High-quality headphones for music and calls.".to_string(),
            weight: 0.2,
            common_attributes: vec![
                ("type".to_string(), "wireless".to_string()),
                ("battery_life".to_string(), "20 hours".to_string()),
            ],
        },
        crate::data_generator::config::ProductTypeConfig {
            name: "Fitness Tracker".to_string(),
            description: "A wearable device to track your fitness activities.".to_string(),
            weight: 0.15,
            common_attributes: vec![
                ("water_resistant".to_string(), "true".to_string()),
                ("display".to_string(), "color".to_string()),
            ],
        },
        crate::data_generator::config::ProductTypeConfig {
            name: "Coffee Maker".to_string(),
            description: "A programmable coffee maker for your kitchen.".to_string(),
            weight: 0.15,
            common_attributes: vec![
                ("capacity".to_string(), "12 cups".to_string()),
                ("programmable".to_string(), "true".to_string()),
            ],
        },
    ]
}

/// Validate that generated reviews pass validation
pub fn validate_reviews<T: reviews::Entity>(reviews: &[reviews::Review<T>]) -> Result<(), feedback_core::FeedbackError> {
    for review in reviews {
        review.validate().map_err(|e| feedback_core::FeedbackError::Validation(e.to_string()))?;
    }
    Ok(())
}

/// Create sample reviews for previewing visualizations
pub fn create_sample_reviews() -> Vec<reviews::Review<crate::data_generator::generators::products::Product>> {
    use crate::data_generator::generators::products::Product;
    use reviews::{Review, Rating, RatingMethod, Attribute, Demographics};
    use uuid::Uuid;
    use chrono::Utc;
    
    let products = vec![
        Product {
            id: Uuid::new_v4(),
            name: "Premium Water Bottle".to_string(),
            description: "A high-quality reusable water bottle.".to_string(),
        },
        Product {
            id: Uuid::new_v4(),
            name: "Smart Fitness Tracker".to_string(),
            description: "A wearable device to track your fitness activities.".to_string(),
        },
        Product {
            id: Uuid::new_v4(),
            name: "Wireless Headphones".to_string(),
            description: "High-quality headphones for music and calls.".to_string(),
        },
    ];
    
    let sample_reviews = vec![
        Review {
            id: Uuid::new_v4(),
            entity: products[0].clone(),
            user_id: Uuid::new_v4(),
            title: "Excellent Product!".to_string(),
            content: "This water bottle is fantastic! Keeps drinks cold for hours and the design is sleek. Highly recommend to anyone looking for a reliable hydration solution.".to_string(),
            ratings: vec![
                Rating {
                    metric: "overall".to_string(),
                    value: 0.9,
                    unit: Some("%".to_string()),
                    method: RatingMethod::UserReported,
                },
                Rating {
                    metric: "quality".to_string(),
                    value: 0.95,
                    unit: Some("%".to_string()),
                    method: RatingMethod::UserReported,
                },
                Rating {
                    metric: "value".to_string(),
                    value: 0.85,
                    unit: Some("%".to_string()),
                    method: RatingMethod::UserReported,
                },
                Rating {
                    metric: "design".to_string(),
                    value: 0.9,
                    unit: Some("%".to_string()),
                    method: RatingMethod::UserReported,
                },
            ],
            attributes: vec![
                Attribute {
                    key: "pros".to_string(),
                    value: "durable construction, eco-friendly materials, excellent value".to_string(),
                },
            ],
            demographics: Some(Demographics {
                age_group: "25-34".to_string(),
                gender: "female".to_string(),
                location: "San Francisco, CA".to_string(),
                occupation: Some("Software Engineer".to_string()),
            }),
            survey_response: None,
            created_at: Utc::now(),
        },
        Review {
            id: Uuid::new_v4(),
            entity: products[1].clone(),
            user_id: Uuid::new_v4(),
            title: "Good but could be better".to_string(),
            content: "The fitness tracker works well for basic activity tracking, but the battery life is shorter than expected. The app interface is intuitive and syncs reliably with my phone.".to_string(),
            ratings: vec![
                Rating {
                    metric: "overall".to_string(),
                    value: 0.7,
                    unit: Some("%".to_string()),
                    method: RatingMethod::UserReported,
                },
                Rating {
                    metric: "quality".to_string(),
                    value: 0.75,
                    unit: Some("%".to_string()),
                    method: RatingMethod::UserReported,
                },
                Rating {
                    metric: "value".to_string(),
                    value: 0.65,
                    unit: Some("%".to_string()),
                    method: RatingMethod::UserReported,
                },
                Rating {
                    metric: "design".to_string(),
                    value: 0.8,
                    unit: Some("%".to_string()),
                    method: RatingMethod::UserReported,
                },
            ],
            attributes: vec![
                Attribute {
                    key: "pros".to_string(),
                    value: "intuitive interface, reliable performance".to_string(),
                },
                Attribute {
                    key: "cons".to_string(),
                    value: "short battery life, limited features".to_string(),
                },
            ],
            demographics: Some(Demographics {
                age_group: "18-24".to_string(),
                gender: "male".to_string(),
                location: "Austin, TX".to_string(),
                occupation: Some("Student".to_string()),
            }),
            survey_response: None,
            created_at: Utc::now(),
        },
        Review {
            id: Uuid::new_v4(),
            entity: products[2].clone(),
            user_id: Uuid::new_v4(),
            title: "Disappointing quality".to_string(),
            content: "These headphones sounded great initially, but after just two months the left earpiece stopped working. Customer service was unhelpful and the warranty process was frustrating.".to_string(),
            ratings: vec![
                Rating {
                    metric: "overall".to_string(),
                    value: 0.3,
                    unit: Some("%".to_string()),
                    method: RatingMethod::UserReported,
                },
                Rating {
                    metric: "quality".to_string(),
                    value: 0.2,
                    unit: Some("%".to_string()),
                    method: RatingMethod::UserReported,
                },
                Rating {
                    metric: "value".to_string(),
                    value: 0.25,
                    unit: Some("%".to_string()),
                    method: RatingMethod::UserReported,
                },
                Rating {
                    metric: "design".to_string(),
                    value: 0.4,
                    unit: Some("%".to_string()),
                    method: RatingMethod::UserReported,
                },
            ],
            attributes: vec![
                Attribute {
                    key: "cons".to_string(),
                    value: "fragile components, average durability".to_string(),
                },
            ],
            demographics: Some(Demographics {
                age_group: "35-44".to_string(),
                gender: "non-binary".to_string(),
                location: "Seattle, WA".to_string(),
                occupation: Some("Designer".to_string()),
            }),
            survey_response: None,
            created_at: Utc::now(),
        },
        Review {
            id: Uuid::new_v4(),
            entity: products[0].clone(),
            user_id: Uuid::new_v4(),
            title: "Perfect for daily use!".to_string(),
            content: "I use this water bottle every day at work and it's perfect. Keeps my water cold and fits nicely in my bag. The lid seals well and there are no leaks.".to_string(),
            ratings: vec![
                Rating {
                    metric: "overall".to_string(),
                    value: 0.95,
                    unit: Some("%".to_string()),
                    method: RatingMethod::UserReported,
                },
                Rating {
                    metric: "quality".to_string(),
                    value: 0.9,
                    unit: Some("%".to_string()),
                    method: RatingMethod::UserReported,
                },
                Rating {
                    metric: "value".to_string(),
                    value: 0.9,
                    unit: Some("%".to_string()),
                    method: RatingMethod::UserReported,
                },
                Rating {
                    metric: "design".to_string(),
                    value: 0.95,
                    unit: Some("%".to_string()),
                    method: RatingMethod::UserReported,
                },
            ],
            attributes: vec![
                Attribute {
                    key: "pros".to_string(),
                    value: "sleek design, easy to use, reliable performance".to_string(),
                },
            ],
            demographics: Some(Demographics {
                age_group: "45-54".to_string(),
                gender: "female".to_string(),
                location: "Boston, MA".to_string(),
                occupation: Some("Manager".to_string()),
            }),
            survey_response: None,
            created_at: Utc::now(),
        },
        Review {
            id: Uuid::new_v4(),
            entity: products[1].clone(),
            user_id: Uuid::new_v4(),
            title: "Decent tracker for the price".to_string(),
            content: "This fitness tracker is okay for basic activity monitoring. The step counting seems accurate, but the heart rate monitor is a bit inconsistent. Good value for the price point.".to_string(),
            ratings: vec![
                Rating {
                    metric: "overall".to_string(),
                    value: 0.65,
                    unit: Some("%".to_string()),
                    method: RatingMethod::UserReported,
                },
                Rating {
                    metric: "quality".to_string(),
                    value: 0.7,
                    unit: Some("%".to_string()),
                    method: RatingMethod::UserReported,
                },
                Rating {
                    metric: "value".to_string(),
                    value: 0.8,
                    unit: Some("%".to_string()),
                    method: RatingMethod::UserReported,
                },
                Rating {
                    metric: "design".to_string(),
                    value: 0.6,
                    unit: Some("%".to_string()),
                    method: RatingMethod::UserReported,
                },
            ],
            attributes: vec![
                Attribute {
                    key: "pros".to_string(),
                    value: "good value, accurate step counting".to_string(),
                },
                Attribute {
                    key: "cons".to_string(),
                    value: "inconsistent heart rate monitor".to_string(),
                },
            ],
            demographics: Some(Demographics {
                age_group: "25-34".to_string(),
                gender: "male".to_string(),
                location: "Denver, CO".to_string(),
                occupation: Some("Engineer".to_string()),
            }),
            survey_response: None,
            created_at: Utc::now(),
        },
    ];
    
    sample_reviews
}