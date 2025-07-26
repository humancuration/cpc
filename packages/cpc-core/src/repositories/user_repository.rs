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
    async fn find_many_by_ids(&self, user_ids: &[Uuid]) -> Result<Vec<User>>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>>;
    async fn find_user_by_social_id(&self, provider: &str, social_id: &str) -> Result<Option<User>>;
    async fn find_all(&self) -> Result<Vec<User>>;
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
                auth_method,
                social_id,
                created_at,
                updated_at,
                display_name,
                bio,
                avatar_url,
                friends,
                followers
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            user.id,
            user.username,
            user.email,
            user.password_hash,
            user.auth_method,
            user.social_id,
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

    /// Finds users by a list of IDs
    async fn find_many_by_ids(&self, user_ids: &[Uuid]) -> Result<Vec<User>> {
        if user_ids.is_empty() {
            return Ok(Vec::new());
        }

        let placeholders = user_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
        let query_str = format!("SELECT * FROM users WHERE id IN ({})", placeholders);

        let mut query = sqlx::query(&query_str);
        for id in user_ids {
            query = query.bind(id);
        }

        let rows = query.fetch_all(&self.pool).await?;
        rows.iter().map(|row| self.map_row_to_user(row)).collect()
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

    /// Finds a user by social provider and social ID
    async fn find_user_by_social_id(&self, provider: &str, social_id: &str) -> Result<Option<User>> {
        let auth_method = match provider {
            "google" => "Google",
            "tiktok" => "Tiktok",
            "instagram" => "Instagram",
            _ => return Err(anyhow!("Unsupported provider: {}", provider)),
        };

        let row = sqlx::query!(
            "SELECT * FROM users WHERE auth_method = ? AND social_id = ?",
            auth_method,
            social_id
        )
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(self.map_row_to_user(&row)?)),
            None => Ok(None),
        }
    }

    /// Finds all users
    async fn find_all(&self) -> Result<Vec<User>> {
        let rows = sqlx::query!(
            "SELECT * FROM users"
        )
        .fetch_all(&self.pool)
        .await?;

        rows.iter().map(|row| self.map_row_to_user(row)).collect()
    }

    /// Updates a user in the database
    async fn update(&self, user: &User) -> Result<()> {
        sqlx::query!(
            "UPDATE users SET
                username = ?,
                email = ?,
                password_hash = ?,
                auth_method = ?,
                social_id = ?,
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
            user.auth_method,
            user.social_id,
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
            auth_method: row.try_get("auth_method")?,
            social_id: row.try_get("social_id")?,
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