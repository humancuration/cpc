//! Test to verify database migrations work correctly

use sqlx::{PgPool, Row};
use std::env;

#[tokio::test]
async fn test_migrations() -> Result<(), Box<dyn std::error::Error>> {
    // Use test database URL
    let database_url = env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://localhost/allat_test".to_string());
    
    // Create connection pool
    let pool = PgPool::connect(&database_url).await?;
    
    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;
    
    // Verify tables exist by querying them
    let tables = vec!["communities", "users", "posts", "media_assets", "votes"];
    
    for table in tables {
        let count: i64 = sqlx::query(&format!("SELECT COUNT(*) as count FROM {}", table))
            .fetch_one(&pool)
            .await?
            .get("count");
        
        // Just verify we can query the table (count should be 0 for empty tables)
        assert!(count >= 0);
    }
    
    // Verify indexes exist by querying the pg_indexes table
    let indexes = vec![
        "idx_posts_community_id",
        "idx_posts_user_id", 
        "idx_posts_parent_id",
        "idx_posts_created_at",
        "idx_media_assets_post_id",
        "idx_votes_post_id",
        "idx_votes_user_id"
    ];
    
    for index in indexes {
        let exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM pg_indexes WHERE indexname = $1)"
        )
        .bind(index)
        .fetch_one(&pool)
        .await?;
        
        assert!(exists, "Index {} should exist", index);
    }
    
    Ok(())
}