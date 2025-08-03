//! # Survey Module
//! 
//! A reusable survey system that integrates with the existing review system.
//! 
//! ## Features
//! - Multiple question types (StarRating, TextResponse, MultipleChoice, LikertScale, Matrix)
//! - Survey response validation
//! - Statistical analysis helpers
//! - Integration with the review system
//! 
//! ## Usage
//! 
//! ```rust
//! use survey::{Survey, Question, SurveyResponse, Answer};
//! use uuid::Uuid;
//! 
//! // Create a product satisfaction survey
//! let survey = Survey {
//!     id: Uuid::new_v4(),
//!     title: "Product Satisfaction".to_string(),
//!     description: "Tell us about your experience".to_string(),
//!     questions: vec![
//!         Question::StarRating {
//!             min: 0.0,
//!             max: 5.0,
//!             step: 0.5
//!         },
//!         Question::TextResponse {
//!             max_length: Some(500)
//!         }
//!     ],
//!     scoring_config: None
//! };
//! 
//! // Create a response
//! let response = SurveyResponse {
//!     survey_id: survey.id,
//!     answers: vec![
//!         Answer::StarRating(4.5),
//!         Answer::TextResponse("Great product!".to_string())
//!     ],
//!     created_at: chrono::Utc::now()
//! };
//! ```

pub mod models;
pub mod validation;
pub mod analysis;
pub mod visualization;
pub mod template_service;
pub mod error;

pub use models::*;
pub use validation::*;
pub use analysis::*;
pub use visualization::*;
pub use template_service::*;
pub use error::*;
pub use analysis::AnalysisError;
pub use visualization::VisualizationError;
pub use template_service::TemplateError;

#[cfg(test)]
mod tests;
//! ## Example: Integration with Reviews
//! 
//! ```rust
//! use survey::{Survey, Question, SurveyResponse, Answer};
//! use reviews::{Review, Rating, RatingMethod};
//! use uuid::Uuid;
//! 
//! // Create a survey
//! let survey = Survey {
//!     id: Uuid::new_v4(),
//!     title: "Product Satisfaction".to_string(),
//!     description: "Tell us about your experience".to_string(),
//!     questions: vec![
//!         Question::StarRating {
//!             min: 0.0,
//!             max: 5.0,
//!             step: 0.5
//!         },
//!         Question::TextResponse {
//!             max_length: Some(500)
//!         }
//!     ],
//!     scoring_config: None
//! };
//! 
//! // Create a response
//! let response = SurveyResponse {
//!     survey_id: survey.id,
//!     answers: vec![
//!         Answer::StarRating(4.5),
//!         Answer::TextResponse("Great product!".to_string())
//!     ],
//!     created_at: chrono::Utc::now()
//! };
//! 
//! // Attach to review (simplified example)
//! // let review = Review {
//! //     // ... other fields
//! //     survey_response: Some(response),
//! // };
//! ```