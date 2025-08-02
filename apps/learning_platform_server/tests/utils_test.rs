#[cfg(test)]
mod tests {
    use learning_platform_server::utils::*;
    use learning_platform_server::error::AppError;
    
    #[test]
    fn test_parse_uuid_valid() {
        let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
        let result = parse_uuid(uuid_str);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_parse_uuid_invalid() {
        let uuid_str = "invalid-uuid";
        let result = parse_uuid(uuid_str);
        assert!(result.is_err());
        
        if let Err(AppError::Validation(msg)) = result {
            assert!(msg.contains("Invalid UUID"));
        } else {
            panic!("Expected Validation error");
        }
    }
    
    #[test]
    fn test_validate_not_empty_valid() {
        let result = validate_not_empty("test", "field");
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_validate_not_empty_invalid() {
        let result = validate_not_empty("", "field");
        assert!(result.is_err());
        
        if let Err(AppError::Validation(msg)) = result {
            assert!(msg.contains("cannot be empty"));
        } else {
            panic!("Expected Validation error");
        }
    }
    
    #[test]
    fn test_validate_positive_valid() {
        let result = validate_positive(5.0, "field");
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_validate_positive_invalid() {
        let result = validate_positive(0.0, "field");
        assert!(result.is_err());
        
        if let Err(AppError::Validation(msg)) = result {
            assert!(msg.contains("must be positive"));
        } else {
            panic!("Expected Validation error");
        }
    }
    
    #[test]
    fn test_validate_email_valid() {
        let result = validate_email("test@example.com");
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_validate_email_invalid() {
        let result = validate_email("invalid-email");
        assert!(result.is_err());
        
        if let Err(AppError::Validation(msg)) = result {
            assert_eq!(msg, "Invalid email format");
        } else {
            panic!("Expected Validation error");
        }
    }
    
    #[test]
    fn test_validate_password_strength_valid() {
        let result = validate_password_strength("password123");
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_validate_password_strength_invalid() {
        let result = validate_password_strength("123");
        assert!(result.is_err());
        
        if let Err(AppError::Validation(msg)) = result {
            assert_eq!(msg, "Password must be at least 6 characters");
        } else {
            panic!("Expected Validation error");
        }
    }
}