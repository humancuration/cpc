//! Integration tests for the SocialRepository
//!
//! These tests require a SQLite database to run.
//! Set the TEST_DATABASE_URL environment variable to point to a test database.

use std::sync::Arc;
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;

use crate::repositories::social_repository::{SocialRepository, SqliteSocialRepository, CreatePostData, CreateCommentData};
use crate::models::social::{
    Post, Comment, Like, Share, Repost, Follow, Block, Mute, UserActivity,
    Visibility, LikeTargetType, ShareType, ActivityType, FeedType
};

/// Setup a test database connection
async fn setup_test_db() -> SqlitePool {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| ":memory:".to_string());
    
    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("Failed to connect to test database");
    
    // Run migrations if using a file database, or create tables if using in-memory
    if database_url == ":memory:" {
        // Create posts table
        sqlx::query!(
            r#"
            CREATE TABLE IF NOT EXISTS posts (
                id TEXT PRIMARY KEY,
                author_id TEXT NOT NULL,
                content TEXT NOT NULL,
                visibility TEXT NOT NULL,
                cooperative_id TEXT,
                feed_position INTEGER,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                reply_to_post_id TEXT,
                repost_of_post_id TEXT
            )
            "#
        )
        .execute(&pool)
        .await
        .expect("Failed to create posts table");
        
        // Create post_tags table
        sqlx::query!(
            r#"
            CREATE TABLE IF NOT EXISTS post_tags (
                post_id TEXT NOT NULL,
                tag TEXT NOT NULL,
                PRIMARY KEY (post_id, tag)
            )
            "#
        )
        .execute(&pool)
        .await
        .expect("Failed to create post_tags table");
        
        // Create post_mentions table
        sqlx::query!(
            r#"
            CREATE TABLE IF NOT EXISTS post_mentions (
                post_id TEXT NOT NULL,
                user_id TEXT NOT NULL,
                PRIMARY KEY (post_id, user_id)
            )
            "#
        )
        .execute(&pool)
        .await
        .expect("Failed to create post_mentions table");
        
        // Create comments table
        sqlx::query!(
            r#"
            CREATE TABLE IF NOT EXISTS comments (
                id TEXT PRIMARY KEY,
                post_id TEXT NOT NULL,
                author_id TEXT NOT NULL,
                content TEXT NOT NULL,
                parent_comment_id TEXT,
                thread_depth INTEGER NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )
            "#
        )
        .execute(&pool)
        .await
        .expect("Failed to create comments table");
        
        // Create comment_mentions table
        sqlx::query!(
            r#"
            CREATE TABLE IF NOT EXISTS comment_mentions (
                comment_id TEXT NOT NULL,
                user_id TEXT NOT NULL,
                PRIMARY KEY (comment_id, user_id)
            )
            "#
        )
        .execute(&pool)
        .await
        .expect("Failed to create comment_mentions table");
        
        // Create likes table
        sqlx::query!(
            r#"
            CREATE TABLE IF NOT EXISTS likes (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                target_type TEXT NOT NULL,
                target_id TEXT NOT NULL,
                created_at TEXT NOT NULL
            )
            "#
        )
        .execute(&pool)
        .await
        .expect("Failed to create likes table");
        
        // Create shares table
        sqlx::query!(
            r#"
            CREATE TABLE IF NOT EXISTS shares (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                post_id TEXT NOT NULL,
                share_message TEXT,
                share_type TEXT NOT NULL,
                created_at TEXT NOT NULL
            )
            "#
        )
        .execute(&pool)
        .await
        .expect("Failed to create shares table");
        
        // Create reposts table
        sqlx::query!(
            r#"
            CREATE TABLE IF NOT EXISTS reposts (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                original_post_id TEXT NOT NULL,
                repost_message TEXT,
                created_at TEXT NOT NULL
            )
            "#
        )
        .execute(&pool)
        .await
        .expect("Failed to create reposts table");
        
        // Create follows table
        sqlx::query!(
            r#"
            CREATE TABLE IF NOT EXISTS follows (
                id TEXT PRIMARY KEY,
                follower_id TEXT NOT NULL,
                followed_id TEXT NOT NULL,
                is_mutual INTEGER NOT NULL,
                notification_enabled INTEGER NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )
            "#
        )
        .execute(&pool)
        .await
        .expect("Failed to create follows table");
        
        // Create blocks table
        sqlx::query!(
            r#"
            CREATE TABLE IF NOT EXISTS blocks (
                id TEXT PRIMARY KEY,
                blocker_id TEXT NOT NULL,
                blocked_id TEXT NOT NULL,
                block_reason TEXT,
                is_permanent INTEGER NOT NULL,
                expires_at TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )
            "#
        )
        .execute(&pool)
        .await
        .expect("Failed to create blocks table");
        
        // Create mutes table
        sqlx::query!(
            r#"
            CREATE TABLE IF NOT EXISTS mutes (
                id TEXT PRIMARY KEY,
                muter_id TEXT NOT NULL,
                muted_id TEXT NOT NULL,
                mute_type TEXT NOT NULL,
                expires_at TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )
            "#
        )
        .execute(&pool)
        .await
        .expect("Failed to create mutes table");
        
        // Create user_activities table
        sqlx::query!(
            r#"
            CREATE TABLE IF NOT EXISTS user_activities (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                activity_type TEXT NOT NULL,
                target_type TEXT,
                target_id TEXT,
                metadata TEXT,
                created_at TEXT NOT NULL
            )
            "#
        )
        .execute(&pool)
        .await
        .expect("Failed to create user_activities table");
    }
    
    pool
}

