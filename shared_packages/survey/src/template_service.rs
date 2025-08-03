//! Template service for survey templates
//!
//! This module provides functions to create, retrieve, and version survey templates.

use crate::models::{Survey, SurveyTemplate, QuestionTemplate, Question};
use crate::error::TemplateError;
use uuid::Uuid;
use chrono::Utc;
use serde_json::Value;
use std::collections::HashMap;

/// Convert a Survey to a SurveyTemplate
pub fn create_template(survey: Survey) -> SurveyTemplate {
    let question_templates = survey.questions.into_iter().map(|q| {
        let (question_type, configuration) = match q {
            Question::StarRating { min, max, step } => {
                let config = serde_json::json!({
                    "min": min,
                    "max": max,
                    "step": step
                });
                ("StarRating".to_string(), config)
            },
            Question::TextResponse { max_length } => {
                let config = serde_json::json!({
                    "max_length": max_length
                });
                ("TextResponse".to_string(), config)
            },
            Question::MultipleChoice { options, multiple } => {
                let config = serde_json::json!({
                    "options": options,
                    "multiple": multiple
                });
                ("MultipleChoice".to_string(), config)
            },
            Question::LikertScale { min_label, max_label, steps } => {
                let config = serde_json::json!({
                    "min_label": min_label,
                    "max_label": max_label,
                    "steps": steps
                });
                ("LikertScale".to_string(), config)
            },
            Question::Matrix { rows, columns } => {
                let config = serde_json::json!({
                    "rows": rows,
                    "columns": columns
                });
                ("Matrix".to_string(), config)
            },
        };
        
        QuestionTemplate {
            question_type: match question_type.as_str() {
                "StarRating" => crate::models::QuestionType::StarRating,
                "TextResponse" => crate::models::QuestionType::TextResponse,
                "MultipleChoice" => crate::models::QuestionType::MultipleChoice,
                "LikertScale" => crate::models::QuestionType::LikertScale,
                "Matrix" => crate::models::QuestionType::Matrix,
                _ => crate::models::QuestionType::TextResponse, // default fallback
            },
            configuration,
        }
    }).collect();
    
    SurveyTemplate {
        id: Uuid::new_v4(),
        name: survey.title,
        description: survey.description,
        questions: question_templates,
        version: 1,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

/// Get a template by ID
/// In a real implementation, this would query a database
pub fn get_template(_id: Uuid) -> Option<SurveyTemplate> {
    // This is a placeholder implementation
    // In a real system, you would retrieve the template from storage
    None
}

/// Create a new version of a template
pub fn version_template(
    _template_id: Uuid, 
    mut new_version: SurveyTemplate
) -> Result<(), TemplateError> {
    // In a real implementation, this would:
    // 1. Retrieve the existing template
    // 2. Check version compatibility
    // 3. Save the new version
    // 4. Update version tracking
    
    // For now, we'll just update the timestamps
    new_version.updated_at = Utc::now();
    new_version.version += 1;
    
    Ok(())
}

/// Apply a template to create a new Survey
pub fn apply_template(template: SurveyTemplate) -> Survey {
    let questions = template.questions.into_iter().map(|qt| {
        match qt.question_type {
            crate::models::QuestionType::StarRating => {
                let min = qt.configuration.get("min").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
                let max = qt.configuration.get("max").and_then(|v| v.as_f64()).unwrap_or(5.0) as f32;
                let step = qt.configuration.get("step").and_then(|v| v.as_f64()).unwrap_or(0.5) as f32;
                Question::StarRating { min, max, step }
            },
            crate::models::QuestionType::TextResponse => {
                let max_length = qt.configuration.get("max_length").and_then(|v| v.as_u64()).map(|v| v as usize);
                Question::TextResponse { max_length }
            },
            crate::models::QuestionType::MultipleChoice => {
                let options = qt.configuration.get("options")
                    .and_then(|v| v.as_array())
                    .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                    .unwrap_or_else(Vec::new);
                let multiple = qt.configuration.get("multiple").and_then(|v| v.as_bool()).unwrap_or(false);
                Question::MultipleChoice { options, multiple }
            },
            crate::models::QuestionType::LikertScale => {
                let min_label = qt.configuration.get("min_label").and_then(|v| v.as_str()).unwrap_or("Disagree").to_string();
                let max_label = qt.configuration.get("max_label").and_then(|v| v.as_str()).unwrap_or("Agree").to_string();
                let steps = qt.configuration.get("steps").and_then(|v| v.as_u64()).unwrap_or(5) as u8;
                Question::LikertScale { min_label, max_label, steps }
            },
            crate::models::QuestionType::Matrix => {
                let rows = qt.configuration.get("rows")
                    .and_then(|v| v.as_array())
                    .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                    .unwrap_or_else(Vec::new);
                let columns = qt.configuration.get("columns")
                    .and_then(|v| v.as_array())
                    .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                    .unwrap_or_else(Vec::new);
                Question::Matrix { rows, columns }
            },
        }
    }).collect();
    
    Survey {
        id: Uuid::new_v4(),
        title: template.name,
        description: template.description,
        questions,
        scoring_config: None, // Templates don't include scoring config
    }
}