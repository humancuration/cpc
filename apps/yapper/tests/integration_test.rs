#[cfg(test)]
mod tests {
    use yapper::domain::post::{Yap, YapError};
    use yapper::domain::user_profile::UserProfile;
    use uuid::Uuid;

    #[test]
    fn test_yap_creation() {
        let user_id = Uuid::new_v4();
        let content = "This is a test yap".to_string();
        
        let result = Yap::new(user_id, content.clone());
        
        assert!(result.is_ok());
        let yap = result.unwrap();
        assert_eq!(yap.user_id, user_id);
        assert_eq!(yap.content, content);
        assert_eq!(yap.like_count, 0);
        assert_eq!(yap.share_count, 0);
    }

    #[test]
    fn test_yap_creation_content_too_long() {
        let user_id = Uuid::new_v4();
        let content = "a".repeat(281); // Exceeds 280 character limit
        
        let result = Yap::new(user_id, content);
        
        assert!(matches!(result, Err(YapError::ContentTooLong)));
    }

    #[test]
    fn test_yap_creation_empty_content() {
        let user_id = Uuid::new_v4();
        let content = "".to_string();
        
        let result = Yap::new(user_id, content);
        
        assert!(matches!(result, Err(YapError::ContentEmpty)));
    }

    #[test]
    fn test_user_profile_creation() {
        let user_id = Uuid::new_v4();
        let display_name = "Test User".to_string();
        let bio = "This is a test user".to_string();
        
        let profile = UserProfile::new(user_id, display_name.clone(), bio.clone());
        
        assert_eq!(profile.user_id, user_id);
        assert_eq!(profile.display_name, display_name);
        assert_eq!(profile.bio, bio);
        assert_eq!(profile.followers_count, 0);
        assert_eq!(profile.following_count, 0);
    }
}