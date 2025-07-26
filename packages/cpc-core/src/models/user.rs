// DateTime handling follows the standard defined in [DATETIME_STANDARD.md](../../docs/DATETIME_STANDARD.md)
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum AuthMethod {
    Email,
    Google,
    Tiktok,
    Instagram,
    Passwordless,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OneTimeToken {
    pub id: Uuid,
    pub token: String,
    pub user_id: Option<Uuid>,
    pub email: String,
    #[serde(with = "crate::utils::datetime")]
    pub expires_at: DateTime<Utc>,
    #[serde(with = "crate::utils::datetime")]
    pub created_at: DateTime<Utc>,
    pub used: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,  // Store hashed passwords only
    pub auth_method: AuthMethod,
    pub social_id: Option<String>,
    #[serde(with = "crate::utils::datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "crate::utils::datetime")]
    pub updated_at: DateTime<Utc>,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub friends: Vec<Uuid>,
    pub followers: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserProfile {
    pub user_id: Uuid,
    pub display_name: String,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub cooperative_score: CooperativeScore,
    #[serde(with = "crate::utils::datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "crate::utils::datetime")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CooperativeScore {
    pub value: f64,
    #[serde(with = "crate::utils::datetime")]
    pub last_updated: DateTime<Utc>,
    pub contribution_factors: Vec<ContributionFactor>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContributionFactor {
    pub name: String,
    pub weight: f64,
    pub value: f64,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum UserRelationshipType {
    Following,
    Blocked,
    Muted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserRelationship {
    pub id: Uuid,
    pub user_id: Uuid,
    pub related_user_id: Uuid,
    pub relationship_type: UserRelationshipType,
    #[serde(with = "crate::utils::datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "crate::utils::datetime")]
    pub updated_at: DateTime<Utc>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,  // Plaintext password for registration
    pub display_name: Option<String>,
    pub auth_method: AuthMethod,
    pub social_id: Option<String>,
}
    pub auth_method: AuthMethod,
    pub social_id: Option<String>,
}

impl NewUser {
    /// Validates user input with a focus on security
    pub fn validate(&self) -> Result<(), String> {
        // Username validation
        if self.username.len() < 3 {
            return Err("Username must be at least 3 characters".to_string());
        }
        
        // Email validation
        if !self.email.contains('@') {
            return Err("Invalid email format".to_string());
        }
        
        // Password strength requirements
        if self.password.len() < 8 {
            return Err("Password must be at least 8 characters".to_string());
        }
        
        // Display name validation (if provided)
        if let Some(display_name) = &self.display_name {
            if display_name.len() > 50 {
                return Err("Display name must be 50 characters or less".to_string());
            }
        }
        
        Ok(())
    }

    /// Updates the user's password with a new one
    pub fn update_password(&mut self, new_password: String) {
        self.password = new_password;
    }
}

impl UserProfile {
    /// Creates a new UserProfile with default cooperative score
    pub fn new(user_id: Uuid, display_name: String) -> Self {
        let now = Utc::now();
        Self {
            user_id,
            display_name,
            bio: None,
            avatar_url: None,
            cooperative_score: CooperativeScore::default(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Updates the profile's updated_at timestamp
    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }
}

impl Default for CooperativeScore {
    fn default() -> Self {
        Self {
            value: 0.0,
            last_updated: Utc::now(),
            contribution_factors: Vec::new(),
        }
    }
}

impl CooperativeScore {
    /// Creates a new cooperative score with initial factors
    pub fn new() -> Self {
        Self::default()
    }

    /// Updates the cooperative score based on contribution factors
    /// TODO: This is a placeholder implementation - needs proper theorycraft
    pub fn recalculate(&mut self) {
        // Placeholder calculation - sum weighted factors
        self.value = self.contribution_factors
            .iter()
            .map(|factor| factor.weight * factor.value)
            .sum();
        self.last_updated = Utc::now();
    }

    /// Adds or updates a contribution factor
    pub fn update_factor(&mut self, name: String, weight: f64, value: f64, description: Option<String>) {
        if let Some(existing) = self.contribution_factors.iter_mut().find(|f| f.name == name) {
            existing.weight = weight;
            existing.value = value;
            existing.description = description;
        } else {
            self.contribution_factors.push(ContributionFactor {
                name,
                weight,
                value,
                description,
            });
        }
        self.recalculate();
    }

    /// Removes a contribution factor by name
    pub fn remove_factor(&mut self, name: &str) {
        self.contribution_factors.retain(|f| f.name != name);
        self.recalculate();
    }
}

impl UserRelationship {
    /// Creates a new user relationship
    pub fn new(user_id: Uuid, related_user_id: Uuid, relationship_type: UserRelationshipType) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            related_user_id,
            relationship_type,
            created_at: now,
            updated_at: now,
        }
    }

    /// Creates a following relationship
    pub fn follow(user_id: Uuid, target_user_id: Uuid) -> Self {
        Self::new(user_id, target_user_id, UserRelationshipType::Following)
    }

    /// Creates a blocking relationship
    pub fn block(user_id: Uuid, target_user_id: Uuid) -> Self {
        Self::new(user_id, target_user_id, UserRelationshipType::Blocked)
    }

    /// Creates a muting relationship
    pub fn mute(user_id: Uuid, target_user_id: Uuid) -> Self {
        Self::new(user_id, target_user_id, UserRelationshipType::Muted)
    }

    /// Checks if this relationship is a follow
    pub fn is_following(&self) -> bool {
        self.relationship_type == UserRelationshipType::Following
    }

    /// Checks if this relationship is a block
    pub fn is_blocked(&self) -> bool {
        self.relationship_type == UserRelationshipType::Blocked
    }

    /// Checks if this relationship is a mute
    pub fn is_muted(&self) -> bool {
        self.relationship_type == UserRelationshipType::Muted
    }

    /// Updates the relationship's updated_at timestamp
    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_user_serialization() {
        let dt = Utc.with_ymd_and_hms(2025, 7, 22, 1, 42, 45).unwrap().with_nanosecond(82000000).unwrap();
        let user = User {
            id: Uuid::nil(), // Using nil UUID for test
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "hashed_password".to_string(),
            created_at: dt,
            updated_at: dt,
            display_name: Some("Test User".to_string()),
            bio: Some("Test bio".to_string()),
            avatar_url: Some("https://example.com/avatar.jpg".to_string()),
            friends: vec![],
            followers: vec![],
        };

        let json = serde_json::to_string(&user).unwrap();
        assert!(json.contains("\"created_at\":\"2025-07-22T01:42:45.082Z\""));
        assert!(json.contains("\"displayName\":\"Test User\""));
        assert!(json.contains("\"bio\":\"Test bio\""));
        assert!(json.contains("\"avatarUrl\":\"https://example.com/avatar.jpg\""));
    }

    #[test]
    fn test_user_profile_creation() {
        let user_id = Uuid::new_v4();
        let profile = UserProfile::new(user_id, "Test User".to_string());
        
        assert_eq!(profile.user_id, user_id);
        assert_eq!(profile.display_name, "Test User");
        assert!(profile.bio.is_none());
        assert!(profile.avatar_url.is_none());
        assert_eq!(profile.cooperative_score.value, 0.0);
    }

    #[test]
    fn test_cooperative_score_calculation() {
        let mut score = CooperativeScore::new();
        
        // Add some contribution factors
        score.update_factor("posts".to_string(), 0.3, 10.0, Some("Number of posts".to_string()));
        score.update_factor("likes".to_string(), 0.2, 25.0, Some("Likes received".to_string()));
        score.update_factor("comments".to_string(), 0.5, 8.0, Some("Quality comments".to_string()));
        
        // Score should be: (0.3 * 10.0) + (0.2 * 25.0) + (0.5 * 8.0) = 3.0 + 5.0 + 4.0 = 12.0
        assert_eq!(score.value, 12.0);
        assert_eq!(score.contribution_factors.len(), 3);
    }

    #[test]
    fn test_user_relationship_creation() {
        let user_id = Uuid::new_v4();
        let target_id = Uuid::new_v4();
        
        let follow_rel = UserRelationship::follow(user_id, target_id);
        assert!(follow_rel.is_following());
        assert!(!follow_rel.is_blocked());
        assert!(!follow_rel.is_muted());
        
        let block_rel = UserRelationship::block(user_id, target_id);
        assert!(!block_rel.is_following());
        assert!(block_rel.is_blocked());
        assert!(!block_rel.is_muted());
        
        let mute_rel = UserRelationship::mute(user_id, target_id);
        assert!(!mute_rel.is_following());
        assert!(!mute_rel.is_blocked());
        assert!(mute_rel.is_muted());
    }

    #[test]
    fn test_relationship_type_serialization() {
        let relationship = UserRelationship::follow(Uuid::new_v4(), Uuid::new_v4());
        let json = serde_json::to_string(&relationship).unwrap();
        assert!(json.contains("\"relationshipType\":\"following\""));
    }
}