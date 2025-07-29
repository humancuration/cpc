use tauri::command;
use crate::types::{Post, Comment};
use sqlx::{PgPool, FromRow};
use tauri::State;
use anyhow::Context;
use uuid::Uuid;

#[derive(Debug, FromRow)]
struct DbPost {
    id: Uuid,
    content: String,
    author_id: Uuid,
    likes: i32,
}

#[derive(Debug, FromRow)]
struct DbComment {
    id: Uuid,
    post_id: Uuid,
    author_id: Uuid,
    content: String,
}

#[command]
pub async fn create_post(
    content: String,
    author_id: String,
    db: State<'_, PgPool>
) -> Result<Post, String> {
    // Validate content
    if content.trim().is_empty() {
        return Err("Content cannot be empty".to_string());
    }

    let author_uuid = Uuid::parse_str(&author_id)
        .map_err(|e| format!("Invalid author ID: {}", e))?;

    let post = sqlx::query_as!(
        DbPost,
        "INSERT INTO posts (content, author_id) VALUES ($1, $2) RETURNING id, content, author_id, likes",
        content, author_uuid
    )
    .fetch_one(&*db)
    .await
    .map_err(|e| format!("Database error: {}", e))?;

    Ok(Post {
        id: post.id.to_string(),
        content: post.content,
        author_id: post.author_id.to_string(),
        likes: post.likes,
        comments: Vec::new(),
    })
}

#[command]
pub async fn like_post(
    post_id: String,
    db: State<'_, PgPool>
) -> Result<Post, String> {
    let post_uuid = Uuid::parse_str(&post_id)
        .map_err(|e| format!("Invalid post ID: {}", e))?;

    let post = sqlx::query_as!(
        DbPost,
        "UPDATE posts SET likes = likes + 1 WHERE id = $1 RETURNING id, content, author_id, likes",
        post_uuid
    )
    .fetch_one(&*db)
    .await
    .map_err(|e| format!("Database error: {}", e))?;

    Ok(Post {
        id: post.id.to_string(),
        content: post.content,
        author_id: post.author_id.to_string(),
        likes: post.likes,
        comments: Vec::new(),
    })
}

#[command]
pub async fn comment_post(
    post_id: String,
    content: String,
    author_id: String,
    db: State<'_, PgPool>
) -> Result<Comment, String> {
    // Validate content
    if content.trim().is_empty() {
        return Err("Content cannot be empty".to_string());
    }

    let post_uuid = Uuid::parse_str(&post_id)
        .map_err(|e| format!("Invalid post ID: {}", e))?;
    
    let author_uuid = Uuid::parse_str(&author_id)
        .map_err(|e| format!("Invalid author ID: {}", e))?;

    let comment = sqlx::query_as!(
        DbComment,
        "INSERT INTO comments (post_id, author_id, content) VALUES ($1, $2, $3) RETURNING id, post_id, author_id, content",
        post_uuid, author_uuid, content
    )
    .fetch_one(&*db)
    .await
    .map_err(|e| format!("Database error: {}", e))?;

    Ok(Comment {
        id: comment.id.to_string(),
        post_id: comment.post_id.to_string(),
        author_id: comment.author_id.to_string(),
        content: comment.content,
    })
}

pub fn register_commands(builder: tauri::Builder) -> tauri::Builder {
    builder
}