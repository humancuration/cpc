#[cfg(test)]
mod tests {
    use learning_platform_server::error::{AppError, AppResult};
    use tonic::Status;
    
    #[test]
    fn test_app_error_from_sqlx_error() {
        let sqlx_error = sqlx::Error::RowNotFound;
        let app_error: AppError = sqlx_error.into();
        
        match app_error {
            AppError::Database(_) => (), // Expected
            _ => panic!("Expected Database error"),
        }
    }
    
    #[test]
    fn test_app_error_from_config_error() {
        let config_error = config::ConfigError::NotFound("test".to_string());
        let app_error: AppError = config_error.into();
        
        match app_error {
            AppError::Config(_) => (), // Expected
            _ => panic!("Expected Config error"),
        }
    }
    
    #[test]
    fn test_app_error_to_status() {
        let app_error = AppError::Auth("test".to_string());
        let status: Status = app_error.into();
        
        assert_eq!(status.code(), tonic::Code::Unauthenticated);
    }
    
    #[test]
    fn test_app_result() {
        let result: AppResult<i32> = Ok(42);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
        
        let result: AppResult<i32> = Err(AppError::Internal("test".to_string()));
        assert!(result.is_err());
    }
}