/// Cleanup test data
async fn cleanup_test_data(pool: &SqlitePool) {
    sqlx::query!("DELETE FROM user_activities").execute(pool).await.expect("Failed to cleanup user_activities");
    sqlx::query!("DELETE FROM mutes").execute(pool).await.expect("Failed to cleanup mutes");
    sqlx::query!("DELETE FROM blocks").execute(pool).await.expect("Failed to cleanup blocks");
    sqlx::query!("DELETE FROM follows").execute(pool).await.expect("Failed to cleanup follows");
    sqlx::query!("DELETE FROM reposts").execute(pool).await.expect("Failed to cleanup reposts");
    sqlx::query!("DELETE FROM shares").execute(pool).await.expect("Failed to cleanup shares");
    sqlx::query!("DELETE FROM likes").execute(pool).await.expect("Failed to cleanup likes");
    sqlx::query!("DELETE FROM comment_mentions").execute(pool).await.expect("Failed to cleanup comment_mentions");
    sqlx::query!("DELETE FROM comments").execute(pool).await.expect("Failed to cleanup comments");
    sqlx::query!("DELETE FROM post_mentions").execute(pool).await.expect("Failed to cleanup post_mentions");
    sqlx::query!("DELETE FROM post_tags").execute(pool).await.expect("Failed to cleanup post_tags");
    sqlx::query!("DELETE FROM posts").execute(pool).await.expect("Failed to cleanup posts");
}

