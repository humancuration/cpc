#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_auth_method_serialization() {
        let auth_methods = vec![
            AuthMethod::Email,
            AuthMethod::Google,
            AuthMethod::Tiktok,
            AuthMethod::Instagram,
            AuthMethod::Passwordless,
        ];

        for auth_method in auth_methods {
            let json = serde_json::to_string(&auth_method).unwrap();
            let deserialized: AuthMethod = serde_json::from_str(&json).unwrap();
            assert_eq!(auth_method, deserialized);
        }
    }

    #[test]
    fn test_user_relationship_type_serialization() {
        let relationship_types = vec![
            UserRelationshipType::Following,
            UserRelationshipType::Blocked,
            UserRelationshipType::Muted,
        ];

        for relationship_type in relationship_types {
            let json = serde_json::to_string(&relationship_type).unwrap();
            let deserialized: UserRelationshipType = serde_json::from_str(&json).unwrap();
            assert_eq!(relationship_type, deserialized);
        }
    }

    #[test]
    fn test_new_user_validation() {
        // Test valid user
        let valid_user = NewUser {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            display_name: Some("Test User".to_string()),
            auth_method: AuthMethod::Email,
            social_id: None,
        };
        assert!(valid_user.validate().is_ok());

        // Test username too short
        let invalid_user = NewUser {
            username: "ab".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            display_name: None,
            auth_method: AuthMethod::Email,
            social_id: None,
        };
        assert_eq!(
            invalid_user.validate(),
            Err("Username must be at least 3 characters".to_string())
        );

        // Test invalid email
        let invalid_user = NewUser {
            username: "testuser".to_string(),
            email: "invalid-email".to_string(),
            password: "password123".to_string(),
            display_name: None,
            auth_method: AuthMethod::Email,
            social_id: None,
        };
        assert_eq!(
            invalid_user.validate(),
            Err("Invalid email format".to_string())
        );

        // Test password too short
        let invalid_user = NewUser {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "pass".to_string(),
            display_name: None,
            auth_method: AuthMethod::Email,
            social_id: None,
        };
        assert_eq!(
            invalid_user.validate(),
            Err("Password must be at least 8 characters".to_string())
        );

        // Test display name too long
        let invalid_user = NewUser {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            display_name: Some("a".repeat(51)),
            auth_method: AuthMethod::Email,
            social_id: None,
        };
        assert_eq!(
            invalid_user.validate(),
            Err("Display name must be 50 characters or less".to_string())
        );
    }

    #[test]
    fn test_user_serialization() {
        let dt = Utc.with_ymd_and_hms(2025, 7, 22, 1, 42, 45).unwrap().with_nanosecond(82000000).unwrap();
        let user = User {
            id: Uuid::nil(), // Using nil UUID for test
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "hashed_password".to_string(),
            auth_method: AuthMethod::Email,
            social_id: Some("social123".to_string()),
            created_at: dt,
            updated_at: dt,
            display_name: Some("Test User".to_string()),
            bio: Some("Test bio".to_string()),
            avatar_url: Some("https://example.com/avatar.jpg".to_string()),
            friends: vec![],
            followers: vec![],
        };

        let json = serde_json::to_string(&user).unwrap();
        assert!(json.contains("\"created_at\":\"2025-07-22T01:42:45.082Z\""));
        assert!(json.contains("\"username\":\"testuser\""));
        assert!(json.contains("\"email\":\"test@example.com\""));
        assert!(json.contains("\"displayName\":\"Test User\""));
        assert!(json.contains("\"bio\":\"Test bio\""));
        assert!(json.contains("\"avatarUrl\":\"https://example.com/avatar.jpg\""));
        assert!(json.contains("\"authMethod\":\"Email\""));
        assert!(json.contains("\"socialId\":\"social123\""));
    }

    #[test]
    fn test_user_profile_creation() {
        let user_id = Uuid::new_v4();
        let profile = UserProfile::new(user_id, "Test User".to_string());
        
        assert_eq!(profile.user_id, user_id);
        assert_eq!(profile.display_name, "Test User");
        assert!(profile.bio.is_none());
        assert!(profile.avatar_url.is_none());
        assert_eq!(profile.cooperative_score.value, 0.0);
    }

    #[test]
    fn test_cooperative_score_calculation() {
        let mut score = CooperativeScore::new();
        
        // Add some contribution factors
        score.update_factor("posts".to_string(), 0.3, 10.0, Some("Number of posts".to_string()));
        score.update_factor("likes".to_string(), 0.2, 25.0, Some("Likes received".to_string()));
        score.update_factor("comments".to_string(), 0.5, 8.0, Some("Quality comments".to_string()));
        
        // Score should be: (0.3 * 10.0) + (0.2 * 25.0) + (0.5 * 8.0) = 3.0 + 5.0 + 4.0 = 12.0
        assert_eq!(score.value, 12.0);
        assert_eq!(score.contribution_factors.len(), 3);
    }

    #[test]
    fn test_user_relationship_creation() {
        let user_id = Uuid::new_v4();
        let target_id = Uuid::new_v4();
        
        let follow_rel = UserRelationship::follow(user_id, target_id);
        assert!(follow_rel.is_following());
        assert!(!follow_rel.is_blocked());
        assert!(!follow_rel.is_muted());
        
        let block_rel = UserRelationship::block(user_id, target_id);
        assert!(!block_rel.is_following());
        assert!(block_rel.is_blocked());
        assert!(!block_rel.is_muted());
        
        let mute_rel = UserRelationship::mute(user_id, target_id);
        assert!(!mute_rel.is_following());
        assert!(!mute_rel.is_blocked());
        assert!(mute_rel.is_muted());
    }

    #[test]
    fn test_relationship_type_serialization() {
        let relationship = UserRelationship::follow(Uuid::new_v4(), Uuid::new_v4());
        let json = serde_json::to_string(&relationship).unwrap();
        assert!(json.contains("\"relationshipType\":\"following\""));
    }
}