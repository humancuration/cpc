use tauri::command;
use crate::types::Proposal;
use sqlx::{PgPool, FromRow, Postgres, Transaction};
use tauri::State;
use anyhow::Context;
use uuid::Uuid;

#[derive(Debug, FromRow)]
struct DbProposal {
    id: Uuid,
    title: String,
    description: String,
    votes_for: i32,
    votes_against: i32,
}

#[command]
pub async fn create_proposal(
    title: String,
    description: String,
    creator_id: String,
    db: State<'_, PgPool>
) -> Result<Proposal, String> {
    // Validate inputs
    if title.trim().is_empty() {
        return Err("Title cannot be empty".to_string());
    }
    
    if description.trim().is_empty() {
        return Err("Description cannot be empty".to_string());
    }
    
    let creator_uuid = Uuid::parse_str(&creator_id)
        .map_err(|e| format!("Invalid creator ID: {}", e))?;

    let proposal = sqlx::query_as!(
        DbProposal,
        "INSERT INTO proposals (title, description, created_by) VALUES ($1, $2, $3) RETURNING id, title, description, votes_for, votes_against",
        title, description, creator_uuid
    )
    .fetch_one(&*db)
    .await
    .map_err(|e| format!("Database error: {}", e))?;

    Ok(Proposal {
        id: proposal.id.to_string(),
        title: proposal.title,
        description: proposal.description,
        votes_for: proposal.votes_for,
        votes_against: proposal.votes_against,
    })
}

#[command]
pub async fn vote(
    proposal_id: String,
    user_id: String,
    vote_type: String,
    db: State<'_, PgPool>
) -> Result<Proposal, String> {
    // Validate vote type
    if vote_type != "for" && vote_type != "against" {
        return Err("Invalid vote type. Must be 'for' or 'against'".to_string());
    }
    
    let proposal_uuid = Uuid::parse_str(&proposal_id)
        .map_err(|e| format!("Invalid proposal ID: {}", e))?;
    
    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|e| format!("Invalid user ID: {}", e))?;

    // Start a transaction
    let mut tx: Transaction<Postgres> = db.begin()
        .await
        .map_err(|e| format!("Failed to start transaction: {}", e))?;

    // Check for existing vote
    let existing_vote = sqlx::query!(
        "SELECT vote_type FROM votes WHERE user_id = $1 AND proposal_id = $2",
        user_uuid,
        proposal_uuid
    )
    .fetch_optional(&mut tx)
    .await
    .map_err(|e| format!("Database error: {}", e))?;

    match existing_vote {
        Some(record) => {
            // Update existing vote
            if record.vote_type != vote_type {
                // Update the vote type
                sqlx::query!(
                    "UPDATE votes SET vote_type = $1 WHERE user_id = $2 AND proposal_id = $3",
                    vote_type,
                    user_uuid,
                    proposal_uuid
                )
                .execute(&mut tx)
                .await
                .map_err(|e| format!("Failed to update vote: {}", e))?;

                // Update proposal vote counts
                if record.vote_type == "for" {
                    // Changing from for to against
                    sqlx::query!(
                        "UPDATE proposals SET votes_for = votes_for - 1, votes_against = votes_against + 1 WHERE id = $1",
                        proposal_uuid
                    )
                    .execute(&mut tx)
                    .await
                    .map_err(|e| format!("Failed to update proposal votes: {}", e))?;
                } else {
                    // Changing from against to for
                    sqlx::query!(
                        "UPDATE proposals SET votes_against = votes_against - 1, votes_for = votes_for + 1 WHERE id = $1",
                        proposal_uuid
                    )
                    .execute(&mut tx)
                    .await
                    .map_err(|e| format!("Failed to update proposal votes: {}", e))?;
                }
            }
        }
        None => {
            // Create new vote
            sqlx::query!(
                "INSERT INTO votes (user_id, proposal_id, vote_type) VALUES ($1, $2, $3)",
                user_uuid,
                proposal_uuid,
                vote_type
            )
            .execute(&mut tx)
            .await
            .map_err(|e| format!("Failed to insert vote: {}", e))?;

            // Update proposal vote counts
            if vote_type == "for" {
                sqlx::query!(
                    "UPDATE proposals SET votes_for = votes_for + 1 WHERE id = $1",
                    proposal_uuid
                )
                .execute(&mut tx)
                .await
                .map_err(|e| format!("Failed to update proposal votes: {}", e))?;
            } else {
                sqlx::query!(
                    "UPDATE proposals SET votes_against = votes_against + 1 WHERE id = $1",
                    proposal_uuid
                )
                .execute(&mut tx)
                .await
                .map_err(|e| format!("Failed to update proposal votes: {}", e))?;
            }
        }
    }

    // Get updated proposal
    let proposal = sqlx::query_as!(
        DbProposal,
        "SELECT id, title, description, votes_for, votes_against FROM proposals WHERE id = $1",
        proposal_uuid
    )
    .fetch_one(&mut tx)
    .await
    .map_err(|e| format!("Failed to fetch updated proposal: {}", e))?;

    // Commit transaction
    tx.commit()
        .await
        .map_err(|e| format!("Failed to commit transaction: {}", e))?;

    Ok(Proposal {
        id: proposal.id.to_string(),
        title: proposal.title,
        description: proposal.description,
        votes_for: proposal.votes_for,
        votes_against: proposal.votes_against,
    })
}

pub fn register_commands(builder: tauri::Builder) -> tauri::Builder {
    builder
}