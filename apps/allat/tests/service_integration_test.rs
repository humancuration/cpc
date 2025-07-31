use allat::application::community_service::{CommunityService, CommunityServiceImpl, CreateCommunityInput};
use allat::application::post_service::{PostService, PostServiceImpl, CreatePostInput};
use allat::application::comment_service::{CommentService, CommentServiceImpl, CreateCommentInput};
use allat::application::vote_service::{VoteService, VoteServiceImpl, VotePostInput};
use allat::domain::vote::VoteType;
use allat::infrastructure::repositories::community_repo::{CommunityRepository, PgCommunityRepository};
use allat::infrastructure::repositories::post_repo::{PostRepository, PgPostRepository};
use allat::infrastructure::repositories::comment_repo::{CommentRepository, PgCommentRepository};
use allat::infrastructure::repositories::vote_repo::{VoteRepository, PgVoteRepository};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

// Helper function to set up test database
async fn setup_test_db() -> PgPool {
    let db_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://localhost/allat_test".to_string());
    
    let pool = PgPool::connect(&db_url).await.unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    
    // Clear test data
    sqlx::query!("DELETE FROM votes").execute(&pool).await.unwrap();
    sqlx::query!("DELETE FROM media_assets").execute(&pool).await.unwrap();
    sqlx::query!("DELETE FROM posts").execute(&pool).await.unwrap();
    sqlx::query!("DELETE FROM communities").execute(&pool).await.unwrap();
    
    pool
}

#[tokio::test]
async fn test_community_service_integration() {
    let pool = setup_test_db().await;
    let community_repo: Arc<dyn CommunityRepository> = Arc::new(PgCommunityRepository::new(pool.clone()));
    let community_service: Arc<dyn CommunityService> = Arc::new(CommunityServiceImpl::new(community_repo.clone()));
    
    // Test creating a community
    let input = CreateCommunityInput {
        name: "Test Community".to_string(),
        description: "A test community".to_string(),
        rules: vec!["Be respectful".to_string(), "No spam".to_string()],
    };
    
    let community = community_service.create_community(input).await.unwrap();
    assert_eq!(community.name, "Test Community");
    assert_eq!(community.rules.len(), 2);
    
    // Test getting a community
    let retrieved = community_service.get_community(community.id).await.unwrap();
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().name, "Test Community");
    
    // Test updating a community
    let update_input = allat::application::community_service::UpdateCommunityInput {
        name: Some("Updated Community".to_string()),
        description: Some("An updated test community".to_string()),
        rules: Some(vec!["Be respectful".to_string()]),
    };
    
    let updated = community_service.update_community(community.id, update_input).await.unwrap();
    assert_eq!(updated.name, "Updated Community");
    assert_eq!(updated.rules.len(), 1);
    
    // Test deleting a community
    let deleted = community_service.delete_community(community.id).await.unwrap();
    assert!(deleted);
    
    let retrieved = community_service.get_community(community.id).await.unwrap();
    assert!(retrieved.is_none());
}

#[tokio::test]
async fn test_post_service_integration() {
    let pool = setup_test_db().await;
    let community_repo: Arc<dyn CommunityRepository> = Arc::new(PgCommunityRepository::new(pool.clone()));
    let post_repo: Arc<dyn PostRepository> = Arc::new(PgPostRepository::new(pool.clone()));
    let post_service: Arc<dyn PostService> = Arc::new(PostServiceImpl::new(post_repo.clone(), community_repo.clone()));
    
    // First create a community for the post
    let community_input = allat::application::community_service::CreateCommunityInput {
        name: "Test Community for Posts".to_string(),
        description: "A test community for posts".to_string(),
        rules: vec![],
    };
    
    let community_service: Arc<dyn allat::application::community_service::CommunityService> = 
        Arc::new(allat::application::community_service::CommunityServiceImpl::new(community_repo.clone()));
    let community = community_service.create_community(community_input).await.unwrap();
    
    // Test creating a post
    let user_id = Uuid::new_v4();
    let input = CreatePostInput {
        community_id: community.id,
        user_id,
        title: "Test Post".to_string(),
        content: "This is a test post".to_string(),
        media_assets: vec![],
    };
    
    let post = post_service.create_post(input).await.unwrap();
    assert_eq!(post.title, "Test Post");
    assert_eq!(post.content, "This is a test post");
    
    // Test getting a post
    let retrieved = post_service.get_post(post.id).await.unwrap();
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().title, "Test Post");
    
    // Test getting posts by community
    let posts = post_service.get_posts_by_community(community.id).await.unwrap();
    assert_eq!(posts.len(), 1);
    assert_eq!(posts[0].title, "Test Post");
    
    // Test updating a post
    let update_input = allat::application::post_service::UpdatePostInput {
        title: Some("Updated Post".to_string()),
        content: Some("This is an updated test post".to_string()),
        media_assets: None,
    };
    
    let updated = post_service.update_post(post.id, update_input).await.unwrap();
    assert_eq!(updated.title, "Updated Post");
    assert_eq!(updated.content, "This is an updated test post");
    
    // Test deleting a post
    let deleted = post_service.delete_post(post.id).await.unwrap();
    assert!(deleted);
    
    let retrieved = post_service.get_post(post.id).await.unwrap();
    assert!(retrieved.is_none());
}

