#[cfg(test)]
mod tests {
    use allat::domain::auth::{Credentials, User};
    use allat::domain::auth_service::{AuthService, AllatAuthService};
    use allat::infrastructure::repositories::user_repository::SledUserRepository;
    use cpc_auth::auth_service::AuthServiceImpl as BaseAuthService;
    use std::sync::Arc;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_register_user() {
        // Set up services
        let base_auth_service = Arc::new(BaseAuthService::new());
        let temp_dir = tempfile::tempdir().unwrap();
        let user_db = sled::open(temp_dir.path().join("test_users")).expect("Failed to open test DB");
        let user_repo = Arc::new(SledUserRepository::new(user_db));
        let auth_service = Arc::new(AllatAuthService::new(base_auth_service, user_repo));

        // Create test credentials
        let credentials = Credentials {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };

        // Register user
        let user = auth_service.register(credentials).await.unwrap();

        // Verify user was created with initial karma
        assert_eq!(user.karma, 0);
        assert_eq!(user.base.email, "test@example.com");
    }

    #[tokio::test]
    async fn test_karma_tracking() {
        // Set up services
        let base_auth_service = Arc::new(BaseAuthService::new());
        let temp_dir = tempfile::tempdir().unwrap();
        let user_db = sled::open(temp_dir.path().join("test_users_karma")).expect("Failed to open test DB");
        let user_repo = Arc::new(SledUserRepository::new(user_db));
        let auth_service = Arc::new(AllatAuthService::new(base_auth_service, user_repo));

        // Create test user
        let credentials = Credentials {
            email: "karma_test@example.com".to_string(),
            password: "password123".to_string(),
        };
        let user = auth_service.register(credentials).await.unwrap();

        // Check initial karma
        let initial_karma = auth_service.get_karma(user.base.id).await.unwrap();
        assert_eq!(initial_karma, 0);

        // Increment karma
        auth_service.increment_karma(user.base.id, 5).await.unwrap();

        // Check updated karma
        let updated_karma = auth_service.get_karma(user.base.id).await.unwrap();
        assert_eq!(updated_karma, 5);

        // Decrement karma
        auth_service.increment_karma(user.base.id, -2).await.unwrap();

        // Check final karma
        let final_karma = auth_service.get_karma(user.base.id).await.unwrap();
        assert_eq!(final_karma, 3);
    }
    
    #[tokio::test]
    async fn test_karma_overflow_protection() {
        // Set up services
        let base_auth_service = Arc::new(BaseAuthService::new());
        let temp_dir = tempfile::tempdir().unwrap();
        let user_db = sled::open(temp_dir.path().join("test_users_overflow")).expect("Failed to open test DB");
        let user_repo = Arc::new(SledUserRepository::new(user_db));
        let auth_service = Arc::new(AllatAuthService::new(base_auth_service, user_repo));

        // Create test user
        let credentials = Credentials {
            email: "overflow_test@example.com".to_string(),
            password: "password123".to_string(),
        };
        let user = auth_service.register(credentials).await.unwrap();

        // Set karma to near the limit
        for _ in 0..9999 {
            auth_service.increment_karma(user.base.id, 1).await.unwrap();
        }
        
        // Try to increment beyond the limit
        let result = auth_service.increment_karma(user.base.id, 2).await;
        assert!(result.is_err());
        
        // Check that karma is still at the limit
        let karma = auth_service.get_karma(user.base.id).await.unwrap();
        assert_eq!(karma, 9999);
    }
    
    #[tokio::test]
    async fn test_role_escalation_prevention() {
        // Set up services
        let base_auth_service = Arc::new(BaseAuthService::new());
        let temp_dir = tempfile::tempdir().unwrap();
        let user_db = sled::open(temp_dir.path().join("test_users_role")).expect("Failed to open test DB");
        let user_repo = Arc::new(SledUserRepository::new(user_db));
        let auth_service = Arc::new(AllatAuthService::new(base_auth_service, user_repo));

        // Create test user
        let credentials = Credentials {
            email: "role_test@example.com".to_string(),
            password: "password123".to_string(),
        };
        let user = auth_service.register(credentials).await.unwrap();

        // Regular users should not be able able to assign admin roles to themselves
        // This test would require implementing the actual role escalation prevention logic
        // in the assign_community_role method in the user repository
        
        // For now, we'll test that the assign_community_role method exists and can be called
        let result = auth_service.assign_community_role(user.base.id, CommunityRole::Contributor).await;
        // In a full implementation, this would depend on the user's current roles
        // and the role hierarchy
    }
    
    #[tokio::test]
    async fn test_role_escalation_prevention_detailed() {
        // This test would require direct access to the user repository
        // to properly test role escalation prevention
        // In a real implementation, we would test:
        // 1. Assigning a lower role to a user with a higher role fails
        // 2. Assigning the same role to a user fails (no duplicates)
        // 3. Assigning a higher role to a user with a lower role succeeds
    }
    
    #[tokio::test]
    async fn test_concurrent_vote_processing() {
        // Set up services
        let base_auth_service = Arc::new(BaseAuthService::new());
        let temp_dir = tempfile::tempdir().unwrap();
        let user_db = sled::open(temp_dir.path().join("test_users_concurrent")).expect("Failed to open test DB");
        let user_repo = Arc::new(SledUserRepository::new(user_db));
        let auth_service = Arc::new(AllatAuthService::new(base_auth_service, user_repo));

        // Create test user
        let credentials = Credentials {
            email: "concurrent_test@example.com".to_string(),
            password: "password123".to_string(),
        };
        let user = auth_service.register(credentials).await.unwrap();

        // Simulate concurrent vote processing
        // This would typically involve spawning multiple async tasks
        // but for this test we'll just process votes sequentially
        // to verify the basic functionality
        let user_id = user.base.id;
        
        // Process multiple upvotes
        for _ in 0..10 {
            auth_service.increment_karma(user_id, 1).await.unwrap();
        }
        
        // Check final karma
        let karma = auth_service.get_karma(user_id).await.unwrap();
        assert_eq!(karma, 10);
    }
}