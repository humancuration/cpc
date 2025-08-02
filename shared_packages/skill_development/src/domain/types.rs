use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillProgress {
    pub id: Uuid,
    pub skill_id: Uuid,
    pub user_id: Uuid,
    pub progress: u8, // 0-100
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Certification {
    pub id: Uuid,
    pub name: String,
    pub issuing_organization: String,
    pub issue_date: chrono::NaiveDate,
    pub user_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningPath {
    pub id: Uuid,
    pub user_id: Uuid,
    pub skill_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub milestones: Vec<Milestone>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Milestone {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub is_completed: bool,
    pub order_index: i32,
    pub estimated_duration_hours: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillWithProgress {
    pub skill: Skill,
    pub progress: Option<SkillProgress>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSkillProfile {
    pub user_id: Uuid,
    pub skills: Vec<SkillWithProgress>,
    pub certifications: Vec<Certification>,
    pub learning_paths: Vec<LearningPath>,
}