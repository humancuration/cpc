use crate::{
    models::user::User,
    utils::{datetime::to_utc_datetime, password},
};
use anyhow::{anyhow, Result};
use rusqlite::{params, Connection, OptionalExtension};
use serde_json;
use uuid::Uuid;

/// Repository for user database operations
pub struct UserRepository {
    conn: Connection,
}

impl UserRepository {
    /// Creates a new UserRepository instance
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }

    /// Creates a new user in the database
    pub async fn create(&self, user: &mut User) -> Result<()> {
        let tx = self.conn.transaction()?;

        tx.execute(
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
            params![
                user.id.to_string(),
                user.username,
                user.email,
                user.password_hash,
                to_utc_datetime(&user.created_at),
                to_utc_datetime(&user.updated_at),
                user.display_name,
                user.bio,
                user.avatar_url,
                serde_json::to_string(&user.friends)?,
                serde_json::to_string(&user.followers)?,
            ],
        )?;

        tx.commit()?;
        Ok(())
    }

    /// Finds a user by ID
    pub async fn find_by_id(&self, user_id: Uuid) -> Result<Option<User>> {
        self.conn
            .query_row(
                "SELECT * FROM users WHERE id = ?",
                [user_id.to_string()],
                |row| self.map_row_to_user(row),
            )
            .optional()
            .map_err(|e| anyhow!("Database error: {}", e))
    }

    /// Finds a user by email
    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        self.conn
            .query_row("SELECT * FROM users WHERE email = ?", [email], |row| {
                self.map_row_to_user(row)
            })
            .optional()
            .map_err(|e| anyhow!("Database error: {}", e))
    }

    /// Updates a user in the database
    pub async fn update(&self, user: &User) -> Result<()> {
        self.conn.execute(
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
            params![
                user.username,
                user.email,
                user.password_hash,
                to_utc_datetime(&user.updated_at),
                user.display_name,
                user.bio,
                user.avatar_url,
                serde_json::to_string(&user.friends)?,
                serde_json::to_string(&user.followers)?,
                user.id.to_string(),
            ],
        )?;

        Ok(())
    }

    /// Deletes a user from the database
    pub async fn delete(&self, user_id: Uuid) -> Result<()> {
        self.conn.execute(
            "DELETE FROM users WHERE id = ?",
            [user_id.to_string()],
        )?;
        Ok(())
    }

    /// Maps a database row to a User struct
    fn map_row_to_user(&self, row: &rusqlite::Row) -> rusqlite::Result<User> {
        let friends_json: String = row.get("friends")?;
        let followers_json: String = row.get("followers")?;

        Ok(User {
            id: Uuid::parse_str(&row.get::<_, String>("id")?).unwrap(),
            username: row.get("username")?,
            email: row.get("email")?,
            password_hash: row.get("password_hash")?,
            created_at: row.get("created_at")?,
            updated_at: row.get("updated_at")?,
            display_name: row.get("display_name")?,
            bio: row.get("bio")?,
            avatar_url: row.get("avatar_url")?,
            friends: serde_json::from_str(&friends_json).unwrap_or_default(),
            followers: serde_json::from_str(&followers_json).unwrap_or_default(),
        })
    }
}