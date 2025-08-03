//! Basic usage example of the survey module

use survey::{Survey, Question, SurveyResponse, Answer, validate_survey, validate_survey_response};
use uuid::Uuid;
use chrono::Utc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a product satisfaction survey
    let survey = Survey {
        id: Uuid::new_v4(),
        title: "Product Satisfaction".to_string(),
        description: "Tell us about your experience with our product".to_string(),
        questions: vec![
            Question::StarRating {
                min: 0.0,
                max: 5.0,
                step: 0.5,
            },
            Question::TextResponse {
                max_length: Some(500),
            },
            Question::MultipleChoice {
                options: vec![
                    "Quality".to_string(),
                    "Price".to_string(),
                    "Design".to_string(),
                    "Features".to_string(),
                ],
                multiple: true,
            },
        ],
        scoring_config: None,
    };

    // Validate the survey
    validate_survey(&survey)?;
    println!("Survey is valid!");

    // Create a response
    let response = SurveyResponse {
        survey_id: survey.id,
        answers: vec![
            Answer::StarRating(4.5),
            Answer::TextResponse("Great product! Really enjoyed using it.".to_string()),
            Answer::MultipleChoice(vec![0, 2]), // Quality and Design
        ],
        created_at: Utc::now(),
    };

    // Validate the response
    validate_survey_response(&response, &survey)?;
    println!("Survey response is valid!");

    println!("Survey: {}", survey.title);
    println!("Response created at: {}", response.created_at);
    
    Ok(())
}