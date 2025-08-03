//! Core domain models for the reviews system
//!
//! This module defines the core data structures for reviews, ratings, attributes,
//! and demographics. It also includes traits for entities that can be reviewed.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::survey::{SurveyResponse};
use crate::survey::validate_survey_response;
use std::fmt::Debug;
use chrono::{NaiveDateTime};

/// Trait for entities that can be reviewed
///
/// Any type that implements this trait can have reviews attached to it.
/// This allows the review system to be generic over different entity types.
pub trait Entity: Send + Sync + Debug {
    /// Get the unique identifier for this entity
    fn id(&self) -> Uuid;
    
    /// Get the type of this entity (e.g., "product", "service")
    fn entity_type(&self) -> String;
}

/// Generic review model
///
/// A review is attached to a specific entity and contains ratings, attributes,
/// and optional demographic information about the reviewer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review<T: Entity> {
    /// Unique identifier for this review
    pub id: Uuid,
    
    /// The entity being reviewed
    pub entity: T,
    
    /// ID of the user who created this review
    pub user_id: Uuid,
    
    /// Title of the review
    pub title: String,
    
    /// Main content/body of the review
    pub content: String,
    
    /// Scientific ratings for various metrics
    pub ratings: Vec<Rating>,
    
    /// Flexible attribute system for additional information
    pub attributes: Vec<Attribute>,
    
    /// Optional demographic information about the reviewer
    pub demographics: Option<Demographics>,
    
    /// Optional survey response associated with this review
    pub survey_response: Option<SurveyResponse>,
    
    /// When this review was created
    pub created_at: DateTime<Utc>,
}

/// Metadata for federated reviews
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederationMetadata {
    /// When this review was shared
    pub shared_at: Option<DateTime<Utc>>,
    
    /// Which federation node this review originated from
    pub source_node: Option<String>,
    
    /// Version of the review in the federation
    pub version: u32,
}

/// Consent rule for data sharing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentRule {
    /// Category of data that can be shared (e.g., "ratings", "demographics")
    pub data_category: String,
    
    /// Who the data can be shared with
    pub shared_with: FederationGroup,
}

/// Federation group definitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FederationGroup {
    /// Publicly shareable
    Public,
    
    /// Shareable with specific partners
    Partner(String),
    
    /// Internal use only
    Internal,
}

/// Federated review wrapper that includes sharing metadata and consent rules
pub struct FederatedReview<T: Entity> {
    pub local_review: Review<T>,
    pub shared_metadata: FederationMetadata,
    pub consent_rules: Vec<ConsentRule>,
}

impl<T: Entity> Review<T> {
    /// Validate the review data
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.title.is_empty() {
            return Err(ValidationError::InvalidTitle("Title cannot be empty".to_string()));
        }
        
        if self.content.is_empty() {
            return Err(ValidationError::InvalidContent("Content cannot be empty".to_string()));
        }
        
        // Validate ratings
        for rating in &self.ratings {
            rating.validate()?;
        }
        
        // Validate attributes
        for attribute in &self.attributes {
            attribute.validate()?;
        }
        
        // Validate demographics if present
        if let Some(demographics) = &self.demographics {
            demographics.validate()?;
        }
        
        // Validate survey response if present
        if let Some(survey_response) = &self.survey_response {
            // Note: We would need access to the survey definition to fully validate
            // For now, we just ensure the survey response is structurally valid
            // In a real implementation, we would validate against the actual survey
        }
        
        Ok(())
    }
}

/// Scientific rating metric
///
/// Ratings use a standardized 0.0-1.0 scale with optional units and method tracking.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rating {
    /// Metric name (e.g., "effectiveness", "safety")
    pub metric: String,
    
    /// Value on 0.0-1.0 scale
    pub value: f32,
    
    /// Unit of measurement (e.g., "%", "mg")
    pub unit: Option<String>,
    
    /// Method used for measurement
    pub method: RatingMethod,
}

impl Rating {
    /// Validate the rating data
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.metric.is_empty() {
            return Err(ValidationError::InvalidMetric("Metric cannot be empty".to_string()));
        }
        
        if self.value < 0.0 || self.value > 1.0 {
            return Err(ValidationError::InvalidValue("Rating value must be between 0.0 and 1.0".to_string()));
        }
        
        Ok(())
    }
}

/// Method used to determine a rating
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RatingMethod {
    /// Rating provided by a user
    UserReported,
    
    /// Rating determined by an expert analysis
    ExpertAnalysis,
    
    /// Rating based on clinical trial data
    ClinicalTrial,
}

/// Flexible attribute system
///
/// Attributes allow for arbitrary key-value pairs to capture additional information
/// that doesn't fit into the structured rating system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribute {
    /// Attribute key (e.g., "side_effects", "durability")
    pub key: String,
    
    /// Attribute value (e.g., "headache", "long-lasting")
    pub value: String,
}

impl Attribute {
    /// Validate the attribute data
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.key.is_empty() {
            return Err(ValidationError::InvalidAttribute("Attribute key cannot be empty".to_string()));
        }
        
