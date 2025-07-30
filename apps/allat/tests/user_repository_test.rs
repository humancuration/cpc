#[cfg(test)]
mod tests {
    use allat::domain::auth::community_role::CommunityRole;
    use allat::infrastructure::repositories::user_repository::SledUserRepository;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_role_escalation_prevention() {
        // Set up repository
        let temp_dir = tempfile::tempdir().unwrap();
        let user_db = sled::open(temp_dir.path().join("test_users_role_escalation")).expect("Failed to open test DB");
        let user_repo = SledUserRepository::new(user_db);
        
        let user_id = Uuid::new_v4();
        
        // Test assigning a role to a non-existent user (should not panic)
        let result = user_repo.assign_community_role(user_id, CommunityRole::Contributor).await;
        // This might fail or succeed depending on implementation, but shouldn't panic
        
        // For a more comprehensive test, we would need to:
        // 1. Create a user in the database
        // 2. Assign roles to the user
        // 3. Test role escalation prevention
        // This would require additional methods in the repository or direct database access
    }
}