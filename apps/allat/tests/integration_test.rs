#[cfg(test)]
mod tests {
    use allat::domain::community::Community;
    use allat::domain::post::Post;
    use allat::domain::comment::Comment;
    use allat::domain::media_asset::{MediaAsset, MediaType};
    use uuid::Uuid;
    use chrono::Utc;

    #[test]
    fn test_community_creation() {
        let name = "Test Community".to_string();
        let description = "A test community".to_string();
        let rules = vec!["Rule 1".to_string(), "Rule 2".to_string()];
        
        let community = Community::new(name.clone(), description.clone(), rules.clone());
        
        assert_eq!(community.name, name);
        assert_eq!(community.description, description);
        assert_eq!(community.rules, rules);
    }

    #[test]
    fn test_post_creation() {
        let community_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let title = "Test Post".to_string();
        let content = "This is a test post".to_string();
        let media_assets = vec![MediaAsset::new(
            "http://example.com/image.jpg".to_string(),
            MediaType::Image,
            Some("Test image".to_string())
        )];
        
        let post = Post::new(community_id, user_id, title.clone(), content.clone(), None, media_assets.clone());
        
        assert_eq!(post.community_id, community_id);
        assert_eq!(post.user_id, user_id);
        assert_eq!(post.title, title);
        assert_eq!(post.content, content);
        assert!(post.parent_id.is_none());
        assert_eq!(post.media_assets.len(), 1);
    }

    #[test]
    fn test_comment_creation() {
        let post_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let content = "This is a test comment".to_string();
        
        let comment = Comment::new(post_id, user_id, content.clone(), None);
        
        assert_eq!(comment.post_id, post_id);
        assert_eq!(comment.user_id, user_id);
        assert_eq!(comment.content, content);
        assert!(comment.parent_id.is_none());
    }

    #[test]
    fn test_media_asset_creation() {
        let url = "http://example.com/image.jpg".to_string();
        let media_type = MediaType::Image;
        let alt_text = Some("Test image".to_string());
        
        let media_asset = MediaAsset::new(url.clone(), media_type.clone(), alt_text.clone());
        
        assert_eq!(media_asset.url, url);
        assert_eq!(media_asset.media_type, media_type);
        assert_eq!(media_asset.alt_text, alt_text);
    }
}