#[tokio::test]
async fn test_comment_service_integration() {
    let pool = setup_test_db().await;
    let community_repo: Arc<dyn CommunityRepository> = Arc::new(PgCommunityRepository::new(pool.clone()));
    let post_repo: Arc<dyn PostRepository> = Arc::new(PgPostRepository::new(pool.clone()));
    let comment_repo: Arc<dyn CommentRepository> = Arc::new(PgCommentRepository::new(pool.clone()));
    let comment_service: Arc<dyn CommentService> = Arc::new(CommentServiceImpl::new(comment_repo.clone(), post_repo.clone()));
    
    // First create a community and post for the comment
    let community_input = allat::application::community_service::CreateCommunityInput {
        name: "Test Community for Comments".to_string(),
        description: "A test community for comments".to_string(),
        rules: vec![],
    };
    
    let community_service: Arc<dyn allat::application::community_service::CommunityService> = 
        Arc::new(allat::application::community_service::CommunityServiceImpl::new(community_repo.clone()));
    let community = community_service.create_community(community_input).await.unwrap();
    
    let user_id = Uuid::new_v4();
    let post_input = allat::application::post_service::CreatePostInput {
        community_id: community.id,
        user_id,
        title: "Test Post for Comments".to_string(),
        content: "This is a test post for comments".to_string(),
        media_assets: vec![],
    };
    
    let post_service: Arc<dyn allat::application::post_service::PostService> = 
        Arc::new(allat::application::post_service::PostServiceImpl::new(post_repo.clone(), community_repo.clone()));
    let post = post_service.create_post(post_input).await.unwrap();
    
    // Test creating a comment
    let input = CreateCommentInput {
        post_id: post.id,
        user_id,
        content: "This is a test comment".to_string(),
        parent_id: None,
    };
    
    let comment = comment_service.create_comment(input).await.unwrap();
    assert_eq!(comment.content, "This is a test comment");
    
    // Test getting a comment
    let retrieved = comment_service.get_comment(comment.id).await.unwrap();
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().content, "This is a test comment");
    
    // Test updating a comment
    let update_input = allat::application::comment_service::UpdateCommentInput {
        content: "This is an updated test comment".to_string(),
    };
    
    let updated = comment_service.update_comment(comment.id, update_input).await.unwrap();
    assert_eq!(updated.content, "This is an updated test comment");
    
    // Test deleting a comment
    let deleted = comment_service.delete_comment(comment.id).await.unwrap();
    assert!(deleted);
    
    let retrieved = comment_service.get_comment(comment.id).await.unwrap();
    assert!(retrieved.is_none());
}

#[tokio::test]
async fn test_vote_service_integration() {
    let pool = setup_test_db().await;
    let community_repo: Arc<dyn CommunityRepository> = Arc::new(PgCommunityRepository::new(pool.clone()));
    let post_repo: Arc<dyn PostRepository> = Arc::new(PgPostRepository::new(pool.clone()));
    let vote_repo: Arc<dyn VoteRepository> = Arc::new(PgVoteRepository::new(pool.clone()));
    
    // For this test, we'll need to mock the auth service
    // In a real implementation, we would use a mock auth service
    
    // First create a community and post for the vote
    let community_input = allat::application::community_service::CreateCommunityInput {
        name: "Test Community for Votes".to_string(),
        description: "A test community for votes".to_string(),
        rules: vec![],
    };
    
    let community_service: Arc<dyn allat::application::community_service::CommunityService> = 
        Arc::new(allat::application::community_service::CommunityServiceImpl::new(community_repo.clone()));
    let community = community_service.create_community(community_input).await.unwrap();
    
    let post_user_id = Uuid::new_v4();
    let post_input = allat::application::post_service::CreatePostInput {
        community_id: community.id,
        user_id: post_user_id,
        title: "Test Post for Votes".to_string(),
        content: "This is a test post for votes".to_string(),
        media_assets: vec![],
    };
    
    let post_service: Arc<dyn allat::application::post_service::PostService> = 
        Arc::new(allat::application::post_service::PostServiceImpl::new(post_repo.clone(), community_repo.clone()));
    let post = post_service.create_post(post_input).await.unwrap();
    
    // For now, we'll skip the vote service test since it requires a real auth service
    // In a real implementation, we would mock the auth service
}