# Survey Module Design

## Core Components
```rust
// Survey definition
pub struct Survey {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub questions: Vec<Question>,
    pub scoring_config: Option<ScoringConfig>
}

// Question types
pub enum Question {
    StarRating {
        min: f32,       // Typically 0.0
        max: f32,       // Typically 5.0
        step: f32       // 0.5 for half-star increments
    },
    TextResponse {
        max_length: Option<usize>
    },
    MultipleChoice {
        options: Vec<String>,
        multiple: bool  // Allow multiple selections
    },
    LikertScale {
        min_label: String,  // e.g. "Strongly Disagree"
        max_label: String,  // e.g. "Strongly Agree"
        steps: u8           // Typically 5 or 7
    },
    Matrix {
        rows: Vec<String>,   // Row questions
        columns: Vec<String> // Column options
    }
}

// Survey response
pub struct SurveyResponse {
    pub survey_id: Uuid,
    pub answers: Vec<Answer>,
    pub created_at: DateTime<Utc>
}

// Answer types corresponding to questions
pub enum Answer {
    StarRating(f32),
    TextResponse(String),
    MultipleChoice(Vec<usize>), // Indices of selected options
    LikertScale(u8),            // Selected step (1-based)
    Matrix(Vec<Vec<bool>>)      // Row x Column selections
}

// Scoring configuration
pub struct ScoringConfig {
    pub weights: HashMap<String, f32>, // Weights for questions
    pub formula: Option<String>         // Custom scoring formula
}
```

## Validation Rules
1. Survey must have at least one question
2. StarRating must have min < max and step > 0
3. MultipleChoice must have at least one option
4. Matrix must have non-empty rows and columns
5. Answers must match question types and constraints

## Usage Example
```rust
// Create a product satisfaction survey
let survey = Survey {
    id: Uuid::new_v4(),
    title: "Product Satisfaction".to_string(),
    description: "Tell us about your experience".to_string(),
    questions: vec![
        Question::StarRating {
            min: 0.0,
            max: 5.0,
            step: 0.5
        },
        Question::TextResponse {
            max_length: Some(500)
        }
    ],
    scoring_config: None
};

// Create a response
let response = SurveyResponse {
    survey_id: survey.id,
    answers: vec![
        Answer::StarRating(4.5),
        Answer::TextResponse("Great product!".to_string())
    ],
    created_at: Utc::now()
};

// Attach to review
let review = Review {
    // ... other fields
    survey_response: Some(response),
};
```

## Statistical Analysis
```rust
impl SurveyResponse {
    pub fn calculate_score(&self, survey: &Survey) -> f32 {
        // Calculate score based on scoring config
        // ...
    }
}

// Aggregate analysis
pub fn average_scores(survey_id: Uuid) -> HashMap<Uuid, f32> {
    // Calculate average scores for all questions
    // ...
}

## Template System
```rust
pub struct SurveyTemplate {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub questions: Vec<QuestionTemplate>,
    pub version: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct QuestionTemplate {
    pub question_type: QuestionType,
    pub configuration: serde_json::Value,
}
```

## Visualization
```rust
pub mod plot {
    pub fn histogram(data: &[f32], bins: usize) -> Result<Vec<u8>, VisualizationError> { /* ... */ }
    pub fn heatmap(matrix: &[Vec<u32>]) -> Result<Vec<u8>, VisualizationError> { /* ... */ }
    pub fn word_cloud(words: &[(&str, f32)]) -> Result<Vec<u8>, VisualizationError> { /* ... */ }
    pub fn trend_line(points: &[(DateTime<Utc>, f32)]) -> Result<Vec<u8>, VisualizationError> { /* ... */ }
}
```

## Advanced Statistical Analysis
```rust
// Correlation analysis between two numerical questions
pub fn calculate_correlation(responses: &[SurveyResponse], q1_idx: usize, q2_idx: usize) -> Option<f32> {
    // Implementation for numerical questions
}

// Trend analysis over time periods
pub fn analyze_trends(responses: &[SurveyResponse], question_idx: usize, time_period: TimePeriod) -> TrendResult {
    // Group by time period and calculate averages
}

// Sentiment analysis for text responses
pub fn analyze_sentiment(text_responses: &[String]) -> Vec<f32> {
    // Use NLP library for sentiment scoring
}

// Comparative analysis between demographic groups
pub fn compare_demographic_groups(responses: &[SurveyResponse], demographic: DemographicField, question_idx: usize) -> HashMap<String, NumericalStats> {
    // Group by demographic and calculate stats
}
```