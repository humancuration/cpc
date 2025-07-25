use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use cpc_core::models::governance::{Proposal, Vote, VoteChoice, ProposalStatus};
use cpc_core::services::governance::GovernanceService;
use cpc_core::error::CpcError;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateProposalRequest {
    pub title: String,
    pub description: String,
    pub proposal_type: String,
    pub voting_deadline: DateTime<Utc>,
    pub options: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CastVoteRequest {
    pub proposal_id: Uuid,
    pub choice: VoteChoice,
    pub weight: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GovernanceResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProposalWithVotes {
    pub proposal: Proposal,
    pub total_votes: i64,
    pub user_vote: Option<Vote>,
    pub vote_distribution: Vec<VoteDistribution>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VoteDistribution {
    pub choice: VoteChoice,
    pub count: i64,
    pub weight: f64,
}

/// Tauri command to create a new proposal
#[tauri::command]
pub async fn create_proposal(
    user_id: Uuid,
    request: CreateProposalRequest,
    governance_service: State<'_, GovernanceService>,
) -> Result<GovernanceResponse<Proposal>, String> {
    match governance_service.create_proposal(
        user_id,
        &request.title,
        &request.description,
        &request.proposal_type,
        request.voting_deadline,
        request.options,
    ).await {
        Ok(proposal) => Ok(GovernanceResponse {
            success: true,
            data: Some(proposal),
            error: None,
        }),
        Err(e) => Ok(GovernanceResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

/// Tauri command to get all active proposals
#[tauri::command]
pub async fn get_active_proposals(
    user_id: Option<Uuid>,
    limit: Option<i32>,
    offset: Option<i32>,
    governance_service: State<'_, GovernanceService>,
) -> Result<GovernanceResponse<Vec<ProposalWithVotes>>, String> {
    match governance_service.get_active_proposals(
        limit.unwrap_or(20),
        offset.unwrap_or(0),
    ).await {
        Ok(proposals) => {
            let mut proposals_with_votes = Vec::new();
            
            for proposal in proposals {
                let total_votes = governance_service.get_vote_count(proposal.id).await.unwrap_or(0);
                let user_vote = if let Some(uid) = user_id {
                    governance_service.get_user_vote(proposal.id, uid).await.ok().flatten()
                } else {
                    None
                };
                let vote_distribution = governance_service.get_vote_distribution(proposal.id).await.unwrap_or_default();
                
                proposals_with_votes.push(ProposalWithVotes {
                    proposal,
                    total_votes,
                    user_vote,
                    vote_distribution: vote_distribution.into_iter().map(|(choice, count, weight)| {
                        VoteDistribution { choice, count, weight }
                    }).collect(),
                });
            }
            
            Ok(GovernanceResponse {
                success: true,
                data: Some(proposals_with_votes),
                error: None,
            })
        },
        Err(e) => Ok(GovernanceResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

/// Tauri command to get a specific proposal by ID
#[tauri::command]
pub async fn get_proposal(
    proposal_id: Uuid,
    user_id: Option<Uuid>,
    governance_service: State<'_, GovernanceService>,
) -> Result<GovernanceResponse<ProposalWithVotes>, String> {
    match governance_service.get_proposal_by_id(proposal_id).await {
        Ok(Some(proposal)) => {
            let total_votes = governance_service.get_vote_count(proposal.id).await.unwrap_or(0);
            let user_vote = if let Some(uid) = user_id {
                governance_service.get_user_vote(proposal.id, uid).await.ok().flatten()
            } else {
                None
            };
            let vote_distribution = governance_service.get_vote_distribution(proposal.id).await.unwrap_or_default();
            
            let proposal_with_votes = ProposalWithVotes {
                proposal,
                total_votes,
                user_vote,
                vote_distribution: vote_distribution.into_iter().map(|(choice, count, weight)| {
                    VoteDistribution { choice, count, weight }
                }).collect(),
            };
            
            Ok(GovernanceResponse {
                success: true,
                data: Some(proposal_with_votes),
                error: None,
            })
        },
        Ok(None) => Ok(GovernanceResponse {
            success: false,
            data: None,
            error: Some("Proposal not found".to_string()),
        }),
        Err(e) => Ok(GovernanceResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

/// Tauri command to cast a vote on a proposal
#[tauri::command]
pub async fn cast_vote(
    user_id: Uuid,
    request: CastVoteRequest,
    governance_service: State<'_, GovernanceService>,
) -> Result<GovernanceResponse<Vote>, String> {
    match governance_service.cast_vote(
        user_id,
        request.proposal_id,
        request.choice,
        request.weight,
    ).await {
        Ok(vote) => Ok(GovernanceResponse {
            success: true,
            data: Some(vote),
            error: None,
        }),
        Err(e) => Ok(GovernanceResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

/// Tauri command to get user's voting history
#[tauri::command]
pub async fn get_user_votes(
    user_id: Uuid,
    limit: Option<i32>,
    offset: Option<i32>,
    governance_service: State<'_, GovernanceService>,
) -> Result<GovernanceResponse<Vec<Vote>>, String> {
    match governance_service.get_user_votes(
        user_id,
        limit.unwrap_or(20),
        offset.unwrap_or(0),
    ).await {
        Ok(votes) => Ok(GovernanceResponse {
            success: true,
            data: Some(votes),
            error: None,
        }),
        Err(e) => Ok(GovernanceResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

/// Tauri command to get proposals created by a user
#[tauri::command]
pub async fn get_user_proposals(
    user_id: Uuid,
    limit: Option<i32>,
    offset: Option<i32>,
    governance_service: State<'_, GovernanceService>,
) -> Result<GovernanceResponse<Vec<Proposal>>, String> {
    match governance_service.get_user_proposals(
        user_id,
        limit.unwrap_or(20),
        offset.unwrap_or(0),
    ).await {
        Ok(proposals) => Ok(GovernanceResponse {
            success: true,
            data: Some(proposals),
            error: None,
        }),
        Err(e) => Ok(GovernanceResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

/// Tauri command to update proposal status (admin only)
#[tauri::command]
pub async fn update_proposal_status(
    user_id: Uuid,
    proposal_id: Uuid,
    status: ProposalStatus,
    governance_service: State<'_, GovernanceService>,
) -> Result<GovernanceResponse<Proposal>, String> {
    match governance_service.update_proposal_status(user_id, proposal_id, status).await {
        Ok(proposal) => Ok(GovernanceResponse {
            success: true,
            data: Some(proposal),
            error: None,
        }),
        Err(e) => Ok(GovernanceResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

/// Tauri command to get governance statistics
#[tauri::command]
pub async fn get_governance_stats(
    governance_service: State<'_, GovernanceService>,
) -> Result<GovernanceResponse<serde_json::Value>, String> {
    match governance_service.get_governance_statistics().await {
        Ok(stats) => {
            let stats_json = serde_json::json!({
                "total_proposals": stats.total_proposals,
                "active_proposals": stats.active_proposals,
                "total_votes": stats.total_votes,
                "unique_voters": stats.unique_voters,
                "participation_rate": stats.participation_rate,
            });
            
            Ok(GovernanceResponse {
                success: true,
                data: Some(stats_json),
                error: None,
            })
        },
        Err(e) => Ok(GovernanceResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}