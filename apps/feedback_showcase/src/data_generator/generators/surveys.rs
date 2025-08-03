//! Survey response generator for the data generator module

use crate::data_generator::config::DataGeneratorConfig;
use crate::data_generator::generators::products::Product;
use reviews::Review;
use survey::{Survey, SurveyResponse, Answer, Question};
use fake::{Fake, faker::lorem::en::*};
use rand::Rng;
use rayon::prelude::*;

/// Generate survey responses for reviews
pub fn generate_survey_responses(
    config: &DataGeneratorConfig,
    survey: &Survey,
    reviews: &mut [Review<Product>]
) -> Vec<SurveyResponse> {
    let response_count = (reviews.len() as f32 * config.survey_response_rate) as usize;
    
    if response_count > 10000 {
        // Use streaming generation for large datasets
        generate_survey_responses_streaming(config, survey, reviews, response_count)
    } else {
        // Use parallel generation for smaller datasets
        generate_survey_responses_parallel(config, survey, reviews, response_count)
    }
}

/// Generate survey responses using parallel processing
fn generate_survey_responses_parallel(
    config: &DataGeneratorConfig,
    survey: &Survey,
    reviews: &mut [Review<Product>],
    response_count: usize
) -> Vec<SurveyResponse> {
    // Select random reviews to associate with survey responses
    let selected_indices: Vec<usize> = {
        let mut indices: Vec<usize> = (0..reviews.len()).collect();
        indices.shuffle(&mut rand::thread_rng());
        indices.truncate(response_count);
        indices
    };
    
    // Generate responses in parallel
    let responses: Vec<SurveyResponse> = selected_indices
        .par_iter()
        .map(|&index| generate_single_survey_response(survey, &reviews[index]))
        .collect();
    
    // Associate responses with reviews
    for (i, &index) in selected_indices.iter().enumerate() {
        reviews[index].survey_response = Some(responses[i].clone());
    }
    
    responses
}

/// Generate survey responses using streaming for large datasets
fn generate_survey_responses_streaming(
    config: &DataGeneratorConfig,
    survey: &Survey,
    reviews: &mut [Review<Product>],
    response_count: usize
) -> Vec<SurveyResponse> {
    let mut responses = Vec::with_capacity(response_count);
    
    // Select random reviews to associate with survey responses
    let mut indices: Vec<usize> = (0..reviews.len()).collect();
    indices.shuffle(&mut rand::thread_rng());
    indices.truncate(response_count);
    
    // Generate responses sequentially
    for &index in &indices {
        let response = generate_single_survey_response(survey, &reviews[index]);
        reviews[index].survey_response = Some(response.clone());
        responses.push(response);
    }
    
    responses
}

/// Generate a single survey response
fn generate_single_survey_response(survey: &Survey, review: &Review<Product>) -> SurveyResponse {
    let mut answers = Vec::new();
    
    for question in &survey.questions {
        let answer = generate_answer_for_question(question, review);
        answers.push(answer);
    }
    
    SurveyResponse {
        survey_id: survey.id,
        answers,
        created_at: review.created_at, // Match review creation time
    }
}

/// Generate an answer for a specific question type
fn generate_answer_for_question(question: &Question, review: &Review<Product>) -> Answer {
    let mut rng = rand::thread_rng();
    
    match question {
        Question::StarRating { min, max, step } => {
            // Generate rating based on review's overall rating
            if let Some(overall_rating) = review.ratings.iter().find(|r| r.metric == "overall") {
                // Convert 0-1 scale to min-max scale
                let rating_range = max - min;
                let scaled_rating = min + (overall_rating.value * rating_range);
                
                // Apply step rounding
                let stepped_rating = (scaled_rating / step).round() * step;
                
                // Ensure within bounds
                let final_rating = stepped_rating.max(*min).min(*max);
                
                Answer::StarRating(final_rating)
            } else {
                // Fallback to random rating
                let rating = rng.gen_range(*min..=*max);
                let stepped_rating = (rating / step).round() * step;
                Answer::StarRating(stepped_rating)
            }
        },
        
        Question::TextResponse { max_length } => {
            // Generate response based on review content
            let sentence_count = rng.gen_range(1..=3);
            let mut sentences = Vec::new();
            
            for _ in 0..sentence_count {
                sentences.push(Sentence(5..15).fake::<String>());
            }
            
            let mut response = sentences.join(" ");
            
            // Apply max length if specified
            if let Some(max) = max_length {
                if response.len() > *max {
                    response.truncate(*max - 3);
                    response.push_str("...");
                }
            }
            
            Answer::TextResponse(response)
        },
        
        Question::MultipleChoice { options, multiple } => {
            if *multiple {
                // Select multiple options
                let max_selections = options.len().min(3);
                let selection_count = rng.gen_range(1..=max_selections);
                
                let mut selected_indices: Vec<usize> = (0..options.len()).collect();
                selected_indices.shuffle(&mut rng);
                selected_indices.truncate(selection_count);
                selected_indices.sort();
                
                Answer::MultipleChoice(selected_indices)
            } else {
                // Select single option
                let selected_index = rng.gen_range(0..options.len());
                Answer::MultipleChoice(vec![selected_index])
            }
        },
        
        Question::LikertScale { steps, .. } => {
            // Generate based on review sentiment
            let overall_rating = review.ratings.iter().find(|r| r.metric == "overall")
                .map(|r| r.value)
                .unwrap_or(0.5);
            
            // Map 0-1 rating to 1-steps scale
            let step_value = (overall_rating * (*steps as f32 - 1.0) + 1.0).round() as u8;
            let clamped_step = step_value.max(1).min(*steps);
            
            Answer::LikertScale(clamped_step)
        },
        
        Question::Matrix { rows, columns } => {
            // Generate matrix responses
            let mut matrix = Vec::new();
            
            for _ in 0..rows.len() {
                let mut row = vec![false; columns.len()];
                
                // Select 1-2 random columns per row
                let selection_count = rng.gen_range(1..=columns.len().min(2));
                let mut column_indices: Vec<usize> = (0..columns.len()).collect();
                column_indices.shuffle(&mut rng);
                
                for &col_index in column_indices.iter().take(selection_count) {
                    row[col_index] = true;
                }
                
                matrix.push(row);
            }
            
            Answer::Matrix(matrix)
        },
    }
}