#[cfg(test)]
mod tests {
    use social_integration::cross_posting::CrossPostingService;
    use uuid::Uuid;

    #[test]
    fn test_cross_posting_service_creation() {
        let service = CrossPostingService::new();
        assert!(true); // Simple test to ensure the service can be created
    }

    #[test]
    fn test_cross_post_to_yapper() {
        let service = CrossPostingService::new();
        let allat_post_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        
        let result = service.cross_post_to_yapper(allat_post_id, user_id);
        
        assert!(result.is_ok());
    }

    #[test]
    fn test_cross_post_to_allat() {
        let service = CrossPostingService::new();
        let yapper_post_id = Uuid::new_v4();
        let community_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        
        let result = service.cross_post_to_allat(yapper_post_id, community_id, user_id);
        
        assert!(result.is_ok());
    }
}