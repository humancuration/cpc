#[cfg(test)]
mod tests {
    use learning_platform_server::database::models::*;
    use uuid::Uuid;
    use chrono::Utc;
    
    #[test]
    fn test_course_model() {
        let course = Course {
            id: Uuid::new_v4(),
            title: "Test Course".to_string(),
            description: "Test Description".to_string(),
            creator_id: Uuid::new_v4(),
            created_at: Utc::now(),
        };
        
        assert_eq!(course.title, "Test Course");
        assert_eq!(course.description, "Test Description");
    }
    
    #[test]
    fn test_module_model() {
        let module = Module {
            id: Uuid::new_v4(),
            course_id: Uuid::new_v4(),
            title: "Test Module".to_string(),
            order_index: 1,
            created_at: Utc::now(),
        };
        
        assert_eq!(module.title, "Test Module");
        assert_eq!(module.order_index, 1);
    }
    
    #[test]
    fn test_lesson_model() {
        let lesson = Lesson {
            id: Uuid::new_v4(),
            module_id: Uuid::new_v4(),
            title: "Test Lesson".to_string(),
            content: "Test Content".to_string(),
            media_url: "http://example.com/video.mp4".to_string(),
            order_index: 1,
            created_at: Utc::now(),
        };
        
        assert_eq!(lesson.title, "Test Lesson");
        assert_eq!(lesson.content, "Test Content");
        assert_eq!(lesson.media_url, "http://example.com/video.mp4");
    }
    
    #[test]
    fn test_enrollment_model() {
        let enrollment = Enrollment {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            course_id: Uuid::new_v4(),
            progress: 50.0,
            status: "IN_PROGRESS".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        assert_eq!(enrollment.progress, 50.0);
        assert_eq!(enrollment.status, "IN_PROGRESS");
    }
    
    #[test]
    fn test_credential_model() {
        let credential = AcademicCredential {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            course_id: Uuid::new_v4(),
            credential_type: "CERTIFICATE".to_string(),
            issued_at: Utc::now(),
            verification_code: "VC-12345678".to_string(),
            created_at: Utc::now(),
        };
        
        assert_eq!(credential.credential_type, "CERTIFICATE");
        assert_eq!(credential.verification_code, "VC-12345678");
    }
    
    #[test]
    fn test_tip_model() {
        let tip = Tip {
            id: Uuid::new_v4(),
            from_user_id: Uuid::new_v4(),
            to_user_id: Uuid::new_v4(),
            course_id: Some(Uuid::new_v4()),
            amount: 10.0,
            currency: "USD".to_string(),
            created_at: Utc::now(),
        };
        
        assert_eq!(tip.amount, 10.0);
        assert_eq!(tip.currency, "USD");
    }
    
    #[test]
    fn test_user_model() {
        let user = User {
            id: Uuid::new_v4(),
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "hashed_password".to_string(),
            created_at: Utc::now(),
        };
        
        assert_eq!(user.username, "testuser");
        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.password_hash, "hashed_password");
    }
}