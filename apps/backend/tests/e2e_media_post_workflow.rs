use std::sync::Arc;
use tokio::time::{sleep, Duration};
use axum::{
    body::Bytes,
    http::{StatusCode, Request},
    Router,
};
use tower::ServiceExt; // for `oneshot`
use serde_json::json;
use uuid::Uuid;

use backend::{
    models::media::{Media, MediaStatus, MediaType},
    services::{media_service::MediaService, social_service::SocialService},
    repositories::{media_repository::MediaRepository, social::post_repository::PostRepository},
};

mod test_utils {
    use super::*;
    
    pub async fn setup_test_app() -> (Router, Arc<MediaService>, Arc<SocialService>) {
        let pool = setup_test_db().await;
        
        let media_service = Arc::new(MediaService::new(pool.clone()));
        let social_service = Arc::new(SocialService::new(pool.clone()));
        
        let app = backend::create_router(media_service.clone(), social_service.clone());
        
        (app, media_service, social_service)
    }
    
    pub async fn setup_test_db() -> sqlx::PgPool {
        let database_url = std::env::var("TEST_DATABASE_URL")
            .unwrap_or_else(|_| "postgres://localhost/cpc_test".to_string());
        
        sqlx::PgPool::connect(&database_url)
            .await
            .expect("Failed to connect to test database")
    }
}

#[cfg(test)]
mod e2e_media_post_workflow_tests {
    use super::*;
    use test_utils::*;
    
    #[tokio::test]
    async fn test_complete_media_post_workflow() {
        let (app, media_service, social_service) = setup_test_app().await;
        
        // Step 1: User uploads media
        let media_id = Uuid::new_v4();
        let test_file = Bytes::from("test image data");
        
        let upload_response = media_service.upload_media(
            media_id,
            "test_user".to_string(),
            "test_post_image.jpg".to_string(),
            test_file,
            MediaType::Image,
        ).await;
        
        assert!(upload_response.is_ok());
        
        // Step 2: Wait for processing to complete
        let mut attempts = 0;
        let max_attempts = 10;
        
        while attempts < max_attempts {
            let media = media_service.get_media(media_id).await.unwrap();
            if media.status == MediaStatus::Processed {
                break;
            }
            sleep(Duration::from_millis(500)).await;
            attempts += 1;
        }
        
        assert!(attempts < max_attempts, "Media processing timed out");
        
        // Step 3: Create post with processed media
        let post_content = "Check out this amazing photo!".to_string();
        let post_result = social_service.create_post(
            "test_user".to_string(),
            post_content.clone(),
            vec![media_id],
            None, // no tags
        ).await;
        
        assert!(post_result.is_ok());
        let post = post_result.unwrap();
        
        // Step 4: Verify post appears in feed
        let feed_result = social_service.get_user_feed("test_user".to_string(), None, 10).await;
        assert!(feed_result.is_ok());
        
        let feed = feed_result.unwrap();
        assert!(!feed.is_empty());
        
        let created_post = feed.iter().find(|p| p.id == post.id);
        assert!(created_post.is_some());
        
        let post_with_media = created_post.unwrap();
        assert_eq!(post_with_media.content, post_content);
        assert_eq!(post_with_media.media_attachments.len(), 1);
        assert_eq!(post_with_media.media_attachments[0], media_id);
        
        // Step 5: Verify media is properly linked
        let media = media_service.get_media(media_id).await.unwrap();
        assert!(media.post_id.is_some());
        assert_eq!(media.post_id.unwrap(), post.id);
    }
    
    #[tokio::test]
    async fn test_multiple_media_upload_workflow() {
        let (app, media_service, social_service) = setup_test_app().await;
        
        // Upload multiple media files
        let mut media_ids = vec![];
        for i in 0..3 {
            let media_id = Uuid::new_v4();
            let test_file = Bytes::from(format!("test image data {}", i));
            
            let upload_result = media_service.upload_media(
                media_id,
                "test_user".to_string(),
                format!("image_{}.jpg", i),
                test_file,
                MediaType::Image,
            ).await;
            
            assert!(upload_result.is_ok());
            media_ids.push(media_id);
        }
        
        // Wait for all processing to complete
        for media_id in &media_ids {
            let mut attempts = 0;
            while attempts < 10 {
                let media = media_service.get_media(*media_id).await.unwrap();
                if media.status == MediaStatus::Processed {
                    break;
                }
                sleep(Duration::from_millis(500)).await;
                attempts += 1;
            }
            assert!(attempts < 10, "Media {} processing timed out", media_id);
        }
        
        // Create post with multiple media
        let post_result = social_service.create_post(
            "test_user".to_string(),
            "Multiple photos post!".to_string(),
            media_ids.clone(),
            None,
        ).await;
        
        assert!(post_result.is_ok());
        let post = post_result.unwrap();
        
        // Verify all media are linked
        for media_id in media_ids {
            let media = media_service.get_media(media_id).await.unwrap();
            assert_eq!(media.post_id.unwrap(), post.id);
        }
    }
    
    #[tokio::test]
    async fn test_cancel_media_processing_workflow() {
        let (app, media_service, _social_service) = setup_test_app().await;
        
        // Upload media
        let media_id = Uuid::new_v4();
        let test_file = Bytes::from("test image data");
        
        let upload_result = media_service.upload_media(
            media_id,
            "test_user".to_string(),
            "test.jpg".to_string(),
            test_file,
            MediaType::Image,
        ).await;
        
        assert!(upload_result.is_ok());
        
        // Cancel processing
        let cancel_result = media_service.cancel_processing(media_id).await;
        assert!(cancel_result.is_ok());
        
        // Verify media is cancelled
        let media = media_service.get_media(media_id).await.unwrap();
        assert_eq!(media.status, MediaStatus::Cancelled);
    }
    
    #[tokio::test]
    async fn test_retry_failed_processing() {
        let (app, media_service, _social_service) = setup_test_app().await;
        
        // Upload media
        let media_id = Uuid::new_v4();
        let test_file = Bytes::from("test image data");
        
        let upload_result = media_service.upload_media(
            media_id,
            "test_user".to_string(),
            "test.jpg".to_string(),
            test_file,
            MediaType::Image,
        ).await;
        
        assert!(upload_result.is_ok());
        
        // Simulate processing failure
        let _ = media_service.update_media_status(
            media_id,
            MediaStatus::Failed,
            None,
        ).await;
        
        // Retry processing
        let retry_result = media_service.retry_processing(media_id).await;
        assert!(retry_result.is_ok());
        
        // Verify media is back to processing
        let media = media_service.get_media(media_id).await.unwrap();
        assert_eq!(media.status, MediaStatus::Processing);
    }
}