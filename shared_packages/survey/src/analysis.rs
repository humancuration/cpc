//! Statistical analysis helpers for survey data

use crate::models::{Survey, SurveyResponse, Answer};
use crate::error::AnalysisError;
use std::collections::HashMap;
use feedback_analysis::calculate_correlation as shared_calculate_correlation;
use feedback_analysis::TrendResult as SharedTrendResult;

/// Calculate the average score for star rating questions
pub fn calculate_average_star_rating(responses: &[SurveyResponse], question_index: usize) -> Option<f32> {
    let mut sum = 0.0;
    let mut count = 0;
    
    for response in responses {
        if let Some(Answer::StarRating(value)) = response.answers.get(question_index) {
            sum += value;
            count += 1;
        }
    }
    
    if count > 0 {
        Some(sum / count as f32)
    } else {
        None
    }
}

/// Get the distribution of answers for a multiple choice question
pub fn get_multiple_choice_distribution(responses: &[SurveyResponse], question_index: usize, option_count: usize) -> Vec<u32> {
    let mut distribution = vec![0; option_count];
    
    for response in responses {
        if let Some(Answer::MultipleChoice(indices)) = response.answers.get(question_index) {
            for &index in indices {
                if index < option_count {
                    distribution[index] += 1;
                }
            }
        }
    }
    
    distribution
}

/// Get the distribution of answers for a Likert scale question
pub fn get_likert_scale_distribution(responses: &[SurveyResponse], question_index: usize, steps: usize) -> Vec<u32> {
    let mut distribution = vec![0; steps];
    
    for response in responses {
        if let Some(Answer::LikertScale(value)) = response.answers.get(question_index) {
            let index = (*value as usize).saturating_sub(1); // Convert to 0-based index
            if index < steps {
                distribution[index] += 1;
            }
        }
    }
    
    distribution
}

/// Calculate completion rate (percentage of surveys completed)
pub fn calculate_completion_rate(responses: &[SurveyResponse], total_sent: usize) -> f32 {
    if total_sent == 0 {
        0.0
    } else {
        (responses.len() as f32 / total_sent as f32) * 100.0
    }
}

/// Get statistics for numerical answers (star ratings, etc.)
pub fn get_numerical_statistics(responses: &[SurveyResponse], question_index: usize) -> Option<NumericalStats> {
    let mut values: Vec<f32> = Vec::new();
    
    for response in responses {
        if let Some(Answer::StarRating(value)) = response.answers.get(question_index) {
            values.push(*value);
        }
    }
    
    if values.is_empty() {
        return None;
    }
    
    values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    let count = values.len();
    let sum: f32 = values.iter().sum();
    let mean = sum / count as f32;
    
    let median = if count % 2 == 0 {
        (values[count / 2 - 1] + values[count / 2]) / 2.0
    } else {
        values[count / 2]
    };
    
    let min = *values.first().unwrap();
    let max = *values.last().unwrap();
    
    // Calculate standard deviation
    let variance = values.iter().map(|&x| (x - mean).powi(2)).sum::<f32>() / count as f32;
    let std_dev = variance.sqrt();
    
    Some(NumericalStats {
        count,
        mean,
        median,
        min,
        max,
        std_dev,
    })
}

/// Statistics for numerical data
#[derive(Debug, Clone)]
pub struct NumericalStats {
    pub count: usize,
    pub mean: f32,
    pub median: f32,
    pub min: f32,
    pub max: f32,
    pub std_dev: f32,
}

impl NumericalStats {
    /// Check if the statistics are valid
    pub fn is_valid(&self) -> bool {
        self.count > 0
    }
/// Calculate correlation between two numerical questions
pub fn calculate_correlation(responses: &[SurveyResponse], q1_idx: usize, q2_idx: usize) -> Result<Option<f32>, AnalysisError> {
    // Validate question indices
    if responses.is_empty() {
        return Ok(None);
    }
    
    let first_response = &responses[0];
    if q1_idx >= first_response.answers.len() || q2_idx >= first_response.answers.len() {
        return Err(AnalysisError::InvalidQuestionIndex(
            if q1_idx >= first_response.answers.len() { q1_idx } else { q2_idx }
        ));
    }
    
    let mut pairs: Vec<(f32, f32)> = Vec::new();
    
    // Collect paired responses
    for response in responses {
        if let (Some(Answer::StarRating(val1)), Some(Answer::StarRating(val2))) =
            (response.answers.get(q1_idx), response.answers.get(q2_idx)) {
            pairs.push((*val1, *val2));
        }
    }
    
    if pairs.len() < 2 {
        return Ok(None);
    }
    
    // Use shared correlation calculation
    let result = shared_calculate_correlation(&pairs)
        .map_err(|e| AnalysisError::CorrelationError(e.to_string()))?;
    
    Ok(result)
}
}
}

