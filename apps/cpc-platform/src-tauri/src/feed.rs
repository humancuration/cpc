use tauri::command;
use crate::types::FeedItem;
use sqlx::{PgPool, FromRow};
use tauri::State;
use anyhow::Context;
use chrono::{DateTime, Utc};

#[derive(Debug, FromRow)]
struct FeedPost {
    id: uuid::Uuid,
    content: String,
    author_id: uuid::Uuid,
    created_at: DateTime<Utc>,
    likes: i32,
    comment_count: i64,
}

#[derive(Debug, FromRow)]
struct FeedProposal {
    id: uuid::Uuid,
    title: String,
    description: String,
    votes_for: i32,
    votes_against: i32,
    created_at: DateTime<Utc>,
}

#[command]
pub async fn get_feed(db: State<'_, PgPool>) -> Result<Vec<FeedItem>, String> {
    // Fetch recent posts (last 50) with comment counts
    let posts = sqlx::query_as!(
        FeedPost,
        r#"
        SELECT
            p.id,
            p.content,
            p.author_id,
            p.created_at,
            p.likes,
            COUNT(c.id) as "comment_count!"
        FROM posts p
        LEFT JOIN comments c ON p.id = c.post_id
        GROUP BY p.id
        ORDER BY p.created_at DESC
        LIMIT 50
        "#
    )
    .fetch_all(&*db)
    .await
    .map_err(|e| format!("Failed to fetch posts: {}", e))?;

    // Fetch recent proposals (last 50)
    let proposals = sqlx::query_as!(
        FeedProposal,
        r#"
        SELECT
            id,
            title,
            description,
            votes_for,
            votes_against,
            created_at
        FROM proposals
        ORDER BY created_at DESC
        LIMIT 50
        "#
    )
    .fetch_all(&*db)
    .await
    .map_err(|e| format!("Failed to fetch proposals: {}", e))?;

    // Combine posts and proposals into a single feed
    let mut feed_items = Vec::new();

    // Convert posts to FeedItems
    for post in posts {
        feed_items.push(FeedItem::Post {
            id: post.id.to_string(),
            content: post.content,
            author_id: post.author_id.to_string(),
            likes: post.likes,
            comments: post.comment_count as u32,
        });
    }

    // Convert proposals to FeedItems
    for proposal in proposals {
        feed_items.push(FeedItem::Proposal {
            id: proposal.id.to_string(),
            title: proposal.title,
            description: proposal.description,
            votes_for: proposal.votes_for,
            votes_against: proposal.votes_against,
        });
    }

    // Sort by creation date (newest first)
    feed_items.sort_by(|a, b| {
        let a_time = match a {
            FeedItem::Post { .. } => posts.iter().find(|p| p.id.to_string() == a.id()).map(|p| p.created_at),
            FeedItem::Proposal { .. } => proposals.iter().find(|p| p.id.to_string() == a.id()).map(|p| p.created_at),
        };
        let b_time = match b {
            FeedItem::Post { .. } => posts.iter().find(|p| p.id.to_string() == b.id()).map(|p| p.created_at),
            FeedItem::Proposal { .. } => proposals.iter().find(|p| p.id.to_string() == b.id()).map(|p| p.created_at),
        };
        
        b_time.cmp(&a_time) // Descending order
    });

    Ok(feed_items)
}

pub fn register_commands(builder: tauri::Builder) -> tauri::Builder {
    builder
}