use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Course {
    pub id: String,
    pub title: String,
    pub description: String,
    pub creator_id: String,
    pub modules: Vec<Module>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub id: String,
    pub title: String,
    pub lessons: Vec<Lesson>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lesson {
    pub id: String,
    pub title: String,
    pub content: String,
    pub media_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enrollment {
    pub id: String,
    pub user_id: String,
    pub course_id: String,
    pub progress: f32,
    pub status: i32, // 0=Enrolled, 1=InProgress, 2=Completed, 3=Dropped
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcademicCredential {
    pub id: String,
    pub user_id: String,
    pub course_id: String,
    pub credential_type: i32, // 0=Certificate, 1=MicroDegree, 2=Degree, 3=Badge
    pub issued_at: prost_types::Timestamp,
    pub verification_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillProgress {
    pub id: String,
    pub skill_name: String,
    pub current_level: u8,
    pub target_level: u8,
    pub progress_percentage: f32,
    pub total_hours_invested: i32,
    pub last_practice_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Certification {
    pub id: String,
    pub skill_name: String,
    pub level_achieved: u8,
    pub certification_type: i32,
    pub issued_at: String,
    pub verification_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tip {
    pub id: String,
    pub from_user_id: String,
    pub to_user_id: String,
    pub course_id: Option<String>,
    pub amount: f64,
    pub currency: String,
    pub created_at: prost_types::Timestamp,
}

// Request types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseRequest {
    pub title: String,
    pub description: String,
    pub creator_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnrollmentRequest {
    pub user_id: String,
    pub course_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressUpdateRequest {
    pub enrollment_id: String,
    pub progress_delta: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialRequest {
    pub user_id: String,
    pub course_id: String,
    pub credential_type: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TipRequest {
    pub from_user_id: String,
    pub to_user_id: String,
    pub course_id: Option<String>,
    pub amount: f64,
    pub currency: String,
}