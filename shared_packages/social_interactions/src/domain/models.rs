//! Domain models for social interactions
//!
//! This module defines the core entities for reactions, comments, and shares.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Types of reactions users can give
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ReactionType {
    Like,
    Heart,
    Celebrate,
    Insightful,
    Funny,
    Sad,
    Angry,
}

impl std::fmt::Display for ReactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReactionType::Like => write!(f, "like"),
            ReactionType::Heart => write!(f, "heart"),
            ReactionType::Celebrate => write!(f, "celebrate"),
            ReactionType::Insightful => write!(f, "insightful"),
            ReactionType::Funny => write!(f, "funny"),
            ReactionType::Sad => write!(f, "sad"),
            ReactionType::Angry => write!(f, "angry"),
        }
    }
}

/// Types of targets that can be reacted to or commented on
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TargetType {
    Post,
    Comment,
    Achievement,
    VolunteerActivity,
    SkillExchange,
}

impl std::fmt::Display for TargetType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TargetType::Post => write!(f, "post"),
            TargetType::Comment => write!(f, "comment"),
            TargetType::Achievement => write!(f, "achievement"),
            TargetType::VolunteerActivity => write!(f, "volunteer_activity"),
            TargetType::SkillExchange => write!(f, "skill_exchange"),
        }
    }
}

/// Types of content that can be shared
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ContentType {
    Post,
    Achievement,
    VolunteerActivity,
    SkillExchange,
    Comment,
}

impl std::fmt::Display for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContentType::Post => write!(f, "post"),
            ContentType::Achievement => write!(f, "achievement"),
            ContentType::VolunteerActivity => write!(f, "volunteer_activity"),
            ContentType::SkillExchange => write!(f, "skill_exchange"),
            ContentType::Comment => write!(f, "comment"),
        }
    }
}

/// Reaction entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reaction {
    pub id: Uuid,
    pub user_id: Uuid,
    pub target_id: Uuid,
    pub target_type: TargetType,
    pub reaction_type: ReactionType,
    pub created_at: DateTime<Utc>,
}

impl Reaction {
    pub fn new(
        user_id: Uuid,
        target_id: Uuid,
        target_type: TargetType,
        reaction_type: ReactionType,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            target_id,
            target_type,
            reaction_type,
            created_at: Utc::now(),
        }
    }
}

/// Comment entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: Uuid,
    pub user_id: Uuid,
    pub parent_id: Option<Uuid>, // For nested comments
    pub target_id: Uuid,
    pub target_type: TargetType,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Comment {
    pub fn new(
        user_id: Uuid,
        target_id: Uuid,
        target_type: TargetType,
        content: String,
        parent_id: Option<Uuid>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            parent_id,
            target_id,
            target_type,
            content,
            created_at: Utc::now(),
            updated_at: None,
        }
    }

    pub fn update_content(&mut self, new_content: String) {
        self.content = new_content;
        self.updated_at = Some(Utc::now());
    }
}

/// Share entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Share {
    pub id: Uuid,
    pub user_id: Uuid,
    pub content_id: Uuid,
    pub content_type: ContentType,
    pub shared_with: Option<Uuid>, // None for public, Some(user_id) for private share
    pub created_at: DateTime<Utc>,
}

impl Share {
    pub fn new(
        user_id: Uuid,
        content_id: Uuid,
        content_type: ContentType,
        shared_with: Option<Uuid>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            content_id,
            content_type,
            shared_with,
            created_at: Utc::now(),
        }
    }

    pub fn is_public(&self) -> bool {
        self.shared_with.is_none()
    }
}