use std::sync::Arc;
use sqlx::PgPool;
use tokio::sync::OnceCell;

use crate::services::{media_service::MediaService, social_service::SocialService};

static TEST_DB: OnceCell<PgPool> = OnceCell::const_new();

pub async fn setup_test_db() -> PgPool {
    TEST_DB.get_or_init(|| async {
        let database_url = std::env::var("TEST_DATABASE_URL")
            .unwrap_or_else(|_| "postgres://localhost/cpc_test".to_string());
        
        PgPool::connect(&database_url)
            .await
            .expect("Failed to connect to test database")
    })
    .await
    .clone()
}

pub async fn cleanup_test_db(pool: &PgPool) {
    // Clean up test data in reverse order of dependencies
    sqlx::query!("DELETE FROM posts WHERE created_at > NOW() - INTERVAL '1 hour'")
        .execute(pool)
        .await
        .expect("Failed to cleanup posts");
    
    sqlx::query!("DELETE FROM media WHERE created_at > NOW() - INTERVAL '1 hour'")
        .execute(pool)
        .await
        .expect("Failed to cleanup media");
}

pub async fn setup_test_services() -> (Arc<MediaService>, Arc<SocialService>, PgPool) {
    let pool = setup_test_db().await;
    let media_service = Arc::new(MediaService::new(pool.clone()));
    let social_service = Arc::new(SocialService::new(pool.clone()));
    
    (media_service, social_service, pool)
}

pub async fn create_test_user(pool: &PgPool, user_id: &str) {
    sqlx::query!(
        "INSERT INTO users (id, username, email) VALUES ($1, $2, $3)",
        user_id,
        format!("test_user_{}", user_id),
        format!("test{}@example.com", user_id)
    )
    .execute(pool)
    .await
    .expect("Failed to create test user");
}

pub async fn create_test_post(
    pool: &PgPool,
    user_id: &str,
    content: &str,
    media_ids: Vec<Uuid>
) -> Uuid {
    let post_id = Uuid::new_v4();
    
    sqlx::query!(
        "INSERT INTO posts (id, user_id, content) VALUES ($1, $2, $3)",
        post_id,
        user_id,
        content
    )
    .execute(pool)
    .await
    .expect("Failed to create test post");
    
    // Link media to post
    for media_id in media_ids {
        sqlx::query!(
            "UPDATE media SET post_id = $1 WHERE id = $2",
            post_id,
            media_id
        )
        .execute(pool)
        .await
        .expect("Failed to link media to post");
    }
    
    post_id
}

pub async fn create_test_media(
    pool: &PgPool,
    user_id: &str,
    filename: &str,
    media_type: &str,
    status: &str
) -> Uuid {
    let media_id = Uuid::new_v4();
    
    sqlx::query!(
        "INSERT INTO media (id, user_id, filename, media_type, status) VALUES ($1, $2, $3, $4, $5)",
        media_id,
        user_id,
        filename,
        media_type,
        status
    )
    .execute(pool)
    .await
    .expect("Failed to create test media");
    
    media_id
}

pub fn generate_test_image(size: usize) -> Vec<u8> {
    // Generate a simple test image
    let mut image = vec![0; size];
    for i in 0..size {
        image[i] = (i % 256) as u8;
    }
    image
}

pub fn generate_large_test_file(size_mb: usize) -> Vec<u8> {
    let size = size_mb * 1024 * 1024;
    vec![0; size]
}

pub async fn wait_for_condition<F, Fut>(
    condition: F,
    timeout: Duration,
    interval: Duration,
) -> bool
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = bool>,
{
    let start = std::time::Instant::now();
    
    while start.elapsed() < timeout {
        if condition().await {
            return true;
        }
        tokio::time::sleep(interval).await;
    }
    
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_setup_cleanup() {
        let pool = setup_test_db().await;
        
        // Create test data
        let user_id = "test_user_123";
        create_test_user(&pool, user_id).await;
        
        let media_id = create_test_media(&pool, user_id, "test.jpg", "image/jpeg", "PROCESSING").await;
        let post_id = create_test_post(&pool, user_id, "Test post", vec![media_id]).await;
        
        // Verify data exists
        let media = sqlx::query!("SELECT * FROM media WHERE id = $1", media_id)
            .fetch_one(&pool)
            .await
            .unwrap();
        
        assert_eq!(media.user_id, user_id);
        assert_eq!(media.post_id.unwrap(), post_id);
        
        // Cleanup
        cleanup_test_db(&pool).await;
        
        // Verify cleanup
        let media_count = sqlx::query!("SELECT COUNT(*) as count FROM media WHERE id = $1", media_id)
            .fetch_one(&pool)
            .await
            .unwrap();
        
        assert_eq!(media_count.count, 0);
    }
}