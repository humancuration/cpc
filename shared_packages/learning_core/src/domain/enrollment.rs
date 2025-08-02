use uuid::Uuid;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Enrollment {
    pub id: Uuid,
    pub user_id: Uuid,
    pub course_id: Uuid,
    pub progress: f32,
    pub status: EnrollmentStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EnrollmentStatus {
    Enrolled,
    InProgress,
    Completed,
    Dropped,
}

impl FromStr for EnrollmentStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ENROLLED" => Ok(EnrollmentStatus::Enrolled),
            "IN_PROGRESS" => Ok(EnrollmentStatus::InProgress),
            "COMPLETED" => Ok(EnrollmentStatus::Completed),
            "DROPPED" => Ok(EnrollmentStatus::Dropped),
            _ => Err(()),
        }
    }
}

impl ToString for EnrollmentStatus {
    fn to_string(&self) -> String {
        match self {
            EnrollmentStatus::Enrolled => "ENROLLED".to_string(),
            EnrollmentStatus::InProgress => "IN_PROGRESS".to_string(),
            EnrollmentStatus::Completed => "COMPLETED".to_string(),
            EnrollmentStatus::Dropped => "DROPPED".to_string(),
        }
    }
}

impl Enrollment {
    pub fn new(user_id: Uuid, course_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            course_id,
            progress: 0.0,
            status: EnrollmentStatus::Enrolled,
        }
    }

    pub fn update_progress(&mut self, progress_delta: f32) {
        self.progress += progress_delta;
        if self.progress >= 100.0 {
            self.progress = 100.0;
            self.status = EnrollmentStatus::Completed;
        } else if self.progress > 0.0 {
            self.status = EnrollmentStatus::InProgress;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_enrollment_creation() {
        let user_id = Uuid::new_v4();
        let course_id = Uuid::new_v4();
        
        let enrollment = Enrollment::new(user_id, course_id);
        
        assert_eq!(enrollment.user_id, user_id);
        assert_eq!(enrollment.course_id, course_id);
        assert_eq!(enrollment.progress, 0.0);
        assert_eq!(enrollment.status, EnrollmentStatus::Enrolled);
    }

    #[test]
    fn test_update_progress() {
        let user_id = Uuid::new_v4();
        let course_id = Uuid::new_v4();
        let mut enrollment = Enrollment::new(user_id, course_id);
        
        // Test starting progress
        enrollment.update_progress(25.0);
        assert_eq!(enrollment.progress, 25.0);
        assert_eq!(enrollment.status, EnrollmentStatus::InProgress);
        
        // Test continuing progress
        enrollment.update_progress(50.0);
        assert_eq!(enrollment.progress, 75.0);
        assert_eq!(enrollment.status, EnrollmentStatus::InProgress);
        
        // Test completing course
        enrollment.update_progress(25.0);
        assert_eq!(enrollment.progress, 100.0);
        assert_eq!(enrollment.status, EnrollmentStatus::Completed);
        
        // Test exceeding 100% (should cap at 100%)
        enrollment.update_progress(10.0);
        assert_eq!(enrollment.progress, 100.0);
        assert_eq!(enrollment.status, EnrollmentStatus::Completed);
    }

    #[test]
    fn test_enrollment_status_conversion() {
        // Test from_str
        assert_eq!(EnrollmentStatus::from_str("ENROLLED").unwrap(), EnrollmentStatus::Enrolled);
        assert_eq!(EnrollmentStatus::from_str("IN_PROGRESS").unwrap(), EnrollmentStatus::InProgress);
        assert_eq!(EnrollmentStatus::from_str("COMPLETED").unwrap(), EnrollmentStatus::Completed);
        assert_eq!(EnrollmentStatus::from_str("DROPPED").unwrap(), EnrollmentStatus::Dropped);
        assert!(EnrollmentStatus::from_str("INVALID").is_err());
        
        // Test to_string
        assert_eq!(EnrollmentStatus::Enrolled.to_string(), "ENROLLED");
        assert_eq!(EnrollmentStatus::InProgress.to_string(), "IN_PROGRESS");
        assert_eq!(EnrollmentStatus::Completed.to_string(), "COMPLETED");
        assert_eq!(EnrollmentStatus::Dropped.to_string(), "DROPPED");
    }
}