#[tokio::test]
async fn test_create_and_find_post() {
    let pool = setup_test_db().await;
    let repository = SqliteSocialRepository::new(pool.clone());
    
    // Create a test post
    let author_id = Uuid::new_v4();
    let data = CreatePostData {
        author_id,
        content: "Test post content".to_string(),
        visibility: Visibility::Public,
        cooperative_id: None,
        tags: vec!["test".to_string(), "social".to_string()],
        mentions: vec![],
        reply_to_post_id: None,
        repost_of_post_id: None,
    };
    
    // Test create post
    let post = repository.create_post(data).await.unwrap();
    assert_eq!(post.author_id, author_id);
    assert_eq!(post.content, "Test post content");
    assert_eq!(post.visibility, Visibility::Public);
    assert_eq!(post.tags.len(), 2);
    
    // Test find post by ID
    let retrieved_post = repository.find_post_by_id(post.id).await.unwrap();
    assert!(retrieved_post.is_some());
    let retrieved_post = retrieved_post.unwrap();
    assert_eq!(retrieved_post.id, post.id);
    assert_eq!(retrieved_post.content, post.content);
    assert_eq!(retrieved_post.tags.len(), 2);
    
    cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_create_and_find_comment() {
    let pool = setup_test_db().await;
    let repository = SqliteSocialRepository::new(pool.clone());
    
    // Create a test post first
    let author_id = Uuid::new_v4();
    let post_data = CreatePostData {
        author_id,
        content: "Test post content".to_string(),
        visibility: Visibility::Public,
        cooperative_id: None,
        tags: vec![],
        mentions: vec![],
        reply_to_post_id: None,
        repost_of_post_id: None,
    };
    
    let post = repository.create_post(post_data).await.unwrap();
    
    // Create a test comment
    let commenter_id = Uuid::new_v4();
    let data = CreateCommentData {
        post_id: post.id,
        author_id: commenter_id,
        content: "Test comment content".to_string(),
        parent_comment_id: None,
        mentions: vec![],
    };
    
    // Test create comment
    let comment = repository.create_comment(data).await.unwrap();
    assert_eq!(comment.post_id, post.id);
    assert_eq!(comment.author_id, commenter_id);
    assert_eq!(comment.content, "Test comment content");
    assert_eq!(comment.thread_depth, 0);
    
    // Test find comment by ID
    let retrieved_comment = repository.find_comment_by_id(comment.id).await.unwrap();
    assert!(retrieved_comment.is_some());
    let retrieved_comment = retrieved_comment.unwrap();
    assert_eq!(retrieved_comment.id, comment.id);
    assert_eq!(retrieved_comment.content, comment.content);
    
    cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_create_and_find_like() {
    let pool = setup_test_db().await;
    let repository = SqliteSocialRepository::new(pool.clone());
    
    // Create a test post first
    let author_id = Uuid::new_v4();
    let post_data = CreatePostData {
        author_id,
        content: "Test post content".to_string(),
        visibility: Visibility::Public,
        cooperative_id: None,
        tags: vec![],
        mentions: vec![],
        reply_to_post_id: None,
        repost_of_post_id: None,
    };
    
    let post = repository.create_post(post_data).await.unwrap();
    
    // Create a test like
    let user_id = Uuid::new_v4();
    let like = Like::new_post_like(user_id, post.id);
    
    // Test create like
    let result = repository.create_like(&like).await;
    assert!(result.is_ok());
    
    // Test check user liked
    let is_liked = repository.check_user_liked(user_id, LikeTargetType::Post, post.id).await.unwrap();
    assert!(is_liked);
    
    // Test get post likes
    let likes = repository.get_post_likes(post.id).await.unwrap();
    assert_eq!(likes.len(), 1);
    assert_eq!(likes[0].id, like.id);
    
    // Test remove like
    let result = repository.remove_like(user_id, LikeTargetType::Post, post.id).await;
    assert!(result.is_ok());
    
    // Verify like is removed
    let is_liked = repository.check_user_liked(user_id, LikeTargetType::Post, post.id).await.unwrap();
    assert!(!is_liked);
    
    cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_create_and_find_share() {
    let pool = setup_test_db().await;
    let repository = SqliteSocialRepository::new(pool.clone());
    
    // Create a test post first
    let author_id = Uuid::new_v4();
    let post_data = CreatePostData {
        author_id,
        content: "Test post content".to_string(),
        visibility: Visibility::Public,
        cooperative_id: None,
        tags: vec![],
        mentions: vec![],
        reply_to_post_id: None,
        repost_of_post_id: None,
    };
    
    let post = repository.create_post(post_data).await.unwrap();
    
    // Create a test share
    let user_id = Uuid::new_v4();
    let share = Share::new_direct_share(user_id, post.id, Some("Check this out!".to_string()));
    
    // Test create share
    let result = repository.create_share(&share).await;
    assert!(result.is_ok());
    
    // Test get post shares
    let shares = repository.get_post_shares(post.id).await.unwrap();
    assert_eq!(shares.len(), 1);
    assert_eq!(shares[0].id, share.id);
    assert_eq!(shares[0].share_message, Some("Check this out!".to_string()));
    
    cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_create_and_find_repost() {
    let pool = setup_test_db().await;
    let repository = SqliteSocialRepository::new(pool.clone());
    
    // Create a test post first
    let author_id = Uuid::new_v4();
    let post_data = CreatePostData {
        author_id,
        content: "Test post content".to_string(),
        visibility: Visibility::Public,
        cooperative_id: None,
        tags: vec![],
        mentions: vec![],
        reply_to_post_id: None,
        repost_of_post_id: None,
    };
    
    let post = repository.create_post(post_data).await.unwrap();
    
    // Create a test repost
    let user_id = Uuid::new_v4();
    let repost = Repost::new(user_id, post.id, Some("Great post!".to_string()));
    
    // Test create repost
    let result = repository.create_repost(&repost).await;
    assert!(result.is_ok());
    
    // Test get post reposts
    let reposts = repository.get_post_reposts(post.id).await.unwrap();
    assert_eq!(reposts.len(), 1);
    assert_eq!(reposts[0].id, repost.id);
    assert_eq!(reposts[0].repost_message, Some("Great post!".to_string()));
    
    cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_create_and_find_follow() {
    let pool = setup_test_db().await;
    let repository = SqliteSocialRepository::new(pool.clone());
    
    // Create a test follow
    let follower_id = Uuid::new_v4();
    let followed_id = Uuid::new_v4();
    let follow = Follow::new(follower_id, followed_id);
    
    // Test create follow
    let result = repository.create_follow(&follow).await;
    assert!(result.is_ok());
    
    // Test check is following
    let is_following = repository.check_is_following(follower_id, followed_id).await.unwrap();
    assert!(is_following);
    
    // Test get user followers
    let followers = repository.get_user_followers(followed_id).await.unwrap();
    assert_eq!(followers.len(), 1);
    assert_eq!(followers[0].id, follow.id);
    
    // Test get user following
    let following = repository.get_user_following(follower_id).await.unwrap();
    assert_eq!(following.len(), 1);
    assert_eq!(following[0].id, follow.id);
    
    // Test remove follow
    let result = repository.remove_follow(follower_id, followed_id).await;
    assert!(result.is_ok());
    
    // Verify follow is removed
    let is_following = repository.check_is_following(follower_id, followed_id).await.unwrap();
    assert!(!is_following);
    
    cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_create_and_find_block() {
    let pool = setup_test_db().await;
    let repository = SqliteSocialRepository::new(pool.clone());
    
    // Create a test block
    let blocker_id = Uuid::new_v4();
    let blocked_id = Uuid::new_v4();
    let block = Block::new(blocker_id, blocked_id, Some("Spam".to_string()));
    
    // Test create block
    let result = repository.create_block(&block).await;
    assert!(result.is_ok());
    
    // Test check is blocked
    let is_blocked = repository.check_is_blocked(blocker_id, blocked_id).await.unwrap();
    assert!(is_blocked);
    
    // Test get user blocks
    let blocks = repository.get_user_blocks(blocker_id).await.unwrap();
    assert_eq!(blocks.len(), 1);
    assert_eq!(blocks[0].id, block.id);
    
    // Test remove block
    let result = repository.remove_block(blocker_id, blocked_id).await;
    assert!(result.is_ok());
    
    // Verify block is removed
    let is_blocked = repository.check_is_blocked(blocker_id, blocked_id).await.unwrap();
    assert!(!is_blocked);
    
    cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_create_and_find_mute() {
    let pool = setup_test_db().await;
    let repository = SqliteSocialRepository::new(pool.clone());
    
    // Create a test mute
    let muter_id = Uuid::new_v4();
    let muted_id = Uuid::new_v4();
    let mute = Mute::new(muter_id, muted_id);
    
    // Test create mute
    let result = repository.create_mute(&mute).await;
    assert!(result.is_ok());
    
    // Test check is muted
    let is_muted = repository.check_is_muted(muter_id, muted_id).await.unwrap();
    assert!(is_muted);
    
    // Test get user mutes
    let mutes = repository.get_user_mutes(muter_id).await.unwrap();
    assert_eq!(mutes.len(), 1);
    assert_eq!(mutes[0].id, mute.id);
    
    // Test remove mute
    let result = repository.remove_mute(muter_id, muted_id).await;
    assert!(result.is_ok());
    
    // Verify mute is removed
    let is_muted = repository.check_is_muted(muter_id, muted_id).await.unwrap();
    assert!(!is_muted);
    
    cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_record_and_get_user_activity() {
    let pool = setup_test_db().await;
    let repository = SqliteSocialRepository::new(pool.clone());
    
    // Create a test activity
    let user_id = Uuid::new_v4();
    let activity = UserActivity::new(user_id, ActivityType::PostCreated, Some("POST".to_string()), Some(Uuid::new_v4()));
    
    // Test record activity
    let result = repository.record_user_activity(&activity).await;
    assert!(result.is_ok());
    
    // Test get user activity
    let activities = repository.get_user_activity(user_id, 10, 0).await.unwrap();
    assert_eq!(activities.len(), 1);
    assert_eq!(activities[0].id, activity.id);
    assert_eq!(activities[0].user_id, user_id);
    assert_eq!(activities[0].activity_type, ActivityType::PostCreated);
    
    cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_get_user_posts() {
    let pool = setup_test_db().await;
    let repository = SqliteSocialRepository::new(pool.clone());
    
    // Create a test user
    let user_id = Uuid::new_v4();
    
    // Create test posts
    for i in 1..=3 {
        let post_data = CreatePostData {
            author_id: user_id,
            content: format!("Test post {}", i),
            visibility: Visibility::Public,
            cooperative_id: None,
            tags: vec![],
            mentions: vec![],
            reply_to_post_id: None,
            repost_of_post_id: None,
        };
        
        repository.create_post(post_data).await.unwrap();
    }
    
    // Test get user posts
    let posts = repository.get_user_posts(user_id, 10, 0).await.unwrap();
    assert_eq!(posts.len(), 3);
    
    // Verify posts are ordered by created_at descending
    for i in 0..posts.len()-1 {
        assert!(posts[i].created_at >= posts[i+1].created_at);
    }
    
    cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_get_post_comments() {
    let pool = setup_test_db().await;
    let repository = SqliteSocialRepository::new(pool.clone());
    
    // Create a test post
    let author_id = Uuid::new_v4();
    let post_data = CreatePostData {
        author_id,
        content: "Test post content".to_string(),
        visibility: Visibility::Public,
        cooperative_id: None,
        tags: vec![],
        mentions: vec![],
        reply_to_post_id: None,
        repost_of_post_id: None,
    };
    
    let post = repository.create_post(post_data).await.unwrap();
    
    // Create test comments
    for i in 1..=3 {
        let commenter_id = Uuid::new_v4();
        let comment_data = CreateCommentData {
            post_id: post.id,
            author_id: commenter_id,
            content: format!("Test comment {}", i),
            parent_comment_id: None,
            mentions: vec![],
        };
        
        repository.create_comment(comment_data).await.unwrap();
    }
    
    // Test get post comments
    let comments = repository.get_post_comments(post.id, 10, 0).await.unwrap();
    assert_eq!(comments.len(), 3);
    
    // Verify comments are ordered by created_at ascending
    for i in 0..comments.len()-1 {
        assert!(comments[i].created_at <= comments[i+1].created_at);
    }
    
    cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_generate_user_feed() {
    let pool = setup_test_db().await;
    let repository = SqliteSocialRepository::new(pool.clone());
    
    // Create test users
    let user_id = Uuid::new_v4();
    let followed_user_id = Uuid::new_v4();
    
    // Create a follow relationship
    let follow = Follow::new(user_id, followed_user_id);
    repository.create_follow(&follow).await.unwrap();
    
    // Create a post by the followed user
    let post_data = CreatePostData {
        author_id: followed_user_id,
        content: "Test post in feed".to_string(),
        visibility: Visibility::Public,
        cooperative_id: None,
        tags: vec![],
        mentions: vec![],
        reply_to_post_id: None,
        repost_of_post_id: None,
    };
    
    repository.create_post(post_data).await.unwrap();
    
    // Test generate user feed
    let feed_posts = repository.generate_user_feed(user_id, FeedType::Home, 10, 0).await.unwrap();
    assert_eq!(feed_posts.len(), 1);
    assert_eq!(feed_posts[0].content, "Test post in feed");
    
    cleanup_test_data(&pool).await;
}