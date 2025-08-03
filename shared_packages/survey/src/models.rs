//! Core domain models for the survey system
//! 
//! This module defines the core data structures for surveys, questions, responses, and answers.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Survey definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Survey {
    /// Unique identifier for this survey
    pub id: Uuid,
    
    /// Title of the survey
    pub title: String,
    
    /// Description of the survey
    pub description: String,
    
    /// Questions in the survey
    pub questions: Vec<Question>,
    
    /// Optional scoring configuration
    pub scoring_config: Option<ScoringConfig>,
}

/// Question types supported in surveys
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Question {
    /// Star rating question (e.g., 0-5 stars)
    StarRating {
        /// Minimum rating value (typically 0.0)
        min: f32,
        /// Maximum rating value (typically 5.0)
        max: f32,
        /// Step increment (e.g., 0.5 for half-star increments)
        step: f32,
    },
    
    /// Text response question
    TextResponse {
        /// Maximum length of response (None for unlimited)
        max_length: Option<usize>,
    },
    
    /// Multiple choice question
    MultipleChoice {
        /// Available options
        options: Vec<String>,
        /// Whether multiple selections are allowed
        multiple: bool,
    },
    
    /// Likert scale question
    LikertScale {
        /// Label for minimum value (e.g., "Strongly Disagree")
        min_label: String,
        /// Label for maximum value (e.g., "Strongly Agree")
        max_label: String,
        /// Number of steps (typically 5 or 7)
        steps: u8,
    },
    
    /// Matrix question (grid of rows and columns)
    Matrix {
        /// Row labels (questions)
        rows: Vec<String>,
        /// Column labels (options)
        columns: Vec<String>,
    },
}

/// Survey response containing answers to a survey
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurveyResponse {
    /// ID of the survey this response is for
    pub survey_id: Uuid,
    
    /// Answers to the survey questions
    pub answers: Vec<Answer>,
    
    /// When this response was created
    pub created_at: DateTime<Utc>,
}

/// Answer types corresponding to question types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Answer {
    /// Star rating answer
    StarRating(f32),
    
    /// Text response answer
    TextResponse(String),
    
    /// Multiple choice answer (indices of selected options)
    MultipleChoice(Vec<usize>),
    
    /// Likert scale answer (selected step, 1-based)
    LikertScale(u8),
    
    /// Matrix answer (row x column selections)
    Matrix(Vec<Vec<bool>>),
}

/// Configuration for scoring surveys
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoringConfig {
    /// Weights for questions (question index -> weight)
    pub weights: HashMap<String, f32>,
    
    /// Optional custom scoring formula
    pub formula: Option<String>,
}

impl Survey {
    /// Get the number of questions in the survey
    pub fn len(&self) -> usize {
        self.questions.len()
    }
    
    /// Check if the survey has no questions
    pub fn is_empty(&self) -> bool {
        self.questions.is_empty()
    }
}

impl Question {
    /// Get a human-readable name for the question type
    pub fn type_name(&self) -> &'static str {
        match self {
            Question::StarRating { .. } => "Star Rating",
            Question::TextResponse { .. } => "Text Response",
            Question::MultipleChoice { .. } => "Multiple Choice",
            Question::LikertScale { .. } => "Likert Scale",
            Question::Matrix { .. } => "Matrix",
        }
    }
}

/// Mapping between survey questions and review attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewAttributeMapping {
    /// ID of the survey question
    pub survey_question_id: Uuid,
    
    /// Key for the review attribute
    pub review_attribute_key: String,
}

/// Survey template for reusable survey designs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurveyTemplate {
    /// Unique identifier for this template
    pub id: Uuid,
    
    /// Name of the template
    pub name: String,
    
    /// Description of the template
    pub description: String,
    
    /// Questions in the template
    pub questions: Vec<QuestionTemplate>,
    
    /// Version number for the template
    pub version: u32,
    
    /// When this template was created
    pub created_at: DateTime<Utc>,
    
    /// When this template was last updated
    pub updated_at: DateTime<Utc>,
    
    /// Mappings between survey questions and review attributes
    pub review_attribute_mappings: Vec<ReviewAttributeMapping>,
}

/// Question template for reusable question configurations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionTemplate {
    /// Type of question
    pub question_type: QuestionType,
    
    /// Configuration for the question (JSON format for flexibility)
    pub configuration: serde_json::Value,
}

/// Question types for templates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuestionType {
    StarRating,
    TextResponse,
    MultipleChoice,
    LikertScale,
    Matrix,
}