/// Time period for trend analysis
#[derive(Debug, Clone)]
pub enum TimePeriod {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

/// Result of trend analysis
pub type TrendResult = SharedTrendResult;

/// Analyze trends over time periods
pub fn analyze_trends(responses: &[SurveyResponse], question_idx: usize, time_period: TimePeriod) -> Result<TrendResult, AnalysisError> {
    // Validate question index
    if responses.is_empty() {
        return Err(AnalysisError::InsufficientData);
    }
    
    let first_response = &responses[0];
    if question_idx >= first_response.answers.len() {
        return Err(AnalysisError::InvalidQuestionIndex(question_idx));
    }
    
    use std::collections::HashMap;
    use chrono::{Datelike, Duration};
    
    let mut period_data: HashMap<String, Vec<f32>> = HashMap::new();
    
    // Group responses by time period
    for response in responses {
        if let Some(Answer::StarRating(value)) = response.answers.get(question_idx) {
            let period_key = match time_period {
                TimePeriod::Daily => response.created_at.format("%Y-%m-%d").to_string(),
                TimePeriod::Weekly => {
                    let week = response.created_at.iso_week().week();
                    format!("{}-W{:02}", response.created_at.year(), week)
                },
                TimePeriod::Monthly => response.created_at.format("%Y-%m").to_string(),
                TimePeriod::Yearly => response.created_at.format("%Y").to_string(),
            };
            
            period_data.entry(period_key).or_insert_with(Vec::new).push(*value);
        }
    }
    
    // Calculate averages for each period and populate shared TrendResult
    let mut trend_result = TrendResult::new();
    let mut periods: Vec<String> = period_data.keys().cloned().collect();
    periods.sort();
    
    for period in &periods {
        if let Some(values) = period_data.get(period) {
            let sum: f32 = values.iter().sum();
            let average = sum / values.len() as f32;
            let count = values.len();
            trend_result.add_point(period.clone(), average, count);
        } else {
            trend_result.add_point(period.clone(), 0.0, 0);
        }
    }
    
    Ok(trend_result)
}

/// Analyze sentiment of text responses
pub fn analyze_sentiment(text_responses: &[String]) -> Vec<f32> {
    // Simple sentiment analysis based on keyword scoring
    // In a real implementation, this would use a proper NLP library
    let positive_words = ["good", "great", "excellent", "amazing", "wonderful", "fantastic", "love", "like"];
    let negative_words = ["bad", "terrible", "awful", "horrible", "hate", "dislike", "poor", "worst"];
    
    text_responses.iter().map(|text| {
        let lower_text = text.to_lowercase();
        let mut score = 0.0;
        
        for word in &positive_words {
            if lower_text.contains(word) {
                score += 1.0;
            }
        }
        
        for word in &negative_words {
            if lower_text.contains(word) {
                score -= 1.0;
            }
        }
        
        // Normalize to [-1, 1] range
        score / (positive_words.len().max(negative_words.len()) as f32)
    }).collect()
}

/// Demographic field for comparative analysis
#[derive(Debug, Clone)]
pub enum DemographicField {
    Age,
    Gender,
    Location,
    // Add more demographic fields as needed
}

/// Compare demographic groups for a specific question
pub fn compare_demographic_groups(
    responses: &[SurveyResponse],
    _demographic: DemographicField,
    question_idx: usize
) -> Result<HashMap<String, NumericalStats>, AnalysisError> {
    // Validate question index
    if responses.is_empty() {
        return Err(AnalysisError::InsufficientData);
    }
    
    let first_response = &responses[0];
    if question_idx >= first_response.answers.len() {
        return Err(AnalysisError::InvalidQuestionIndex(question_idx));
    }
    
    // This is a simplified implementation
    // In a real implementation, demographic data would be part of the response
    
    // For demonstration, we'll group by whether the rating is above or below average
    let avg = calculate_average_star_rating(responses, question_idx).unwrap_or(3.0);
    
    let mut above_avg_responses = Vec::new();
    let mut below_avg_responses = Vec::new();
    
    for response in responses {
        if let Some(Answer::StarRating(value)) = response.answers.get(question_idx) {
            if *value >= avg {
                above_avg_responses.push(response.clone());
            } else {
                below_avg_responses.push(response.clone());
            }
        }
    }
    
    let mut result = HashMap::new();
    
    if let Some(stats) = get_numerical_statistics(&above_avg_responses, question_idx) {
        result.insert("Above Average".to_string(), stats);
    }
    
    if let Some(stats) = get_numerical_statistics(&below_avg_responses, question_idx) {
        result.insert("Below Average".to_string(), stats);
    }
    
    Ok(result)
}

/// Check cache for computed result
fn with_cache<F, T>(key: &str, compute: F) -> Result<T, AnalysisError>
where
    F: FnOnce() -> T,
    T: serde::Serialize + for<'de> serde::Deserialize<'de>,
{
    // In a real implementation, this would check sled cache first
    // For now, we'll just compute the result
    Ok(compute())
}

/// Get a random sample of responses
pub fn sampled_responses(responses: &[SurveyResponse], sample_size: usize) -> Result<Vec<SurveyResponse>, AnalysisError> {
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    
    if sample_size == 0 {
        return Err(AnalysisError::SamplingError("Sample size must be greater than 0".to_string()));
    }
    
    if sample_size >= responses.len() {
        return Ok(responses.to_vec());
    }
    
    let mut rng = thread_rng();
    let mut sampled = responses.to_vec();
    sampled.shuffle(&mut rng);
    sampled.truncate(sample_size);
    Ok(sampled)
}