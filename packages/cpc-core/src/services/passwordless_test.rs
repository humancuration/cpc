#[cfg(test)]
mod tests {
    use crate::models::user::{User, AuthMethod};
    use crate::repositories::user_repository::UserRepository;
    use crate::services::identity::IdentityService;
    use anyhow::Result;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use uuid::Uuid;
    use async_trait::async_trait;

    /// Mock user repository for testing
    pub struct MockUserRepository {
        users: Arc<Mutex<Vec<User>>>,
    }

    impl MockUserRepository {
        pub fn new() -> Self {
            Self {
                users: Arc::new(Mutex::new(Vec::new())),
            }
        }
    }

    #[async_trait::async_trait]
    impl UserRepository for MockUserRepository {
        async fn create(&self, user: &mut User) -> Result<()> {
            let mut users = self.users.lock().await;
            users.push(user.clone());
            Ok(())
        }

        async fn find_by_id(&self, user_id: Uuid) -> Result<Option<User>> {
            let users = self.users.lock().await;
            Ok(users.iter().find(|u| u.id == user_id).cloned())
        }

        async fn find_many_by_ids(&self, user_ids: &[Uuid]) -> Result<Vec<User>> {
            let users = self.users.lock().await;
            Ok(users
                .iter()
                .filter(|u| user_ids.contains(&u.id))
                .cloned()
                .collect())
        }

        async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
            let users = self.users.lock().await;
            Ok(users.iter().find(|u| u.email == email).cloned())
        }

        async fn update(&self, user: &User) -> Result<()> {
            let mut users = self.users.lock().await;
            if let Some(index) = users.iter().position(|u| u.id == user.id) {
                users[index] = user.clone();
            }
            Ok(())
        }

        async fn delete(&self, user_id: Uuid) -> Result<()> {
            let mut users = self.users.lock().await;
            users.retain(|u| u.id != user_id);
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_passwordless_flow() {
        // Create mock repository
        let user_repo = Box::new(MockUserRepository::new());
        
        // Create identity service with mock repository
        let identity_service = IdentityService::new(
            user_repo,
            "test-secret-key-for-jwt-signing".to_string(),
        ).expect("Failed to create identity service");

        // Test initiate passwordless login
        let email = "test@example.com";
        let user = identity_service
            .initiate_passwordless_login(email)
            .await
            .expect("Failed to initiate passwordless login");

        // Verify user was created
        assert_eq!(user.email, email);
        assert_eq!(user.auth_method, AuthMethod::Passwordless);

        // Test verify passwordless login
        // First, we need to extract the token from the email service (in real scenario)
        // For now, we'll generate a test token manually
        let test_token = identity_service.generate_token(user.id)
            .expect("Failed to generate test token");

        let (verified_user, token) = identity_service
            .verify_passwordless_login(email, &test_token)
            .await
            .expect("Failed to verify passwordless login");

        // Verify the same user is returned
        assert_eq!(verified_user.id, user.id);
        assert_eq!(verified_user.email, email);
        assert!(!token.is_empty());

        // Test invalid token
        let invalid_result = identity_service
            .verify_passwordless_login(email, "invalid-token")
            .await;
        
        assert!(invalid_result.is_err());
    }
}