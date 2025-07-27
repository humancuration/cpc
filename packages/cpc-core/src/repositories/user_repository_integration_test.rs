//! Integration tests for the UserRepository
//!
//! These tests require a SQLite database to run.
//! Set the TEST_DATABASE_URL environment variable to point to a test database.

use std::sync::Arc;
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;

use crate::repositories::user_repository::{UserRepository, SqliteUserRepository};
use crate::models::user::{User, AuthMethod, NewUser};

/// Setup a test database connection
async fn setup_test_db() -> SqlitePool {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| ":memory:".to_string());
    
    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("Failed to connect to test database");
    
    // Run migrations if using a file database, or create tables if using in-memory
    if database_url == ":memory:" {
        sqlx::query!(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY,
                username TEXT NOT NULL UNIQUE,
                email TEXT NOT NULL UNIQUE,
                password_hash TEXT NOT NULL,
                auth_method TEXT NOT NULL,
                social_id TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                display_name TEXT,
                bio TEXT,
                avatar_url TEXT,
                friends TEXT NOT NULL,
                followers TEXT NOT NULL
            )
            "#
        )
        .execute(&pool)
        .await
        .expect("Failed to create users table");
    }
    
    pool
}

/// Cleanup test data
async fn cleanup_test_data(pool: &SqlitePool) {
    sqlx::query!("DELETE FROM users")
        .execute(pool)
        .await
        .expect("Failed to cleanup users");
}

