#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{Course, Enrollment, AcademicCredential, Tip, EnrollmentStatus, CredentialType};
    use uuid::Uuid;
    use async_trait::async_trait;
    use std::collections::HashMap;

    // Mock repositories for testing
    struct MockCourseRepository;
    
    #[async_trait]
    impl CourseRepository for MockCourseRepository {
        async fn create_course(&self, course: &Course) -> Result<Course> {
            Ok(course.clone())
        }
        
        async fn find_by_id(&self, _id: Uuid) -> Result<Course> {
            let course = Course::new(
                "Test Course".to_string(),
                "Test Description".to_string(),
                Uuid::new_v4()
            );
            Ok(course)
        }
    }

    struct MockEnrollmentRepository {
        enrollments: HashMap<Uuid, Enrollment>,
    }
    
    impl MockEnrollmentRepository {
        fn new() -> Self {
            Self {
                enrollments: HashMap::new(),
            }
        }
    }
    
    #[async_trait]
    impl EnrollmentRepository for MockEnrollmentRepository {
        async fn enroll_user(&self, enrollment: &Enrollment) -> Result<Enrollment> {
            Ok(enrollment.clone())
        }
        
        async fn find_by_id(&self, id: Uuid) -> Result<Enrollment> {
            self.enrollments.get(&id)
                .cloned()
                .ok_or(LearningPlatformError::EnrollmentNotFound)
        }
        
        async fn update_progress(&self, enrollment: &Enrollment) -> Result<Enrollment> {
            Ok(enrollment.clone())
        }
        
        async fn find_by_user_and_course(&self, user_id: Uuid, course_id: Uuid) -> Result<Enrollment> {
            for enrollment in self.enrollments.values() {
                if enrollment.user_id == user_id && enrollment.course_id == course_id {
                    return Ok(enrollment.clone());
                }
            }
            Err(LearningPlatformError::EnrollmentNotFound)
        }
    }

    struct MockCredentialRepository;
    
    #[async_trait]
    impl CredentialRepository for MockCredentialRepository {
        async fn issue_credential(&self, credential: &AcademicCredential) -> Result<AcademicCredential> {
            Ok(credential.clone())
        }
    }

    struct MockTipRepository;
    
    #[async_trait]
    impl TipRepository for MockTipRepository {
        async fn create_tip(&self, tip: &Tip) -> Result<Tip> {
            Ok(tip.clone())
        }
    }

    #[tokio::test]
    async fn test_create_course() {
        let course_repo = Box::new(MockCourseRepository);
        let enrollment_repo = Box::new(MockEnrollmentRepository::new());
        let credential_repo = Box::new(MockCredentialRepository);
        let tip_repo = Box::new(MockTipRepository);
        
        let service = LearningPlatformService::new(
            course_repo,
            enrollment_repo,
            credential_repo,
            tip_repo,
        );
        
        let title = "Rust Programming".to_string();
        let description = "Learn Rust programming language".to_string();
        let creator_id = Uuid::new_v4();
        
        let course = service.create_course(title.clone(), description.clone(), creator_id).await.unwrap();
        
        assert_eq!(course.title, title);
        assert_eq!(course.description, description);
        assert_eq!(course.creator_id, creator_id);
    }

    #[tokio::test]
    async fn test_enroll_user() {
        let course_repo = Box::new(MockCourseRepository);
        let enrollment_repo = Box::new(MockEnrollmentRepository::new());
        let credential_repo = Box::new(MockCredentialRepository);
        let tip_repo = Box::new(MockTipRepository);
        
        let service = LearningPlatformService::new(
            course_repo,
            enrollment_repo,
            credential_repo,
            tip_repo,
        );
        
        let user_id = Uuid::new_v4();
        let course_id = Uuid::new_v4();
        
        let enrollment = service.enroll_user(user_id, course_id).await.unwrap();
        
        assert_eq!(enrollment.user_id, user_id);
        assert_eq!(enrollment.course_id, course_id);
        assert_eq!(enrollment.progress, 0.0);
        assert_eq!(enrollment.status, EnrollmentStatus::Enrolled);
    }

    #[tokio::test]
    async fn test_update_progress() {
        let course_repo = Box::new(MockCourseRepository);
        let mut enrollment_repo = MockEnrollmentRepository::new();
        let credential_repo = Box::new(MockCredentialRepository);
        let tip_repo = Box::new(MockTipRepository);
        
        // Create an enrollment to update
        let enrollment_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let course_id = Uuid::new_v4();
        let enrollment = Enrollment::new(user_id, course_id);
        enrollment_repo.enrollments.insert(enrollment_id, enrollment);
        
        let service = LearningPlatformService::new(
            course_repo,
            Box::new(enrollment_repo),
            credential_repo,
            tip_repo,
        );
        
        let updated_enrollment = service.update_progress(enrollment_id, 50.0).await.unwrap();
        
        assert_eq!(updated_enrollment.progress, 50.0);
        assert_eq!(updated_enrollment.status, EnrollmentStatus::InProgress);
    }

    #[tokio::test]
    async fn test_issue_credential_success() {
        let course_repo = Box::new(MockCourseRepository);
        let mut enrollment_repo = MockEnrollmentRepository::new();
        let credential_repo = Box::new(MockCredentialRepository);
        let tip_repo = Box::new(MockTipRepository);
        
        // Create a completed enrollment
        let user_id = Uuid::new_v4();
        let course_id = Uuid::new_v4();
        let mut enrollment = Enrollment::new(user_id, course_id);
        enrollment.status = EnrollmentStatus::Completed;
        enrollment.progress = 100.0;
        enrollment_repo.enrollments.insert(Uuid::new_v4(), enrollment);
        
        let service = LearningPlatformService::new(
            course_repo,
            Box::new(enrollment_repo),
            credential_repo,
            tip_repo,
        );
        
        let credential = service.issue_credential(user_id, course_id, CredentialType::Certificate).await.unwrap();
        
        assert_eq!(credential.user_id, user_id);
        assert_eq!(credential.course_id, course_id);
        assert_eq!(credential.credential_type, CredentialType::Certificate);
    }

    #[tokio::test]
    async fn test_issue_credential_not_completed() {
        let course_repo = Box::new(MockCourseRepository);
        let mut enrollment_repo = MockEnrollmentRepository::new();
        let credential_repo = Box::new(MockCredentialRepository);
        let tip_repo = Box::new(MockTipRepository);
        
        // Create an in-progress enrollment
        let user_id = Uuid::new_v4();
        let course_id = Uuid::new_v4();
        let mut enrollment = Enrollment::new(user_id, course_id);
        enrollment.status = EnrollmentStatus::InProgress;
        enrollment.progress = 50.0;
        enrollment_repo.enrollments.insert(Uuid::new_v4(), enrollment);
        
        let service = LearningPlatformService::new(
            course_repo,
            Box::new(enrollment_repo),
            credential_repo,
            tip_repo,
        );
        
        let result = service.issue_credential(user_id, course_id, CredentialType::Certificate).await;
        
        assert!(result.is_err());
        match result.unwrap_err() {
            LearningPlatformError::InvalidOperation(_) => (),
            _ => panic!("Expected InvalidOperation error"),
        }
    }

    #[tokio::test]
    async fn test_tip_educator_success() {
        let course_repo = Box::new(MockCourseRepository);
        let enrollment_repo = Box::new(MockEnrollmentRepository::new());
        let credential_repo = Box::new(MockCredentialRepository);
        let tip_repo = Box::new(MockTipRepository);
        
        let service = LearningPlatformService::new(
            course_repo,
            enrollment_repo,
            credential_repo,
            tip_repo,
        );
        
        let from_user_id = Uuid::new_v4();
        let to_user_id = Uuid::new_v4();
        let amount = 10.0;
        let currency = "USD".to_string();
        let course_id = Some(Uuid::new_v4());
        
        let tip = service.tip_educator(from_user_id, to_user_id, amount, currency.clone(), course_id).await.unwrap();
        
        assert_eq!(tip.from_user_id, from_user_id);
        assert_eq!(tip.to_user_id, to_user_id);
        assert_eq!(tip.amount, amount);
        assert_eq!(tip.currency, currency);
        assert_eq!(tip.course_id, course_id);
    }

    #[tokio::test]
    async fn test_tip_educator_invalid_amount() {
        let course_repo = Box::new(MockCourseRepository);
        let enrollment_repo = Box::new(MockEnrollmentRepository::new());
        let credential_repo = Box::new(MockCredentialRepository);
        let tip_repo = Box::new(MockTipRepository);
        
        let service = LearningPlatformService::new(
            course_repo,
            enrollment_repo,
            credential_repo,
            tip_repo,
        );
        
        let from_user_id = Uuid::new_v4();
        let to_user_id = Uuid::new_v4();
        let amount = -5.0; // Invalid negative amount
        let currency = "USD".to_string();
        
        let result = service.tip_educator(from_user_id, to_user_id, amount, currency, None).await;
        
        assert!(result.is_err());
        match result.unwrap_err() {
            LearningPlatformError::InvalidOperation(_) => (),
            _ => panic!("Expected InvalidOperation error"),
        }
    }
}