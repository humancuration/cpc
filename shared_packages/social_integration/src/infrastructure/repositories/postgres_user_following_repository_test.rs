//! Tests for the PostgreSQL user following repository

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::repositories::PostgresUserFollowingRepository;
    use sqlx::PgPool;
    use uuid::Uuid;
    
    #[sqlx::test]
    async fn test_follow_and_get_following(pool: PgPool) -> sqlx::Result<()> {
        let repo = PostgresUserFollowingRepository::new(pool);
        let follower_id = Uuid::new_v4();
        let followed_id = Uuid::new_v4();
        
        // Follow a user
        repo.follow(follower_id, followed_id).await.unwrap();
        
        // Get following list
        let following = repo.get_following(follower_id).await.unwrap();
        assert_eq!(following.len(), 1);
        assert_eq!(following[0], followed_id);
        
        Ok(())
    }
    
    #[sqlx::test]
    async fn test_unfollow(pool: PgPool) -> sqlx::Result<()> {
        let repo = PostgresUserFollowingRepository::new(pool);
        let follower_id = Uuid::new_v4();
        let followed_id = Uuid::new_v4();
        
        // Follow a user
        repo.follow(follower_id, followed_id).await.unwrap();
        
        // Verify user is being followed
        let following = repo.get_following(follower_id).await.unwrap();
        assert_eq!(following.len(), 1);
        assert_eq!(following[0], followed_id);
        
        // Unfollow the user
        repo.unfollow(follower_id, followed_id).await.unwrap();
        
        // Verify user is no longer being followed
        let following = repo.get_following(follower_id).await.unwrap();
        assert!(following.is_empty());
        
        Ok(())
    }
    
    #[sqlx::test]
    async fn test_follow_multiple_users(pool: PgPool) -> sqlx::Result<()> {
        let repo = PostgresUserFollowingRepository::new(pool);
        let follower_id = Uuid::new_v4();
        let followed_id1 = Uuid::new_v4();
        let followed_id2 = Uuid::new_v4();
        let followed_id3 = Uuid::new_v4();
        
        // Follow multiple users
        repo.follow(follower_id, followed_id1).await.unwrap();
        repo.follow(follower_id, followed_id2).await.unwrap();
        repo.follow(follower_id, followed_id3).await.unwrap();
        
        // Get following list
        let following = repo.get_following(follower_id).await.unwrap();
        assert_eq!(following.len(), 3);
        
        // Verify all followed users are in the list
        assert!(following.contains(&followed_id1));
        assert!(following.contains(&followed_id2));
        assert!(following.contains(&followed_id3));
        
        Ok(())
    }
    
    #[sqlx::test]
    async fn test_unfollow_one_of_multiple(pool: PgPool) -> sqlx::Result<()> {
        let repo = PostgresUserFollowingRepository::new(pool);
        let follower_id = Uuid::new_v4();
        let followed_id1 = Uuid::new_v4();
        let followed_id2 = Uuid::new_v4();
        let followed_id3 = Uuid::new_v4();
        
        // Follow multiple users
        repo.follow(follower_id, followed_id1).await.unwrap();
        repo.follow(follower_id, followed_id2).await.unwrap();
        repo.follow(follower_id, followed_id3).await.unwrap();
        
        // Unfollow one user
        repo.unfollow(follower_id, followed_id2).await.unwrap();
        
        // Get following list
        let following = repo.get_following(follower_id).await.unwrap();
        assert_eq!(following.len(), 2);
        
        // Verify the unfollowed user is not in the list
        assert!(following.contains(&followed_id1));
        assert!(!following.contains(&followed_id2));
        assert!(following.contains(&followed_id3));
        
        Ok(())
    }
    
    #[sqlx::test]
    async fn test_get_following_returns_empty_for_nonexistent_user(pool: PgPool) -> sqlx::Result<()> {
        let repo = PostgresUserFollowingRepository::new(pool);
        let nonexistent_user_id = Uuid::new_v4();
        
        // Get following list for a user that doesn't follow anyone
        let following = repo.get_following(nonexistent_user_id).await.unwrap();
        assert!(following.is_empty());
        
        Ok(())
    }
    
    #[sqlx::test]
    async fn test_follow_same_user_twice_is_idempotent(pool: PgPool) -> sqlx::Result<()> {
        let repo = PostgresUserFollowingRepository::new(pool);
        let follower_id = Uuid::new_v4();
        let followed_id = Uuid::new_v4();
        
        // Follow a user twice
        repo.follow(follower_id, followed_id).await.unwrap();
        repo.follow(follower_id, followed_id).await.unwrap();
        
        // Get following list - should only contain the user once
        let following = repo.get_following(follower_id).await.unwrap();
        assert_eq!(following.len(), 1);
        assert_eq!(following[0], followed_id);
        
        Ok(())
    }
    
    #[sqlx::test]
    async fn test_unfollow_nonexistent_relationship(pool: PgPool) -> sqlx::Result<()> {
        let repo = PostgresUserFollowingRepository::new(pool);
        let follower_id = Uuid::new_v4();
        let followed_id = Uuid::new_v4();
        
        // Unfollow a user that isn't being followed (should not error)
        let result = repo.unfollow(follower_id, followed_id).await;
        assert!(result.is_ok());
        
        Ok(())
    }
    
    #[sqlx::test]
    async fn test_following_order_is_preserved(pool: PgPool) -> sqlx::Result<()> {
        let repo = PostgresUserFollowingRepository::new(pool);
        let follower_id = Uuid::new_v4();
        let followed_id1 = Uuid::new_v4();
        let followed_id2 = Uuid::new_v4();
        let followed_id3 = Uuid::new_v4();
        
        // Follow users in a specific order
        repo.follow(follower_id, followed_id1).await.unwrap();
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await; // Ensure different timestamps
        repo.follow(follower_id, followed_id2).await.unwrap();
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await; // Ensure different timestamps
        repo.follow(follower_id, followed_id3).await.unwrap();
        
        // Get following list - should be in the order they were followed (descending by creation time)
        let following = repo.get_following(follower_id).await.unwrap();
        assert_eq!(following.len(), 3);
        // Most recent first
        assert_eq!(following[0], followed_id3);
        assert_eq!(following[1], followed_id2);
        assert_eq!(following[2], followed_id1);
        
        Ok(())
    }
    
    #[sqlx::test]
    async fn test_circular_following(pool: PgPool) -> sqlx::Result<()> {
        let repo = PostgresUserFollowingRepository::new(pool);
        let user1_id = Uuid::new_v4();
        let user2_id = Uuid::new_v4();
        
        // User 1 follows User 2
        repo.follow(user1_id, user2_id).await.unwrap();
        
        // User 2 follows User 1
        repo.follow(user2_id, user1_id).await.unwrap();
        
        // Check User 1's following
        let user1_following = repo.get_following(user1_id).await.unwrap();
        assert_eq!(user1_following.len(), 1);
        assert_eq!(user1_following[0], user2_id);
        
        // Check User 2's following
        let user2_following = repo.get_following(user2_id).await.unwrap();
        assert_eq!(user2_following.len(), 1);
        assert_eq!(user2_following[0], user1_id);
        
        Ok(())
    }
}