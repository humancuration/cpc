#[cfg(test)]
mod tests {
    use super::*;
    use test_helpers::setup_test_db;
    use serial_test::serial;
    
    // Import repositories and domain models
    use allat::infrastructure::repositories::{
        community_repo::{PgCommunityRepository, CommunityRepository},
        post_repo::{PgPostRepository, PostRepository},
        comment_repo::{PgCommentRepository, CommentRepository},
        user_repo::{PgUserRepository, UserRepository}
    };
    use allat::domain::community::Community;
    use allat::domain::post::Post;
    use allat::domain::comment::Comment;
    use allat::domain::auth::user::User;
    use allat::domain::media_asset::{MediaAsset, MediaType};
    use allat::domain::vote::{Vote, VoteType};
    use uuid::Uuid;
    use chrono::Utc;
    use cpc_auth::models::User as BaseUser;

    #[tokio::test]
    #[serial]
    async fn test_community_repository() {
        let pool = setup_test_db().await;
        let repo = PgCommunityRepository::new(pool);
        
        // Create community
        let community = Community {
            id: Uuid::new_v4(),
            name: "Test Community".to_string(),
            description: "Test Description".to_string(),
            rules: vec!["Rule 1".to_string(), "Rule 2".to_string()],
            created_at: Utc::now(),
        };
        
        // Test create
        repo.create(&community).await.expect("Failed to create community");
        
        // Test find_by_id
        let found = repo.find_by_id(community.id).await.expect("Failed to find community");
        assert!(found.is_some());
        let found = found.unwrap();
        assert_eq!(found.name, community.name);
        
        // Test find_by_name
        let found = repo.find_by_name(&community.name).await.expect("Failed to find by name");
        assert!(found.is_some());
        let found = found.unwrap();
        assert_eq!(found.id, community.id);
        
        // Test update
        let mut updated = community.clone();
        updated.name = "Updated Community".to_string();
        repo.update(&updated).await.expect("Failed to update community");
        
        let found = repo.find_by_id(updated.id).await.expect("Failed to find updated community");
        assert!(found.is_some());
        let found = found.unwrap();
        assert_eq!(found.name, "Updated Community");
        
        // Test delete
        repo.delete(updated.id).await.expect("Failed to delete community");
        let found = repo.find_by_id(updated.id).await.expect("Failed to find community");
        assert!(found.is_none());
    }

    #[tokio::test]
    #[serial]
    async fn test_post_repository() {
        let pool = setup_test_db().await;
        let community_repo = PgCommunityRepository::new(pool.clone());
        let user_repo = PgUserRepository::new(pool.clone());
        let post_repo = PgPostRepository::new(pool.clone());
        
        // Create community
        let community = Community {
            id: Uuid::new_v4(),
            name: "Test Community".to_string(),
            description: "Test Description".to_string(),
            rules: vec!["Rule 1".to_string()],
            created_at: Utc::now(),
        };
        community_repo.create(&community).await.expect("Failed to create community");
        
        // Create user
        // Note: We're not actually saving the user to the database in this test
        // since the user repository implementation is minimal
        
        // Create post
        let post = Post::new(
            community.id,
            Uuid::new_v4(), // user_id
            "Test Post".to_string(),
            "Test Content".to_string(),
            None, // parent_id
            vec![MediaAsset::new(
                "http://example.com/image.jpg".to_string(),
                MediaType::Image,
                Some("Test image".to_string()),
            )],
        );
        
        // Test create
        post_repo.create(&post).await.expect("Failed to create post");
        
        // Test find_by_id
        let found = post_repo.find_by_id(post.id).await.expect("Failed to find post");
        assert!(found.is_some());
        let found = found.unwrap();
        assert_eq!(found.title, post.title);
        assert_eq!(found.content, post.content);
        assert_eq!(found.media_assets.len(), 1);
        assert_eq!(found.media_assets[0].url, "http://example.com/image.jpg");
        
        // Test find_by_community
        let posts = post_repo.find_by_community(community.id).await.expect("Failed to find posts by community");
        assert_eq!(posts.len(), 1);
        assert_eq!(posts[0].id, post.id);
        
        // Test update
        let mut updated = post.clone();
        updated.title = "Updated Post".to_string();
        updated.content = "Updated Content".to_string();
        updated.media_assets = vec![MediaAsset::new(
            "http://example.com/video.mp4".to_string(),
            MediaType::Video,
            Some("Test video".to_string()),
        ))];
        post_repo.update(&updated).await.expect("Failed to update post");
        
        let found = post_repo.find_by_id(updated.id).await.expect("Failed to find updated post");
        assert!(found.is_some());
        let found = found.unwrap();
        assert_eq!(found.title, "Updated Post");
        assert_eq!(found.content, "Updated Content");
        assert_eq!(found.media_assets.len(), 1);
        assert_eq!(found.media_assets[0].url, "http://example.com/video.mp4");
        
