//! Domain models for user skill management.

use crate::skill_management::models::Skill;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::fmt::{self, Display};
use std::str::FromStr;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Type)]
#[sqlx(type_name = "skill_level_enum", rename_all = "lowercase")]
pub enum SkillLevel {
    Beginner,
    Intermediate,
    Advanced,
}

impl Display for SkillLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SkillLevel::Beginner => write!(f, "beginner"),
            SkillLevel::Intermediate => write!(f, "intermediate"),
            SkillLevel::Advanced => write!(f, "advanced"),
        }
    }
}

#[derive(Debug, Error)]
#[error("Invalid skill level")]
pub struct ParseSkillLevelError;

impl FromStr for SkillLevel {
    type Err = ParseSkillLevelError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "beginner" => Ok(SkillLevel::Beginner),
            "intermediate" => Ok(SkillLevel::Intermediate),
            "advanced" => Ok(SkillLevel::Advanced),
            _ => Err(ParseSkillLevelError),
        }
    }
}

/// Represents a skill associated with a user in the database.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct UserSkill {
    pub user_id: Uuid,
    pub skill_id: Uuid,
    pub skill_level: SkillLevel,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserSkill {
    pub fn new(user_id: Uuid, skill_id: Uuid, skill_level: SkillLevel) -> Self {
        let now = Utc::now();
        Self {
            user_id,
            skill_id,
            skill_level,
            created_at: now,
            updated_at: now,
        }
    }
}

/// Represents detailed information about a user's skill, including the skill's name and category.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct UserSkillDetails {
    pub user_id: Uuid,
    #[sqlx(flatten)]
    pub skill: Skill,
    pub skill_level: SkillLevel,
    pub created_at: DateTime<Utc>,
}