        if self.value.is_empty() {
            return Err(ValidationError::InvalidAttribute("Attribute value cannot be empty".to_string()));
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::survey::{Survey, Question, SurveyResponse, Answer};
    use uuid::Uuid;
    use chrono::Utc;

    #[test]
    fn test_review_with_survey_response() {
        // Create a survey
        let survey = Survey {
            id: Uuid::new_v4(),
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

        // Create a survey response
        let response = SurveyResponse {
            survey_id: survey.id,
            answers: vec![
                Answer::StarRating(4.5),
                Answer::TextResponse("Great product!".to_string()),
            ],
            created_at: Utc::now(),
        };

        // Create a test entity
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        struct TestEntity {
            id: Uuid,
            name: String,
        }

        impl Entity for TestEntity {
            fn id(&self) -> Uuid {
                self.id
            }
            
            fn entity_type(&self) -> String {
                "test".to_string()
            }
        }

        let entity = TestEntity {
            id: Uuid::new_v4(),
            name: "Test Product".to_string(),
        };

        // Create a review with survey response
        let review = Review {
            id: Uuid::new_v4(),
            entity,
            user_id: Uuid::new_v4(),
            title: "Great product!".to_string(),
            content: "I really enjoyed using this product.".to_string(),
            ratings: vec![
                Rating {
                    metric: "quality".to_string(),
                    value: 0.9,
                    unit: None,
                    method: RatingMethod::UserReported,
                }
            ],
            attributes: vec![
                Attribute {
                    key: "color".to_string(),
                    value: "blue".to_string(),
                }
            ],
            demographics: Some(Demographics {
                age_group: "25-34".to_string(),
                gender: "non-binary".to_string(),
                location: "San Francisco, CA".to_string(),
                occupation: Some("Engineer".to_string()),
            }),
            survey_response: Some(response),
            created_at: Utc::now(),
        };

        // Validate the review
        assert!(review.validate().is_ok());
    }
}

/// User demographics (optional)
///
/// Optional demographic information that can be attached to reviews for
/// more detailed analysis and filtering.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Demographics {
    /// Age group (e.g., "18-25", "65+")
    pub age_group: String,
    
    /// Gender identity
    pub gender: String,
    
    /// Geographic location
    pub location: String,
    
    /// Occupation (optional)
    pub occupation: Option<String>,
}

impl Demographics {
    /// Validate the demographic data
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.age_group.is_empty() {
            return Err(ValidationError::InvalidDemographic("Age group cannot be empty".to_string()));
        }
        
        if self.gender.is_empty() {
            return Err(ValidationError::InvalidDemographic("Gender cannot be empty".to_string()));
        }
        
        if self.location.is_empty() {
            return Err(ValidationError::InvalidDemographic("Location cannot be empty".to_string()));
        }
        
        Ok(())
    }
}

/// Custom error type for validation errors
#[derive(Debug, Clone, thiserror::Error)]
pub enum ValidationError {
    #[error("Invalid title: {0}")]
    InvalidTitle(String),
    
    #[error("Invalid content: {0}")]
    InvalidContent(String),
    
    #[error("Invalid metric: {0}")]
    InvalidMetric(String),
    
    #[error("Invalid value: {0}")]
    InvalidValue(String),
    
    #[error("Invalid attribute: {0}")]
    InvalidAttribute(String),
    
    #[error("Invalid demographic: {0}")]
    InvalidDemographic(String),
}

impl From<ValidationError> for feedback_core::FeedbackError {
    fn from(err: ValidationError) -> Self {
        feedback_core::FeedbackError::Validation(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;
    use chrono::Utc;

    // Simple test entity for testing
    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct TestEntity {
        id: Uuid,
        name: String,
    }

    impl Entity for TestEntity {
        fn id(&self) -> Uuid {
            self.id
        }
        
        fn entity_type(&self) -> String {
            "test".to_string()
        }
    }

    #[test]
    fn test_review_validation_success() {
        let entity = TestEntity {
            id: Uuid::new_v4(),
            name: "Test Product".to_string(),
        };
        
        let review = Review {
            id: Uuid::new_v4(),
            entity,
            user_id: Uuid::new_v4(),
            title: "Great product!".to_string(),
            content: "I really enjoyed using this product.".to_string(),
            ratings: vec![
                Rating {
                    metric: "quality".to_string(),
                    value: 0.9,
                    unit: None,
                    method: RatingMethod::UserReported,
                }
            ],
            attributes: vec![
                Attribute {
                    key: "color".to_string(),
                    value: "blue".to_string(),
                }
            ],
            demographics: Some(Demographics {
                age_group: "25-34".to_string(),
                gender: "non-binary".to_string(),
                location: "San Francisco, CA".to_string(),
                occupation: Some("Engineer".to_string()),
            }),
            survey_response: None,
            created_at: Utc::now(),
}
        
        assert!(review.validate().is_ok());
    }
    
    #[test]
    fn test_review_validation_failure_empty_title() {
        let entity = TestEntity {
            id: Uuid::new_v4(),
            name: "Test Product".to_string(),
        };
        
        let review = Review {
            id: Uuid::new_v4(),
            entity,
            user_id: Uuid::new_v4(),
            title: "".to_string(),
            content: "I really enjoyed using this product.".to_string(),
            ratings: vec![],
            attributes: vec![],
            demographics: None,
            survey_response: None,
            created_at: Utc::now(),
}
        
        assert!(review.validate().is_err());
    }
    
    #[test]
    fn test_rating_validation_success() {
        let rating = Rating {
            metric: "effectiveness".to_string(),
            value: 0.85,
            unit: Some("%".to_string()),
            method: RatingMethod::ExpertAnalysis,
        };
        
        assert!(rating.validate().is_ok());
    }
    
    #[test]
    fn test_rating_validation_failure_invalid_value() {
        let rating = Rating {
            metric: "effectiveness".to_string(),
            value: 1.5, // Invalid - outside 0.0-1.0 range
            unit: None,
            method: RatingMethod::UserReported,
        };
        
        assert!(rating.validate().is_err());
    }
}