        // Test delete
        post_repo.delete(updated.id).await.expect("Failed to delete post");
        let found = post_repo.find_by_id(updated.id).await.expect("Failed to find post");
        assert!(found.is_none());
    }

    #[tokio::test]
    #[serial]
    async fn test_comment_repository() {
        let pool = setup_test_db().await;
        let community_repo = PgCommunityRepository::new(pool.clone());
        let post_repo = PgPostRepository::new(pool.clone());
        let comment_repo = PgCommentRepository::new(pool.clone());
        
        // Create community
        let community = Community {
            id: Uuid::new_v4(),
            name: "Test Community".to_string(),
            description: "Test Description".to_string(),
            rules: vec!["Rule 1".to_string()],
            created_at: Utc::now(),
        };
        community_repo.create(&community).await.expect("Failed to create community");
        
        // Create post
        let post = Post::new(
            community.id,
            Uuid::new_v4(), // user_id
            "Test Post".to_string(),
            "Test Content".to_string(),
            None, // parent_id
            vec![],
        );
        post_repo.create(&post).await.expect("Failed to create post");
        
        // Create comment
        let comment = Comment::new(
            post.id,
            Uuid::new_v4(), // user_id
            "Test Comment".to_string(),
            None, // parent_id
        );
        
        // Test create
        comment_repo.create(&comment).await.expect("Failed to create comment");
        
        // Test find_by_id
        let found = comment_repo.find_by_id(comment.id).await.expect("Failed to find comment");
        assert!(found.is_some());
        let found = found.unwrap();
        assert_eq!(found.content, comment.content);
        
        // Test find_by_post
        let comments = comment_repo.find_by_post(post.id).await.expect("Failed to find comments by post");
        assert_eq!(comments.len(), 1);
        assert_eq!(comments[0].id, comment.id);
        
        // Test find_replies (for nested comments)
        let reply = Comment::new(
            post.id,
            Uuid::new_v4(), // user_id
            "Test Reply".to_string(),
            Some(comment.id), // parent_id
        );
        comment_repo.create(&reply).await.expect("Failed to create reply");
        
        let replies = comment_repo.find_replies(comment.id).await.expect("Failed to find replies");
        assert_eq!(replies.len(), 1);
        assert_eq!(replies[0].id, reply.id);
        
        // Test update
        let mut updated = comment.clone();
        updated.content = "Updated Comment".to_string();
        comment_repo.update(&updated).await.expect("Failed to update comment");
        
        let found = comment_repo.find_by_id(updated.id).await.expect("Failed to find updated comment");
        assert!(found.is_some());
        let found = found.unwrap();
        assert_eq!(found.content, "Updated Comment");
        
        // Test delete
        comment_repo.delete(updated.id).await.expect("Failed to delete comment");
        let found = comment_repo.find_by_id(updated.id).await.expect("Failed to find comment");
        assert!(found.is_none());
    }

    #[tokio::test]
    #[serial]
    async fn test_user_repository() {
        let pool = setup_test_db().await;
        let user_repo = PgUserRepository::new(pool);
        
        // Test update_karma with valid delta
        let user_id = Uuid::new_v4();
        
        // First, insert a user with 0 karma (since our test DB is fresh)
        sqlx::query!(
            "INSERT INTO users (id, username, email, password_hash, karma) VALUES ($1, $2, $3, $4, $5)",
            user_id,
            "testuser",
            "test@example.com",
            "hash",
            0
        )
        .execute(&pool)
        .await
        .expect("Failed to insert test user");
        
        // Test update_karma with positive delta
        user_repo.update_karma(user_id, 10).await.expect("Failed to update karma");
        
        let new_karma: i32 = sqlx::query_scalar!("SELECT karma FROM users WHERE id = $1", user_id)
            .fetch_one(&pool)
            .await
            .expect("Failed to fetch user karma");
        assert_eq!(new_karma, 10);
        
        // Test update_karma with negative delta
        user_repo.update_karma(user_id, -5).await.expect("Failed to update karma");
        
        let new_karma: i32 = sqlx::query_scalar!("SELECT karma FROM users WHERE id = $1", user_id)
            .fetch_one(&pool)
            .await
            .expect("Failed to fetch user karma");
        assert_eq!(new_karma, 5);
        
        // Test karma limit exceeded
        let result = user_repo.update_karma(user_id, 10001).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    #[serial]
    async fn test_post_community_relationship() {
        let pool = setup_test_db().await;
        let community_repo = PgCommunityRepository::new(pool.clone());
        let post_repo = PgPostRepository::new(pool.clone());
        
        // Create community
        let community = Community {
            id: Uuid::new_v4(),
            name: "Test Community".to_string(),
            description: "Test Description".to_string(),
            rules: vec!["Rule 1".to_string()],
            created_at: Utc::now(),
        };
        community_repo.create(&community).await.unwrap();
        
        // Create post in community
        let post = Post::new(
            community.id,
            Uuid::new_v4(), // user_id
            "Test Post".to_string(),
            "Test Content".to_string(),
            None, // parent_id
            vec![],
        );
        post_repo.create(&post).await.unwrap();
        
        // Test relationship
        let posts = post_repo.find_by_community(community.id).await.unwrap();
        assert_eq!(posts.len(), 1);
        assert_eq!(posts[0].id, post.id);
    }

    #[tokio::test]
    #[serial]
    async fn test_comment_post_relationship() {
        let pool = setup_test_db().await;
        let community_repo = PgCommunityRepository::new(pool.clone());
        let post_repo = PgPostRepository::new(pool.clone());
        let comment_repo = PgCommentRepository::new(pool.clone());
        
        // Create community
        let community = Community {
            id: Uuid::new_v4(),
            name: "Test Community".to_string(),
            description: "Test Description".to_string(),
            rules: vec!["Rule 1".to_string()],
            created_at: Utc::now(),
        };
        community_repo.create(&community).await.unwrap();
        
        // Create post
        let post = Post::new(
            community.id,
            Uuid::new_v4(), // user_id
            "Test Post".to_string(),
            "Test Content".to_string(),
            None, // parent_id
            vec![],
        );
        post_repo.create(&post).await.unwrap();
        
        // Create comment on post
        let comment = Comment::new(
            post.id,
            Uuid::new_v4(), // user_id
            "Test Comment".to_string(),
            None, // parent_id
        );
        comment_repo.create(&comment).await.unwrap();
        
        // Test relationship
        let comments = comment_repo.find_by_post(post.id).await.unwrap();
        assert_eq!(comments.len(), 1);
        assert_eq!(comments[0].id, comment.id);
    }

    #[tokio::test]
    #[serial]
    async fn test_not_found_scenarios() {
        let pool = setup_test_db().await;
        let community_repo = PgCommunityRepository::new(pool.clone());
        let post_repo = PgPostRepository::new(pool.clone());
        let comment_repo = PgCommentRepository::new(pool.clone());
        
        // Test repositories with invalid IDs
        let invalid_id = Uuid::new_v4();
        
        let found = community_repo.find_by_id(invalid_id).await.unwrap();
        assert!(found.is_none());
        
        let found = post_repo.find_by_id(invalid_id).await.unwrap();
        assert!(found.is_none());
        
        let found = comment_repo.find_by_id(invalid_id).await.unwrap();
        assert!(found.is_none());
    }

    #[tokio::test]
    #[serial]
    async fn test_vote_counting() {
        let pool = setup_test_db().await;
        let community_repo = PgCommunityRepository::new(pool.clone());
        let post_repo = PgPostRepository::new(pool.clone());
        
        // Create community
        let community = Community {
            id: Uuid::new_v4(),
            name: "Test Community".to_string(),
            description: "Test Description".to_string(),
            rules: vec!["Rule 1".to_string()],
            created_at: Utc::now(),
        };
        community_repo.create(&community).await.unwrap();
        
        // Create post
        let post = Post::new(
            community.id,
            Uuid::new_v4(), // user_id
            "Test Post".to_string(),
            "Test Content".to_string(),
            None, // parent_id
            vec![],
        );
        post_repo.create(&post).await.unwrap();
        
        // Add some votes to the database directly
        let user1_id = Uuid::new_v4();
        let user2_id = Uuid::new_v4();
        let user3_id = Uuid::new_v4();
        
        // Insert users
        for user_id in &[user1_id, user2_id, user3_id] {
            sqlx::query!(
                "INSERT INTO users (id, username, email, password_hash, karma) VALUES ($1, $2, $3, $4, $5)",
                user_id,
                format!("user_{}", user_id),
                format!("user_{}@example.com", user_id),
                "hash",
                0
            )
            .execute(&pool)
            .await
            .expect("Failed to insert test user");
        }
        
        // Insert votes
        // User 1 upvotes
        sqlx::query!(
            "INSERT INTO votes (id, user_id, post_id, vote_type) VALUES ($1, $2, $3, $4)",
            Uuid::new_v4(),
            user1_id,
            post.id,
            "Upvote"
        )
        .execute(&pool)
        .await
        .expect("Failed to insert upvote");
        
        // User 2 upvotes
        sqlx::query!(
            "INSERT INTO votes (id, user_id, post_id, vote_type) VALUES ($1, $2, $3, $4)",
            Uuid::new_v4(),
            user2_id,
            post.id,
            "Upvote"
        )
        .execute(&pool)
        .await
        .expect("Failed to insert upvote");
        
        // User 3 downvotes
        sqlx::query!(
            "INSERT INTO votes (id, user_id, post_id, vote_type) VALUES ($1, $2, $3, $4)",
            Uuid::new_v4(),
            user3_id,
            post.id,
            "Downvote"
        )
        .execute(&pool)
        .await
        .expect("Failed to insert downvote");
        
        // Test vote counting (2 upvotes - 1 downvote = 1)
        let vote_count = post_repo.get_vote_count(post.id).await.unwrap();
        assert_eq!(vote_count, 1);
    }
}