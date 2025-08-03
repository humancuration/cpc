//! Example showing integration between survey and review modules

use survey::{Survey, Question, SurveyResponse, Answer};
use reviews::{Review, Rating, RatingMethod, Demographics, Attribute};
use uuid::Uuid;
use chrono::Utc;
use std::fmt::Debug;

// Simple test entity for demonstration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Product {
    id: Uuid,
    name: String,
}

impl reviews::Entity for Product {
    fn id(&self) -> Uuid {
        self.id
    }
    
    fn entity_type(&self) -> String {
        "product".to_string()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a product
    let product = Product {
        id: Uuid::new_v4(),
        name: "Test Product".to_string(),
    };

    // Create a survey
    let survey = Survey {
        id: Uuid::new_v4(),
        title: "Product Feedback".to_string(),
        description: "Help us improve our product".to_string(),
        questions: vec![
            Question::StarRating {
                min: 0.0,
                max: 5.0,
                step: 0.5,
            },
            Question::TextResponse {
                max_length: Some(1000),
            },
        ],
        scoring_config: None,
    };

    // Create a survey response
    let response = SurveyResponse {
        survey_id: survey.id,
        answers: vec![
            Answer::StarRating(4.5),
            Answer::TextResponse("Excellent product, very satisfied with my purchase!".to_string()),
        ],
        created_at: Utc::now(),
    };

    // Create a review with the survey response
    let review = Review {
        id: Uuid::new_v4(),
        entity: product,
        user_id: Uuid::new_v4(),
        title: "Highly Recommended!".to_string(),
        content: "This is an excellent product that I would recommend to others.".to_string(),
        ratings: vec![
            Rating {
                metric: "overall".to_string(),
                value: 0.9,
                unit: Some("%".to_string()),
                method: RatingMethod::UserReported,
            },
            Rating {
                metric: "quality".to_string(),
                value: 0.85,
                unit: Some("%".to_string()),
                method: RatingMethod::UserReported,
            },
        ],
        attributes: vec![
            Attribute {
                key: "pros".to_string(),
                value: "durable, well-designed".to_string(),
            },
            Attribute {
                key: "cons".to_string(),
                value: "price could be lower".to_string(),
            },
        ],
        demographics: Some(Demographics {
            age_group: "35-44".to_string(),
            gender: "male".to_string(),
            location: "Seattle, WA".to_string(),
            occupation: Some("Software Engineer".to_string()),
        }),
        survey_response: Some(response),
        created_at: Utc::now(),
    };

    // Validate the review (this will also validate the survey response)
    match review.validate() {
        Ok(()) => println!("Review with survey response is valid!"),
        Err(e) => println!("Validation error: {}", e),
    }

    println!("Review title: {}", review.title);
    println!("Survey response answers: {}", review.survey_response.as_ref().map(|r| r.answers.len()).unwrap_or(0));
    
    Ok(())
}