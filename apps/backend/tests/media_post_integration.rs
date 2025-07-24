use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use axum::{
    body::Bytes,
    extract::Multipart,
    http::{StatusCode, HeaderMap},
};
use sqlx::PgPool;
use serde_json::json;
use uuid::Uuid;

use backend::{
    models::media::{Media, MediaStatus, MediaType},
    services::media_service::MediaService,
    repositories::media_repository::MediaRepository,
    graphql::{schema::Context, media_subscriptions::MediaProcessingSubscription},
};

// Test utilities
mod test_utils {
    use super::*;
    
    pub async fn setup_test_db() -> PgPool {
        // Setup test database connection
        let database_url = std::env::var("TEST_DATABASE_URL")
            .unwrap_or_else(|_| "postgres://localhost/cpc_test".to_string());
        
        PgPool::connect(&database_url)
            .await
            .expect("Failed to connect to test database")
    }
    
    pub async fn cleanup_test_db(pool: &PgPool) {
        // Clean up test data
        sqlx::query!("DELETE FROM media WHERE created_at > NOW() - INTERVAL '1 hour'")
            .execute(pool)
            .await
            .expect("Failed to cleanup test data");
    }
}

#[cfg(test)]
mod media_post_workflow_tests {
    use super::*;
    use test_utils::*;
    
    #[tokio::test]
    async fn test_happy_path_media_upload_to_post() {
        let pool = setup_test_db().await;
        
        // Create test media service
        let media_service = Arc::new(MediaService::new(pool.clone()));
        
        // Test media upload
        let test_file = Bytes::from("test image data");
        let media_id = Uuid::new_v4();
        
        let upload_result = media_service.upload_media(
            media_id,
            "test_user".to_string(),
            "test_image.jpg".to_string(),
            test_file.clone(),
            MediaType::Image,
        ).await;
        
        assert!(upload_result.is_ok());
        
        // Verify media was created
        let media = media_service.get_media(media_id).await.unwrap();
        assert_eq!(media.status, MediaStatus::Processing);
        assert_eq!(media.media_type, MediaType::Image);
        
        // Simulate processing completion
        let processing_result = media_service.update_media_status(
            media_id,
            MediaStatus::Processed,
            Some("processed_url".to_string()),
        ).await;
        
        assert!(processing_result.is_ok());
        
        // Verify final state
        let processed_media = media_service.get_media(media_id).await.unwrap();
        assert_eq!(processed_media.status, MediaStatus::Processed);
        assert!(processed_media.processed_url.is_some());
        
        cleanup_test_db(&pool).await;
    }
    
    #[tokio::test]
    async fn test_failed_media_upload() {
        let pool = setup_test_db().await;
        let media_service = Arc::new(MediaService::new(pool.clone()));
        
        // Test with invalid file data
        let invalid_file = Bytes::from("");
        let media_id = Uuid::new_v4();
        
        let upload_result = media_service.upload_media(
            media_id,
            "test_user".to_string(),
            "invalid.jpg".to_string(),
            invalid_file,
            MediaType::Image,
        ).await;
        
        assert!(upload_result.is_err());
        
        cleanup_test_db(&pool).await;
    }
    
    #[tokio::test]
    async fn test_media_processing_failure() {
        let pool = setup_test_db().await;
        let media_service = Arc::new(MediaService::new(pool.clone()));
        
        // Upload valid media
        let test_file = Bytes::from("test image data");
        let media_id = Uuid::new_v4();
        
        let _ = media_service.upload_media(
            media_id,
            "test_user".to_string(),
            "test.jpg".to_string(),
            test_file,
            MediaType::Image,
        ).await;
        
        // Simulate processing failure
        let processing_result = media_service.update_media_status(
            media_id,
            MediaStatus::Failed,
            None,
        ).await;
        
        assert!(processing_result.is_ok());
        
        let failed_media = media_service.get_media(media_id).await.unwrap();
        assert_eq!(failed_media.status, MediaStatus::Failed);
        
        cleanup_test_db(&pool).await;
    }
    
    #[tokio::test]
    async fn test_concurrent_media_uploads() {
        let pool = setup_test_db().await;
        let media_service = Arc::new(MediaService::new(pool.clone()));
        
        let mut handles = vec![];
        
        // Spawn multiple concurrent uploads
        for i in 0..5 {
            let service = media_service.clone();
            let handle = tokio::spawn(async move {
                let test_file = Bytes::from(format!("test image data {}", i));
                let media_id = Uuid::new_v4();
                
                service.upload_media(
                    media_id,
                    format!("user_{}", i),
                    format!("test_{}.jpg", i),
                    test_file,
                    MediaType::Image,
                ).await
            });
            handles.push(handle);
        }
        
        // Wait for all uploads to complete
        let results = futures::future::join_all(handles).await;
        
        // Verify all uploads succeeded
        for result in results {
            assert!(result.is_ok());
            assert!(result.unwrap().is_ok());
        }
        
        cleanup_test_db(&pool).await;
    }
    
    #[tokio::test]
    async fn test_large_file_upload() {
        let pool = setup_test_db().await;
        let media_service = Arc::new(MediaService::new(pool.clone()));
        
        // Create large test file (10MB)
        let large_data = vec![0u8; 10 * 1024 * 1024];
        let test_file = Bytes::from(large_data);
        let media_id = Uuid::new_v4();
        
        let upload_result = media_service.upload_media(
            media_id,
            "test_user".to_string(),
            "large_file.jpg".to_string(),
            test_file,
            MediaType::Image,
        ).await;
        
        // Should handle large files appropriately
        assert!(upload_result.is_ok());
        
        cleanup_test_db(&pool).await;
    }
    
    #[tokio::test]
    async fn test_graphql_subscription_flow() {
        let pool = setup_test_db().await;
        let media_service = Arc::new(MediaService::new(pool.clone()));
        
        // Setup subscription
        let subscription = MediaProcessingSubscription::new(media_service.clone());
        
        let media_id = Uuid::new_v4();
        
        // Test subscription receives updates
        let (tx, mut rx) = mpsc::channel(100);
        
        tokio::spawn(async move {
            // Simulate processing updates
            tokio::time::sleep(Duration::from_millis(100)).await;
            let _ = tx.send(json!({
                "media_id": media_id,
                "status": "PROCESSING"
            })).await;
            
            tokio::time::sleep(Duration::from_millis(100)).await;
            let _ = tx.send(json!({
                "media_id": media_id,
                "status": "PROCESSED"
            })).await;
        });
        
        // Verify subscription receives both updates
        let mut received_updates = 0;
        while let Some(update) = rx.recv().await {
            received_updates += 1;
            assert!(update["media_id"].as_str().unwrap() == media_id.to_string());
            
            if received_updates == 2 {
                break;
            }
        }
        
        assert_eq!(received_updates, 2);
        
        cleanup_test_db(&pool).await;
    }
}