#[cfg(test)]
mod tests {
    use yapper::domain::user::{User, Role};
    use yapper::domain::credentials::Credentials;
    use yapper::domain::auth_error::AuthError;
    use uuid::Uuid;

    #[test]
    fn test_user_creation() {
        let email = "test@example.com".to_string();
        let password_hash = "hashed_password".to_string();
        
        let user = User::new(email.clone(), password_hash.clone());
        
        assert_eq!(user.email, email);
        assert_eq!(user.password_hash, password_hash);
        assert!(!user.is_verified);
        assert_eq!(user.roles.len(), 1);
        assert_eq!(user.roles[0], Role::User);
    }

    #[test]
    fn test_credentials_creation() {
        let email = "test@example.com".to_string();
        let password = "password123".to_string();
        
        let credentials = Credentials::new(email.clone(), password.clone());
        
        assert_eq!(credentials.email, email);
        assert_eq!(credentials.password, password);
    }
}