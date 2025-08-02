use uuid::Uuid;
use crate::error::AppError;

/// Parse a UUID string, returning an AppError if invalid
pub fn parse_uuid(uuid_str: &str) -> Result<Uuid, AppError> {
    Uuid::parse_str(uuid_str)
        .map_err(|_| AppError::Validation(format!("Invalid UUID: {}", uuid_str)))
}

/// Validate that a string is not empty
pub fn validate_not_empty(value: &str, field_name: &str) -> Result<(), AppError> {
    if value.trim().is_empty() {
        Err(AppError::Validation(format!("{} cannot be empty", field_name)))
    } else {
        Ok(())
    }
}

/// Validate that a value is positive
pub fn validate_positive<T>(value: T, field_name: &str) -> Result<(), AppError>
where
    T: PartialOrd + Default + std::fmt::Display,
{
    if value <= T::default() {
        Err(AppError::Validation(format!("{} must be positive", field_name)))
    } else {
        Ok(())
    }
}

/// Validate email format (simple validation)
pub fn validate_email(email: &str) -> Result<(), AppError> {
    if !email.contains('@') || !email.contains('.') {
        Err(AppError::Validation("Invalid email format".to_string()))
    } else {
        Ok(())
    }
}

/// Validate password strength
pub fn validate_password_strength(password: &str) -> Result<(), AppError> {
    if password.len() < 6 {
        Err(AppError::Validation("Password must be at least 6 characters".to_string()))
    } else {
        Ok(())
    }
}