use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct AcademicCredential {
    pub id: Uuid,
    pub user_id: Uuid,
    pub course_id: Uuid,
    pub credential_type: CredentialType,
    pub issued_at: DateTime<Utc>,
    pub verification_code: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CredentialType {
    Certificate,
    MicroDegree,
    Degree,
    Badge,
}

impl FromStr for CredentialType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CERTIFICATE" => Ok(CredentialType::Certificate),
            "MICRO_DEGREE" => Ok(CredentialType::MicroDegree),
            "DEGREE" => Ok(CredentialType::Degree),
            "BADGE" => Ok(CredentialType::Badge),
            _ => Err(()),
        }
    }
}

impl ToString for CredentialType {
    fn to_string(&self) -> String {
        match self {
            CredentialType::Certificate => "CERTIFICATE".to_string(),
            CredentialType::MicroDegree => "MICRO_DEGREE".to_string(),
            CredentialType::Degree => "DEGREE".to_string(),
            CredentialType::Badge => "BADGE".to_string(),
        }
    }
}

impl AcademicCredential {
    pub fn new(user_id: Uuid, course_id: Uuid, credential_type: CredentialType) -> Self {
        let verification_code = format!("VC-{}", Uuid::new_v4().to_string()[..8].to_uppercase());
        
        Self {
            id: Uuid::new_v4(),
            user_id,
            course_id,
            credential_type,
            issued_at: Utc::now(),
            verification_code,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_credential_creation() {
        let user_id = Uuid::new_v4();
        let course_id = Uuid::new_v4();
        let credential_type = CredentialType::Certificate;
        
        let credential = AcademicCredential::new(user_id, course_id, credential_type.clone());
        
        assert_eq!(credential.user_id, user_id);
        assert_eq!(credential.course_id, course_id);
        assert_eq!(credential.credential_type, credential_type);
        assert!(!credential.verification_code.is_empty());
        assert!(credential.verification_code.starts_with("VC-"));
    }

    #[test]
    fn test_credential_type_conversion() {
        // Test from_str
        assert_eq!(CredentialType::from_str("CERTIFICATE").unwrap(), CredentialType::Certificate);
        assert_eq!(CredentialType::from_str("MICRO_DEGREE").unwrap(), CredentialType::MicroDegree);
        assert_eq!(CredentialType::from_str("DEGREE").unwrap(), CredentialType::Degree);
        assert_eq!(CredentialType::from_str("BADGE").unwrap(), CredentialType::Badge);
        assert!(CredentialType::from_str("INVALID").is_err());
        
        // Test to_string
        assert_eq!(CredentialType::Certificate.to_string(), "CERTIFICATE");
        assert_eq!(CredentialType::MicroDegree.to_string(), "MICRO_DEGREE");
        assert_eq!(CredentialType::Degree.to_string(), "DEGREE");
        assert_eq!(CredentialType::Badge.to_string(), "BADGE");
    }
}