use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillProgressData {
    pub id: Uuid,
    pub skill_name: String,
    pub current_level: SkillLevel,
    pub target_level: SkillLevel,
    pub progress_percentage: f32,
    pub total_hours_invested: u32,
    pub last_practice_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningPathData {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub estimated_duration_hours: u32,
    pub difficulty_level: DifficultyLevel,
    pub progress: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificationData {
    pub id: Uuid,
    pub skill_name: String,
    pub level_achieved: SkillLevel,
    pub certification_type: CertificationType,
    pub issued_at: String,
    pub verification_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SkillLevel {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
    Master,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DifficultyLevel {
    Beginner,
    Intermediate,
    Advanced,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationType {
    CourseCompletion,
    PeerEndorsement,
    SkillAssessment,
    ProjectReview,
    PortfolioReview,
}