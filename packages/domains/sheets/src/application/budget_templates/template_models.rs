use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Types of budget templates
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TemplateType {
    MonthlyBudget,
    WeeklyBudget,
    ProjectBudget,
    Custom,
}

/// Metadata for budget templates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateMetadata {
    pub template_type: TemplateType,
    pub version: String,
    pub description: String,
}

impl TemplateMetadata {
    pub fn new(template_type: TemplateType, version: String, description: String) -> Self {
        Self {
            template_type,
            version,
            description,
        }
    }
}

/// Template identification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateIdentification {
    pub template_type: TemplateType,
    pub confidence: f64, // 0.0 to 1.0
}