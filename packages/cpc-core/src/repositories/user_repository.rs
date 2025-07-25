use crate::models::user::User;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use sqlx::{SqlitePool, Row};
use serde_json;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: &mut User) -> Result<()>;
    async fn find_by_id(&self, user_id: Uuid) -> Result<Option<User>>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>>;
    async fn update(&self, user: &User) -> Result<()>;
    async fn delete(&self, user_id: Uuid) -> Result<()>;
}

/// SQLite implementation of UserRepository using SQLx
pub struct SqliteUserRepository {
    pool: SqlitePool,
}

impl SqliteUserRepository {
    /// Creates a new SqliteUserRepository instance
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for SqliteUserRepository {
    /// Creates a new user in the database
    async fn create(&self, user: &mut User) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        sqlx::query!(
            "INSERT INTO users (
                id, 
                username, 
                email, 
                password_hash, 
                created_at, 
                updated_at,
                display_name,
                bio,
                avatar_url,
                friends,
                followers
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            user.id,
            user.username,
            user.email,
            user.password_hash,
            user.created_at,
            user.updated_at,
            user.display_name,
            user.bio,
            user.avatar_url,
            serde_json::to_string(&user.friends)?,
            serde_json::to_string(&user.followers)?,
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(())
    }

    /// Finds a user by ID
    async fn find_by_id(&self, user_id: Uuid) -> Result<Option<User>> {
        let row = sqlx::query!(
            "SELECT * FROM users WHERE id = ?",
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(self.map_row_to_user(&row)?)),
            None => Ok(None),
        }
    }

    /// Finds a user by email
    async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let row = sqlx::query!(
            "SELECT * FROM users WHERE email = ?",
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(self.map_row_to_user(&row)?)),
            None => Ok(None),
        }
    }

    /// Updates a user in the database
    async fn update(&self, user: &User) -> Result<()> {
        sqlx::query!(
            "UPDATE users SET
                username = ?,
                email = ?,
                password_hash = ?,
                updated_at = ?,
                display_name = ?,
                bio = ?,
                avatar_url = ?,
                friends = ?,
                followers = ?
            WHERE id = ?",
            user.username,
            user.email,
            user.password_hash,
            user.updated_at,
            user.display_name,
            user.bio,
            user.avatar_url,
            serde_json::to_string(&user.friends)?,
            serde_json::to_string(&user.followers)?,
            user.id,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Deletes a user from the database
    async fn delete(&self, user_id: Uuid) -> Result<()> {
        sqlx::query!(
            "DELETE FROM users WHERE id = ?",
            user_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}

impl SqliteUserRepository {
    /// Maps a database row to a User struct
    fn map_row_to_user(&self, row: &sqlx::sqlite::SqliteRow) -> Result<User> {
        let friends_json: String = row.try_get("friends")?;
        let followers_json: String = row.try_get("followers")?;

        Ok(User {
            id: row.try_get("id")?,
            username: row.try_get("username")?,
            email: row.try_get("email")?,
            password_hash: row.try_get("password_hash")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            display_name: row.try_get("display_name")?,
            bio: row.try_get("bio")?,
            avatar_url: row.try_get("avatar_url")?,
            friends: serde_json::from_str(&friends_json).unwrap_or_default(),
            followers: serde_json::from_str(&followers_json).unwrap_or_default(),
        })
    }
}