#[tokio::test]
async fn test_create_and_find_user() {
    let pool = setup_test_db().await;
    let repository = SqliteUserRepository::new(pool.clone());
    
    // Create a test user
    let mut user = User {
        id: Uuid::new_v4(),
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password_hash: "hashed_password".to_string(),
        auth_method: AuthMethod::Email,
        social_id: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        display_name: Some("Test User".to_string()),
        bio: Some("Test bio".to_string()),
        avatar_url: Some("https://example.com/avatar.jpg".to_string()),
        friends: vec![],
        followers: vec![],
    };
    
    // Test create
    let result = repository.create(&mut user).await;
    assert!(result.is_ok());
    
    // Test find by ID
    let retrieved_user = repository.find_by_id(user.id).await.unwrap();
    assert!(retrieved_user.is_some());
    let retrieved_user = retrieved_user.unwrap();
    assert_eq!(retrieved_user.id, user.id);
    assert_eq!(retrieved_user.username, user.username);
    assert_eq!(retrieved_user.email, user.email);
    
    // Test find by email
    let retrieved_user = repository.find_by_email(&user.email).await.unwrap();
    assert!(retrieved_user.is_some());
    let retrieved_user = retrieved_user.unwrap();
    assert_eq!(retrieved_user.id, user.id);
    assert_eq!(retrieved_user.email, user.email);
    
    cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_find_many_users_by_ids() {
    let pool = setup_test_db().await;
    let repository = SqliteUserRepository::new(pool.clone());
    
    // Create test users
    let user1_id = Uuid::new_v4();
    let mut user1 = User {
        id: user1_id,
        username: "testuser1".to_string(),
        email: "test1@example.com".to_string(),
        password_hash: "hashed_password".to_string(),
        auth_method: AuthMethod::Email,
        social_id: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        display_name: Some("Test User 1".to_string()),
        bio: Some("Test bio 1".to_string()),
        avatar_url: Some("https://example.com/avatar1.jpg".to_string()),
        friends: vec![],
        followers: vec![],
    };
    
    let user2_id = Uuid::new_v4();
    let mut user2 = User {
        id: user2_id,
        username: "testuser2".to_string(),
        email: "test2@example.com".to_string(),
        password_hash: "hashed_password".to_string(),
        auth_method: AuthMethod::Email,
        social_id: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        display_name: Some("Test User 2".to_string()),
        bio: Some("Test bio 2".to_string()),
        avatar_url: Some("https://example.com/avatar2.jpg".to_string()),
        friends: vec![],
        followers: vec![],
    };
    
    repository.create(&mut user1).await.unwrap();
    repository.create(&mut user2).await.unwrap();
    
    // Test find many by IDs
    let user_ids = vec![user1_id, user2_id];
    let users = repository.find_many_by_ids(&user_ids).await.unwrap();
    assert_eq!(users.len(), 2);
    
    let user_ids_found: Vec<Uuid> = users.iter().map(|u| u.id).collect();
    assert!(user_ids_found.contains(&user1_id));
    assert!(user_ids_found.contains(&user2_id));
    
    cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_find_all_users() {
    let pool = setup_test_db().await;
    let repository = SqliteUserRepository::new(pool.clone());
    
    // Create test users
    let mut user1 = User {
        id: Uuid::new_v4(),
        username: "testuser1".to_string(),
        email: "test1@example.com".to_string(),
        password_hash: "hashed_password".to_string(),
        auth_method: AuthMethod::Email,
        social_id: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        display_name: Some("Test User 1".to_string()),
        bio: Some("Test bio 1".to_string()),
        avatar_url: Some("https://example.com/avatar1.jpg".to_string()),
        friends: vec![],
        followers: vec![],
    };
    
    let mut user2 = User {
        id: Uuid::new_v4(),
        username: "testuser2".to_string(),
        email: "test2@example.com".to_string(),
        password_hash: "hashed_password".to_string(),
        auth_method: AuthMethod::Email,
        social_id: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        display_name: Some("Test User 2".to_string()),
        bio: Some("Test bio 2".to_string()),
        avatar_url: Some("https://example.com/avatar2.jpg".to_string()),
        friends: vec![],
        followers: vec![],
    };
    
    repository.create(&mut user1).await.unwrap();
    repository.create(&mut user2).await.unwrap();
    
    // Test find all
    let users = repository.find_all().await.unwrap();
    assert!(users.len() >= 2);
    
    cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_update_user() {
    let pool = setup_test_db().await;
    let repository = SqliteUserRepository::new(pool.clone());
    
    // Create a test user
    let mut user = User {
        id: Uuid::new_v4(),
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password_hash: "hashed_password".to_string(),
        auth_method: AuthMethod::Email,
        social_id: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        display_name: Some("Test User".to_string()),
        bio: Some("Test bio".to_string()),
        avatar_url: Some("https://example.com/avatar.jpg".to_string()),
        friends: vec![],
        followers: vec![],
    };
    
    repository.create(&mut user).await.unwrap();
    
    // Update the user
    let updated_display_name = "Updated Test User".to_string();
    user.display_name = Some(updated_display_name.clone());
    user.updated_at = Utc::now();
    
    let result = repository.update(&user).await;
    assert!(result.is_ok());
    
    // Verify update
    let retrieved_user = repository.find_by_id(user.id).await.unwrap().unwrap();
    assert_eq!(retrieved_user.display_name, Some(updated_display_name));
    
    cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_delete_user() {
    let pool = setup_test_db().await;
    let repository = SqliteUserRepository::new(pool.clone());
    
    // Create a test user
    let mut user = User {
        id: Uuid::new_v4(),
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password_hash: "hashed_password".to_string(),
        auth_method: AuthMethod::Email,
        social_id: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        display_name: Some("Test User".to_string()),
        bio: Some("Test bio".to_string()),
        avatar_url: Some("https://example.com/avatar.jpg".to_string()),
        friends: vec![],
        followers: vec![],
    };
    
    repository.create(&mut user).await.unwrap();
    
    // Verify user exists
    let retrieved_user = repository.find_by_id(user.id).await.unwrap();
    assert!(retrieved_user.is_some());
    
    // Delete user
    let result = repository.delete(user.id).await;
    assert!(result.is_ok());
    
    // Verify user is deleted
    let retrieved_user = repository.find_by_id(user.id).await.unwrap();
    assert!(retrieved_user.is_none());
    
    cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_find_user_by_social_id() {
    let pool = setup_test_db().await;
    let repository = SqliteUserRepository::new(pool.clone());
    
    // Create a test user with social ID
    let social_id = "google_123456789".to_string();
    let mut user = User {
        id: Uuid::new_v4(),
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password_hash: "hashed_password".to_string(),
        auth_method: AuthMethod::Google,
        social_id: Some(social_id.clone()),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        display_name: Some("Test User".to_string()),
        bio: Some("Test bio".to_string()),
        avatar_url: Some("https://example.com/avatar.jpg".to_string()),
        friends: vec![],
        followers: vec![],
    };
    
    repository.create(&mut user).await.unwrap();
    
    // Test find by social ID
    let retrieved_user = repository.find_user_by_social_id("google", &social_id).await.unwrap();
    assert!(retrieved_user.is_some());
    let retrieved_user = retrieved_user.unwrap();
    assert_eq!(retrieved_user.id, user.id);
    assert_eq!(retrieved_user.social_id, Some(social_id));
    
    cleanup_test_data(&pool).await;
}