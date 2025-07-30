#[cfg(test)]
mod tests {
    use allat::domain::community::Community;
    use allat::domain::post::Post;
    use uuid::Uuid;

    #[test]
    fn test_community_creation() {
        let name = "Test Community".to_string();
        let description = "A test community".to_string();
        let creator_id = Uuid::new_v4();
        
        let community = Community::new(name.clone(), description.clone(), creator_id);
        
        assert_eq!(community.name, name);
        assert_eq!(community.description, description);
        assert_eq!(community.moderator_ids.len(), 1);
        assert_eq!(community.moderator_ids[0], creator_id);
    }

    #[test]
    fn test_post_creation() {
        let community_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let title = "Test Post".to_string();
        let content = "This is a test post".to_string();
        
        let post = Post::new(community_id, user_id, title.clone(), content.clone(), None);
        
        assert_eq!(post.community_id, community_id);
        assert_eq!(post.user_id, user_id);
        assert_eq!(post.title, title);
        assert_eq!(post.content, content);
        assert!(post.parent_id.is_none());
    }
}