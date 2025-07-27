#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};
    use uuid::Uuid;

    #[test]
    fn test_visibility_serialization() {
        let visibilities = vec![
            Visibility::Public,
            Visibility::Cooperative,
            Visibility::Private,
        ];

        for visibility in visibilities {
            let json = serde_json::to_string(&visibility).unwrap();
            let deserialized: Visibility = serde_json::from_str(&json).unwrap();
            assert_eq!(visibility, deserialized);
        }
    }

    #[test]
    fn test_media_type_serialization() {
        let media_types = vec![
            MediaType::Image,
            MediaType::Video,
            MediaType::Audio,
            MediaType::Unknown,
        ];

        for media_type in media_types {
            let json = serde_json::to_string(&media_type).unwrap();
            let deserialized: MediaType = serde_json::from_str(&json).unwrap();
            assert_eq!(media_type, deserialized);
        }
    }

    #[test]
    fn test_processing_status_serialization() {
        let processing_statuses = vec![
            ProcessingStatus::Pending,
            ProcessingStatus::Processing,
            ProcessingStatus::Completed,
            ProcessingStatus::Failed,
        ];

        for status in processing_statuses {
            let json = serde_json::to_string(&status).unwrap();
            let deserialized: ProcessingStatus = serde_json::from_str(&json).unwrap();
            assert_eq!(status, deserialized);
        }
    }

    #[test]
    fn test_like_target_type_serialization() {
        let target_types = vec![
            LikeTargetType::Post,
            LikeTargetType::Comment,
        ];

        for target_type in target_types {
            let json = serde_json::to_string(&target_type).unwrap();
            let deserialized: LikeTargetType = serde_json::from_str(&json).unwrap();
            assert_eq!(target_type, deserialized);
        }
    }

    #[test]
    fn test_share_type_serialization() {
        let share_types = vec![
            ShareType::Direct,
            ShareType::Message,
            ShareType::External,
        ];

        for share_type in share_types {
            let json = serde_json::to_string(&share_type).unwrap();
            let deserialized: ShareType = serde_json::from_str(&json).unwrap();
            assert_eq!(share_type, deserialized);
        }
    }

    #[test]
    fn test_post_creation() {
        let author_id = Uuid::new_v4();
        let content = "Test post content".to_string();
        
        let post = Post::new(author_id, content.clone(), Visibility::Public);
        
        assert!(!post.id.is_nil());
        assert_eq!(post.author_id, author_id);
        assert_eq!(post.content, content);
        assert_eq!(post.visibility, Visibility::Public);
        assert!(post.cooperative_id.is_none());
        assert!(post.feed_position.is_none());
        assert!(post.media_items.is_empty());
        assert!(post.tags.is_empty());
        assert!(post.mentions.is_empty());
        assert!(post.reply_to_post_id.is_none());
        assert!(post.repost_of_post_id.is_none());
        assert!(post.edit_history.is_empty());
    }

    #[test]
    fn test_post_reply_creation() {
        let author_id = Uuid::new_v4();
        let content = "Test reply content".to_string();
        let original_post_id = Uuid::new_v4();
        
        let post = Post::new_reply(author_id, content.clone(), original_post_id, Visibility::Public);
        
        assert!(!post.id.is_nil());
        assert_eq!(post.author_id, author_id);
        assert_eq!(post.content, content);
        assert_eq!(post.visibility, Visibility::Public);
        assert_eq!(post.reply_to_post_id, Some(original_post_id));
        assert!(post.is_reply());
    }

    #[test]
    fn test_post_repost_creation() {
        let author_id = Uuid::new_v4();
        let original_post_id = Uuid::new_v4();
        let repost_message = Some("Great post!".to_string());
        
        let post = Post::new_repost(author_id, original_post_id, repost_message.clone(), Visibility::Public);
        
        assert!(!post.id.is_nil());
        assert_eq!(post.author_id, author_id);
        assert_eq!(post.content, repost_message.clone().unwrap_or_default());
        assert_eq!(post.visibility, Visibility::Public);
        assert_eq!(post.repost_of_post_id, Some(original_post_id));
        assert!(post.is_repost());
    }

    #[test]
    fn test_post_media_management() {
        let mut post = Post::new(Uuid::new_v4(), "Test post".to_string(), Visibility::Public);
        let media_item = MediaItem::new(post.id, "https://example.com/image.jpg".to_string(), MediaType::Image);
        
        post.add_media_item(media_item.clone());
        
        assert_eq!(post.media_items.len(), 1);
        assert_eq!(post.media_items[0].url, "https://example.com/image.jpg");
    }

    #[test]
    fn test_post_tag_management() {
        let mut post = Post::new(Uuid::new_v4(), "Test post".to_string(), Visibility::Public);
        
        post.add_tag("rust".to_string());
        post.add_tag("testing".to_string());
        post.add_tag("rust".to_string()); // Duplicate should not be added
        
        assert_eq!(post.tags.len(), 2);
        assert!(post.tags.contains(&"rust".to_string()));
        assert!(post.tags.contains(&"testing".to_string()));
    }

    #[test]
    fn test_post_mention_management() {
        let mut post = Post::new(Uuid::new_v4(), "Test post".to_string(), Visibility::Public);
        let user_id1 = Uuid::new_v4();
        let user_id2 = Uuid::new_v4();
        
        post.add_mention(user_id1);
        post.add_mention(user_id2);
        post.add_mention(user_id1); // Duplicate should not be added
        
        assert_eq!(post.mentions.len(), 2);
        assert!(post.mentions.contains(&user_id1));
        assert!(post.mentions.contains(&user_id2));
    }

    #[test]
    fn test_post_editing() {
        let mut post = Post::new(Uuid::new_v4(), "Original content".to_string(), Visibility::Public);
        let edit_reason = Some("Fixed typo".to_string());
        
        post.edit_content("Updated content".to_string(), edit_reason.clone());
        
        assert_eq!(post.content, "Updated content");
        assert_eq!(post.edit_history.len(), 1);
        assert_eq!(post.edit_history[0].previous_content, "Original content");
        assert_eq!(post.edit_history[0].edit_reason, edit_reason);
    }

    #[test]
    fn test_comment_creation() {
        let post_id = Uuid::new_v4();
        let author_id = Uuid::new_v4();
        let content = "Test comment".to_string();
        
        let comment = Comment::new(post_id, author_id, content.clone());
        
        assert!(!comment.id.is_nil());
        assert_eq!(comment.post_id, post_id);
        assert_eq!(comment.author_id, author_id);
        assert_eq!(comment.content, content);
        assert!(comment.parent_comment_id.is_none());
        assert_eq!(comment.thread_depth, 0);
        assert!(comment.mentions.is_empty());
        assert!(comment.edit_history.is_empty());
    }

    #[test]
    fn test_comment_reply_creation() {
        let post_id = Uuid::new_v4();
        let author_id = Uuid::new_v4();
        let content = "Test reply".to_string();
        let parent_comment_id = Uuid::new_v4();
        let thread_depth = 2;
        
        let comment = Comment::new_reply(post_id, author_id, content.clone(), parent_comment_id, thread_depth);
        
        assert!(!comment.id.is_nil());
        assert_eq!(comment.post_id, post_id);
        assert_eq!(comment.author_id, author_id);
        assert_eq!(comment.content, content);
        assert_eq!(comment.parent_comment_id, Some(parent_comment_id));
        assert_eq!(comment.thread_depth, thread_depth);
        assert!(comment.is_reply());
    }

    #[test]
    fn test_comment_mention_management() {
        let mut comment = Comment::new(Uuid::new_v4(), Uuid::new_v4(), "Test comment".to_string());
        let user_id1 = Uuid::new_v4();
        let user_id2 = Uuid::new_v4();
        
        comment.add_mention(user_id1);
        comment.add_mention(user_id2);
        comment.add_mention(user_id1); // Duplicate should not be added
        
        assert_eq!(comment.mentions.len(), 2);
        assert!(comment.mentions.contains(&user_id1));
        assert!(comment.mentions.contains(&user_id2));
    }

    #[test]
    fn test_comment_editing() {
        let mut comment = Comment::new(Uuid::new_v4(), Uuid::new_v4(), "Original comment".to_string());
        let edit_reason = Some("Fixed grammar".to_string());
        
        comment.edit_content("Updated comment".to_string(), edit_reason.clone());
        
        assert_eq!(comment.content, "Updated comment");
        assert_eq!(comment.edit_history.len(), 1);
        assert_eq!(comment.edit_history[0].previous_content, "Original comment");
        assert_eq!(comment.edit_history[0].edit_reason, edit_reason);
    }

    #[test]
    fn test_media_item_creation() {
        let post_id = Uuid::new_v4();
        let url = "https://example.com/video.mp4".to_string();
        
        let media_item = MediaItem::new(post_id, url.clone(), MediaType::Video);
        
        assert!(!media_item.id.is_nil());
        assert_eq!(media_item.post_id, post_id);
        assert_eq!(media_item.url, url);
        assert_eq!(media_item.media_type, MediaType::Video);
        assert_eq!(media_item.processing_status, ProcessingStatus::Pending);
        assert!(media_item.file_size.is_none());
        assert!(media_item.duration.is_none());
        assert!(media_item.width.is_none());
        assert!(media_item.height.is_none());
        assert!(media_item.thumbnail_url.is_none());
        assert!(media_item.alt_text.is_none());
    }

    #[test]
    fn test_media_item_metadata() {
        let mut media_item = MediaItem::new(Uuid::new_v4(), "https://example.com/image.jpg".to_string(), MediaType::Image);
        
        media_item.set_dimensions(1920, 1080);
        media_item.set_file_size(1024000);
        media_item.set_thumbnail_url("https://example.com/thumb.jpg".to_string());
        media_item.set_alt_text("Test image".to_string());
        media_item.set_processing_status(ProcessingStatus::Completed);
        
        assert_eq!(media_item.width, Some(1920));
        assert_eq!(media_item.height, Some(1080));
        assert_eq!(media_item.file_size, Some(1024000));
        assert_eq!(media_item.thumbnail_url, Some("https://example.com/thumb.jpg".to_string()));
        assert_eq!(media_item.alt_text, Some("Test image".to_string()));
        assert_eq!(media_item.processing_status, ProcessingStatus::Completed);
    }

    #[test]
    fn test_like_creation() {
        let user_id = Uuid::new_v4();
        let post_id = Uuid::new_v4();
        let comment_id = Uuid::new_v4();
        
        let post_like = Like::new_post_like(user_id, post_id);
        let comment_like = Like::new_comment_like(user_id, comment_id);
        
        assert!(!post_like.id.is_nil());
        assert_eq!(post_like.user_id, user_id);
        assert_eq!(post_like.target_type, LikeTargetType::Post);
        assert_eq!(post_like.target_id, post_id);
        assert!(post_like.is_post_like());
        assert!(!post_like.is_comment_like());
        
        assert!(!comment_like.id.is_nil());
        assert_eq!(comment_like.user_id, user_id);
        assert_eq!(comment_like.target_type, LikeTargetType::Comment);
        assert_eq!(comment_like.target_id, comment_id);
        assert!(comment_like.is_comment_like());
        assert!(!comment_like.is_post_like());
    }

    #[test]
    fn test_share_creation() {
        let user_id = Uuid::new_v4();
        let post_id = Uuid::new_v4();
        
        let direct_share = Share::new_direct_share(user_id, post_id, Some("Check this out!".to_string()));
        let message_share = Share::new_message_share(user_id, post_id, None);
        let external_share = Share::new_external_share(user_id, post_id, Some("Shared externally".to_string()));
        
        assert!(!direct_share.id.is_nil());
        assert_eq!(direct_share.user_id, user_id);
        assert_eq!(direct_share.post_id, post_id);
        assert_eq!(direct_share.share_type, ShareType::Direct);
        assert_eq!(direct_share.share_message, Some("Check this out!".to_string()));
        
        assert!(!message_share.id.is_nil());
        assert_eq!(message_share.user_id, user_id);
        assert_eq!(message_share.post_id, post_id);
        assert_eq!(message_share.share_type, ShareType::Message);
        assert_eq!(message_share.share_message, None);
        
        assert!(!external_share.id.is_nil());
        assert_eq!(external_share.user_id, user_id);
        assert_eq!(external_share.post_id, post_id);
        assert_eq!(external_share.share_type, ShareType::External);
        assert_eq!(external_share.share_message, Some("Shared externally".to_string()));
    }

    #[test]
    fn test_repost_creation() {
        let user_id = Uuid::new_v4();
        let original_post_id = Uuid::new_v4();
        let repost_message = Some("This is important!".to_string());
        
        let repost = Repost::new(user_id, original_post_id, repost_message.clone());
        
        assert!(!repost.id.is_nil());
        assert_eq!(repost.user_id, user_id);
        assert_eq!(repost.original_post_id, original_post_id);
        assert_eq!(repost.repost_message, repost_message);
    }

    #[test]
    fn test_reply_creation() {
        let original_post_id = Uuid::new_v4();
        let reply_post_id = Uuid::new_v4();
        let author_id = Uuid::new_v4();
        
        let reply = Reply::new(original_post_id, reply_post_id, author_id);
        
        assert!(!reply.id.is_nil());
        assert_eq!(reply.original_post_id, original_post_id);
        assert_eq!(reply.reply_post_id, reply_post_id);
        assert_eq!(reply.author_id, author_id);
    }
}