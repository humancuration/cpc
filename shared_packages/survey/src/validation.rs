//! Validation logic for surveys and survey responses

use crate::models::{Survey, SurveyResponse, Question, Answer};
use crate::error::ValidationError;

// Re-export the ValidationError enum
pub use crate::error::ValidationError;

/// Validate a survey
pub fn validate_survey(survey: &Survey) -> Result<(), ValidationError> {
    // Survey must have at least one question
    if survey.questions.is_empty() {
        return Err(ValidationError::EmptySurvey);
    }
    
    // Validate each question
    for question in &survey.questions {
        validate_question(question)?;
    }
    
    Ok(())
}

/// Validate a question
pub fn validate_question(question: &Question) -> Result<(), ValidationError> {
    match question {
        Question::StarRating { min, max, step } => {
            // Min must be less than max
            if min >= max {
                return Err(ValidationError::InvalidStarRatingRange);
            }
            
            // Step must be greater than 0
            if *step <= 0.0 {
                return Err(ValidationError::InvalidStarRatingStep);
            }
        },
        
        Question::MultipleChoice { options, .. } => {
            // Must have at least one option
            if options.is_empty() {
                return Err(ValidationError::EmptyMultipleChoice);
            }
        },
        
        Question::Matrix { rows, columns } => {
            // Must have non-empty rows and columns
            if rows.is_empty() || columns.is_empty() {
                return Err(ValidationError::EmptyMatrix);
            }
        },
        
        _ => {} // Other question types don't have specific validation rules
    }
    
    Ok(())
}

/// Validate a survey response
pub fn validate_survey_response(response: &SurveyResponse, survey: &Survey) -> Result<(), ValidationError> {
    // Answer count must match question count
    if response.answers.len() != survey.questions.len() {
        return Err(ValidationError::AnswerCountMismatch);
    }
    
    // Validate each answer against its corresponding question
    for (answer, question) in response.answers.iter().zip(survey.questions.iter()) {
        validate_answer(answer, question)?;
    }
    
    Ok(())
}

/// Validate an answer against its corresponding question
pub fn validate_answer(answer: &Answer, question: &Question) -> Result<(), ValidationError> {
    match (answer, question) {
        (Answer::StarRating(value), Question::StarRating { min, max, step }) => {
            // Value must be within range
            if value < min || value > max {
                return Err(ValidationError::StarRatingOutOfRange { 
                    value: *value, 
                    min: *min, 
                    max: *max 
                });
            }
            
            // Value must match step increment
            let steps = ((value - min) / step).round();
            let expected_value = min + steps * step;
            if (value - expected_value).abs() > 1e-6 {
                return Err(ValidationError::StarRatingInvalidStep { 
                    value: *value, 
                    step: *step 
                });
            }
        },
        
        (Answer::TextResponse(text), Question::TextResponse { max_length }) => {
            // Check max length if specified
            if let Some(max) = max_length {
                if text.len() > *max {
                    return Err(ValidationError::TextResponseTooLong { 
                        max_length: *max 
                    });
                }
            }
        },
        
        (Answer::MultipleChoice(indices), Question::MultipleChoice { options, .. }) => {
            // Check that all indices are valid
            for &index in indices {
                if index >= options.len() {
                    return Err(ValidationError::InvalidMultipleChoiceIndex { 
                        index 
                    });
                }
            }
        },
        
        (Answer::LikertScale(value), Question::LikertScale { steps, .. }) => {
            // Value must be between 1 and steps (inclusive)
            if *value < 1 || *value > *steps {
                return Err(ValidationError::LikertScaleOutOfRange { 
                    value: *value, 
                    max_steps: *steps 
                });
            }
        },
        
        (Answer::Matrix(selections), Question::Matrix { rows, columns }) => {
            // Check dimensions
            if selections.len() != rows.len() {
                return Err(ValidationError::MatrixDimensionMismatch);
            }
            
            for row in selections {
                if row.len() != columns.len() {
                    return Err(ValidationError::MatrixDimensionMismatch);
                }
            }
        },
        
        // Type mismatches
        (Answer::StarRating(_), _) => {
            return Err(ValidationError::AnswerTypeMismatch { 
                expected: question.type_name().to_string(), 
                actual: "Star Rating".to_string() 
            });
        },
        
        (Answer::TextResponse(_), _) => {
            return Err(ValidationError::AnswerTypeMismatch { 
                expected: question.type_name().to_string(), 
                actual: "Text Response".to_string() 
            });
        },
        
        (Answer::MultipleChoice(_), _) => {
            return Err(ValidationError::AnswerTypeMismatch { 
                expected: question.type_name().to_string(), 
                actual: "Multiple Choice".to_string() 
            });
        },
        
        (Answer::LikertScale(_), _) => {
            return Err(ValidationError::AnswerTypeMismatch { 
                expected: question.type_name().to_string(), 
                actual: "Likert Scale".to_string() 
            });
        },
        
        (Answer::Matrix(_), _) => {
            return Err(ValidationError::AnswerTypeMismatch { 
                expected: question.type_name().to_string(), 
                actual: "Matrix".to_string() 
            });
        },
    }
    
    Ok(())
}