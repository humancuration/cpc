use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CertificationType {
    CourseCompletion,
    PeerEndorsement,
    SkillAssessment,
    ProjectReview,
    PortfolioReview,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Certification {
    pub id: Uuid,
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    #[validate(length(min = 1, max = 100))]
    pub issuing_organization: String,
    pub issue_date: NaiveDate,
    pub user_id: Uuid,
    pub skill_id: Option<Uuid>,
    pub certification_type: CertificationType,
    pub level_achieved: Option<u8>, // 0-4 (Beginner, Intermediate, Advanced, Expert, Master)
    pub verification_code: String,
    pub created_at: DateTime<Utc>,
}

impl Certification {
    pub fn new(
        name: String,
        issuing_organization: String,
        issue_date: NaiveDate,
        user_id: Uuid,
        skill_id: Option<Uuid>,
        certification_type: CertificationType,
        level_achieved: Option<u8>,
    ) -> Self {
        let verification_code = format!(
            "{}-{}-{:06}",
            issuing_organization
                .chars()
                .take(4)
                .collect::<String>()
                .to_uppercase(),
            issue_date.format("%Y"),
            rand::random::<u32>() % 1000000
        );

        Self {
            id: Uuid::new_v4(),
            name,
            issuing_organization,
            issue_date,
            user_id,
            skill_id,
            certification_type,
            level_achieved: level_achieved.map(|l| l.min(4)),
            verification_code,
            created_at: Utc::now(),
        }
    }

    pub fn verify_code(&self, code: &str) -> bool {
        self.verification_code == code
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_certification_creation() {
        let issue_date = NaiveDate::from_ymd_opt(2024, 7, 15).unwrap();
        let user_id = Uuid::new_v4();
        let skill_id = Uuid::new_v4();

        let cert = Certification::new(
            "Rust Programming Certification".to_string(),
            "CPC Cooperative".to_string(),
            issue_date,
            user_id,
            Some(skill_id),
            CertificationType::SkillAssessment,
            Some(2), // Advanced
        );

        assert_eq!(cert.name, "Rust Programming Certification");
        assert_eq!(cert.issuing_organization, "CPC Cooperative");
        assert_eq!(cert.issue_date, issue_date);
        assert_eq!(cert.user_id, user_id);
        assert_eq!(cert.skill_id, Some(skill_id));
        assert_eq!(cert.certification_type, CertificationType::SkillAssessment);
        assert_eq!(cert.level_achieved, Some(2));
        assert!(!cert.verification_code.is_empty());
    }

    #[test]
    fn test_certification_verification() {
        let issue_date = NaiveDate::from_ymd_opt(2024, 7, 15).unwrap();
        let user_id = Uuid::new_v4();

        let cert = Certification::new(
            "Rust Programming Certification".to_string(),
            "CPC Cooperative".to_string(),
            issue_date,
            user_id,
            None,
            CertificationType::CourseCompletion,
            None,
        );

        assert!(cert.verify_code(&cert.verification_code));
        assert!(!cert.verify_code("INVALID-CODE"));
    }

    #[test]
    fn test_level_achieved_capping() {
        let issue_date = NaiveDate::from_ymd_opt(2024, 7, 15).unwrap();
        let user_id = Uuid::new_v4();

        let cert = Certification::new(
            "Rust Programming Certification".to_string(),
            "CPC Cooperative".to_string(),
            issue_date,
            user_id,
            None,
            CertificationType::SkillAssessment,
            Some(10), // Should be capped to 4
        );

        assert_eq!(cert.level_achieved, Some(4));
    }

    #[test]
    fn test_certification_validation() {
        let issue_date = NaiveDate::from_ymd_opt(2024, 7, 15).unwrap();
        let user_id = Uuid::new_v4();
        let skill_id = Uuid::new_v4();

        let cert = Certification::new(
            "Rust Programming Certification".to_string(),
            "CPC Cooperative".to_string(),
            issue_date,
            user_id,
            Some(skill_id),
            CertificationType::SkillAssessment,
            Some(2),
        );

        let validation = cert.validate();
        assert!(validation.is_ok());
    }
}