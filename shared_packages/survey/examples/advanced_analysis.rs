//! Advanced analysis example for the survey module
//! 
//! This example demonstrates the advanced statistical analysis and visualization
//! capabilities of the survey module.

use survey::{
    Survey, Question, SurveyResponse, Answer,
    analysis::*, visualization::*, template_service::*
};
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
                    "Price".to_string(),
                    "Quality".to_string(),
                    "Features".to_string(),
                    "Customer Service".to_string(),
                ],
                multiple: true,
            },
        ],
        scoring_config: None,
    };
    
    // Create several responses
    let responses = vec![
        SurveyResponse {
            survey_id: survey.id,
            answers: vec![
                Answer::StarRating(4.5),
                Answer::StarRating(4.0),
                Answer::TextResponse("Great product overall!".to_string()),
                Answer::MultipleChoice(vec![1, 2]), // Quality, Features
            ],
            created_at: Utc::now(),
        },
        SurveyResponse {
            survey_id: survey.id,
            answers: vec![
                Answer::StarRating(3.5),
                Answer::StarRating(3.0),
                Answer::TextResponse("Good product but could be better.".to_string()),
                Answer::MultipleChoice(vec![0, 3]), // Price, Customer Service
            ],
            created_at: Utc::now(),
        },
        SurveyResponse {
            survey_id: survey.id,
            answers: vec![
                Answer::StarRating(5.0),
                Answer::StarRating(4.5),
                Answer::TextResponse("Excellent product! Highly recommend.".to_string()),
                Answer::MultipleChoice(vec![1, 2, 3]), // Quality, Features, Customer Service
            ],
            created_at: Utc::now(),
        },
    ];
    
    // Calculate average ratings
    let avg1 = calculate_average_star_rating(&responses, 0).unwrap();
    let avg2 = calculate_average_star_rating(&responses, 1).unwrap();
    
    println!("Average rating for question 1: {:.2}", avg1);
    println!("Average rating for question 2: {:.2}", avg2);
    
    // Calculate correlation between the two rating questions
    let correlation = calculate_correlation(&responses, 0, 1)?.unwrap();
    println!("Correlation between questions 1 and 2: {:.2}", correlation);
    
    // Analyze trends (in a real scenario, we'd have responses over time)
    let trends = analyze_trends(&responses, 0, TimePeriod::Monthly)?;
    println!("Number of time periods: {}", trends.periods.len());
    
    // Analyze sentiment of text responses
    let text_responses: Vec<String> = responses.iter()
        .filter_map(|r| {
            if let Some(Answer::TextResponse(text)) = r.answers.get(2) {
                Some(text.clone())
            } else {
                None
            }
        })
        .collect();
    
    let sentiments = analyze_sentiment(&text_responses);
    println!("Sentiment scores: {:?}", sentiments);
    
    // Compare demographic groups (simplified example)
    let comparison = compare_demographic_groups(&responses, DemographicField::Age, 0)?;
    println!("Number of demographic groups: {}", comparison.len());
    
    // Create a sample for analysis
    let sample = sampled_responses(&responses, 2)?;
    println!("Sample size: {}", sample.len());
    
    // Create visualizations
    // Histogram of ratings
    let ratings: Vec<f32> = responses.iter()
        .filter_map(|r| {
            if let Some(Answer::StarRating(rating)) = r.answers.get(0) {
                Some(*rating)
            } else {
                None
            }
        })
        .collect();
    
    match plot::histogram(&ratings, 5) {
        Ok(_) => println!("Histogram created successfully"),
        Err(e) => println!("Error creating histogram: {}", e),
    }
    
    // Heatmap of correlation matrix (simplified)
    let matrix = vec![
        vec![10, 8, 6],
        vec![8, 10, 7],
        vec![6, 7, 10],
    ];
    
    match plot::heatmap(&matrix) {
        Ok(_) => println!("Heatmap created successfully"),
        Err(e) => println!("Error creating heatmap: {}", e),
    }
    
    // Word cloud of text responses
    let words = vec![
        ("great", 0.8),
        ("good", 0.6),
        ("excellent", 0.9),
        ("better", 0.5),
        ("recommend", 0.7),
    ];
    
    match plot::word_cloud(&words) {
        Ok(_) => println!("Word cloud created successfully"),
        Err(e) => println!("Error creating word cloud: {}", e),
    }
    
    // Trend line (simplified)
    let points = vec![
        (Utc::now(), 4.0),
        (Utc::now(), 4.2),
        (Utc::now(), 4.5),
    ];
    
    match plot::trend_line(&points) {
        Ok(_) => println!("Trend line created successfully"),
        Err(e) => println!("Error creating trend line: {}", e),
    }
    
    // Template system
    let template = create_template(survey);
    println!("Template created with {} questions", template.questions.len());
    
    let new_survey = apply_template(template);
    println!("New survey created with {} questions", new_survey.questions.len());
    
    Ok(())
}