use tauri::command;
use crate::types::User;
use sqlx::{PgPool, FromRow};
use tauri::State;
use argon2::{Argon2, password_hash::{SaltString, PasswordHasher, PasswordVerifier, PasswordHash, rand_core::OsRng}};
use anyhow::{anyhow, Context};
use regex::Regex;

#[derive(Debug, FromRow)]
struct DbUser {
    id: uuid::Uuid,
    name: String,
    email: String,
    password_hash: String,
}

#[command]
pub async fn login(
    email: String,
    password: String,
    db: State<'_, PgPool>
) -> Result<User, String> {
    // Validate email format
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    if !email_regex.is_match(&email) {
        return Err("Invalid email format".to_string());
    }

    // Validate password length
    if password.len() < 8 {
        return Err("Password must be at least 8 characters".to_string());
    }

    let user = sqlx::query_as!(
        DbUser,
        "SELECT id, name, email, password_hash FROM users WHERE email = $1",
        email
    )
    .fetch_optional(&*db)
    .await
    .map_err(|e| format!("Database error: {}", e))?;

    match user {
        Some(user) => {
            let parsed_hash = PasswordHash::new(&user.password_hash)
                .map_err(|e| format!("Hash error: {}", e))?;
            
            Argon2::default()
                .verify_password(password.as_bytes(), &parsed_hash)
                .map_err(|_| "Invalid password".to_string())?;
            
            Ok(User {
                id: user.id.to_string(),
                name: user.name,
                email: user.email,
            })
        }
        None => Err("User not found".to_string()),
    }
}

#[command]
pub async fn logout() -> Result<(), String> {
    // TODO: Invalidate session/token in future implementations
    Ok(())
}

#[command]
pub async fn register(
    email: String,
    password: String,
    name: String,
    db: State<'_, PgPool>
) -> Result<User, String> {
    // Validate inputs
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    if !email_regex.is_match(&email) {
        return Err("Invalid email format".to_string());
    }

    if password.len() < 8 {
        return Err("Password must be at least 8 characters".to_string());
    }

    if name.trim().is_empty() {
        return Err("Name cannot be empty".to_string());
    }

    // Hash password
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)
        .map_err(|e| format!("Password hashing failed: {}", e))?
        .to_string();

    let user = sqlx::query_as!(
        DbUser,
        "INSERT INTO users (email, password_hash, name) VALUES ($1, $2, $3) RETURNING id, name, email, password_hash",
        email, password_hash, name
    )
    .fetch_one(&*db)
    .await
    .map_err(|e| format!("Database error: {}", e))?;

    Ok(User {
        id: user.id.to_string(),
        name: user.name,
        email: user.email,
    })
}

pub fn register_commands(builder: tauri::Builder) -> tauri::Builder {
    builder
}