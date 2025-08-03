//! Feedback System Showcase with Web UI
//!
//! This example demonstrates the integration of all feedback system components:
//! - Sample product reviews with attributes
//! - Survey responses mapped to reviews
//! - Federation metadata simulation
//! - Visualization rendering examples

// Yew imports for UI
use yew::prelude::*;

// Import UI components
mod components;
use components::data_generator_ui::DataGeneratorUI;

// For compatibility with existing code, we'll keep the original modules
mod data_generator;
use data_generator::generators::products::Product;
use reviews::{Review, Rating, RatingMethod, Attribute, Demographics, Entity, FederationMetadata, ConsentRule, FederationGroup, FederatedReview};
use survey::{Survey, Question, SurveyResponse, Answer};
use feedback_analysis::{RatingDistribution, calculate_correlation, TrendResult};
use feedback_visualization::{Heatmap, TrendComparison, CorrelationMatrix};
use feedback_core::FeedbackError;
use uuid::Uuid;
use chrono::Utc;
use std::collections::HashMap;

fn main() {
    yew::Renderer::<DataGeneratorUI>::new().render();
}

// The following functions are kept for compatibility but won't be used in the UI version
// They can be moved to a separate binary or example if needed

/// Create a sample survey
fn create_sample_survey() -> Result<Survey, FeedbackError> {
    let survey = Survey {
        id: Uuid::new_v4(),
        title: "Product Feedback Survey".to_string(),
        description: "Help us improve our eco-friendly water bottle".to_string(),
        questions: vec![
            Question::StarRating {
                min: 0.0,
                max: 5.0,
                step: 0.5,
            },
            Question::TextResponse {
                max_length: Some(1000),
            },
            Question::MultipleChoice {
                options: vec![
                    "Design".to_string(),
                    "Durability".to_string(),
                    "Price".to_string(),
                    "Eco-friendliness".to_string(),
                ],
                multiple: true,
            },
        ],
        scoring_config: None,
    };
    
    Ok(survey)
}

/// Demonstrate error unification across packages
fn demonstrate_error_unification() -> Result<(), FeedbackError> {
    // This function is kept for compatibility but won't be used in the UI version
    Ok(())
}

/// Perform statistical analysis on reviews
fn perform_statistical_analysis(reviews: &[Review<Product>]) -> Result<Vec<RatingDistribution>, FeedbackError> {
    let mut distributions: HashMap<String, RatingDistribution> = HashMap::new();
    
    // Collect ratings by metric
    for review in reviews {
        for rating in &review.ratings {
            let distribution = distributions
                .entry(rating.metric.clone())
                .or_insert_with(|| RatingDistribution::new(rating.metric.clone()));
            
            distribution.add_rating(rating.value)?;
        }
    }
    
    // Calculate correlations between metrics
    let mut correlations_data = Vec::new();
    for review in reviews {
        let overall_rating = review.ratings.iter().find(|r| r.metric == "overall");
        let quality_rating = review.ratings.iter().find(|r| r.metric == "quality");
        
        if let (Some(overall), Some(quality)) = (overall_rating, quality_rating) {
            correlations_data.push((overall.value, quality.value));
        }
    }
    
    if !correlations_data.is_empty() {
        let correlation = calculate_correlation(&correlations_data)?;
        if let Some(corr) = correlation {
            // In UI version, this would be displayed in the UI
        }
    }
    
    Ok(distributions.into_values().collect())
}

/// Render visualization components
fn render_visualizations(distributions: &[RatingDistribution], reviews: &[Review<Product>]) -> Result<(), FeedbackError> {
    // This function is kept for compatibility but won't be used in the UI version
    Ok(())
}

/// Demonstrate federation consent rules
fn demonstrate_consent_rules(federated_reviews: &[FederatedReview<Product>]) -> Result<(), FeedbackError> {
    // This function is kept for compatibility but won't be used in the UI version
    Ok(())
}