use crate::{
    models::user::{User, UserProfile, CooperativeScore, ContributionFactor, UserRelationship, UserRelationshipType, NewUser},
    repositories::user_repository::UserRepository,
    utils::{datetime::now_utc, password},
    error::PublishError,
};
use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc, Duration};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// JWT claims structure for authentication
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid, // User ID
    pub exp: usize, // Expiration time (as UTC timestamp)
    pub iat: usize, // Issued at time
}

/// Privacy settings for user profiles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacySettings {
    pub profile_visibility: ProfileVisibility,
    pub show_cooperative_score: bool,
    pub show_contribution_factors: bool,
    pub allow_direct_messages: bool,
    pub show_activity_status: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProfileVisibility {
    Public,
    FriendsOnly,
    Private,
}

/// Contribution tracking for cooperative score calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContributionRecord {
    pub id: Uuid,
    pub user_id: Uuid,
    pub contribution_type: String,
    pub description: String,
    pub points: i32,
    pub verified: bool,
    pub verified_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub verified_at: Option<DateTime<Utc>>,
}

/// Enhanced Identity Service for user authentication and profile management
/// Ports functionality from the legacy Android codebase's feature_identity module
pub struct IdentityService {
    user_repo: Box<dyn UserRepository>,
    jwt_secret: String,
    // In-memory fraud detection state (in production, this would be in Redis or similar)
    user_contribution_timestamps: std::sync::Mutex<HashMap<Uuid, Vec<DateTime<Utc>>>>,
}

impl IdentityService {
    /// Creates a new IdentityService instance
    pub fn new(user_repo: Box<dyn UserRepository>, jwt_secret: String) -> Self {
        Self {
            user_repo,
            jwt_secret,
            user_contribution_timestamps: std::sync::Mutex::new(HashMap::new()),
        }
    }

    /// Registers a new user with enhanced validation and returns authentication token
    pub async fn register(&self, new_user: NewUser) -> Result<(User, String)> {
        // Validate user input with security focus
        new_user.validate().map_err(|e| anyhow!(e))?;

        // Check if username or email already exists
        if let Some(_) = self.user_repo.find_by_email(&new_user.email).await? {
            return Err(anyhow!("Email already registered"));
        }

        // Hash password with strong security
        let password_hash = password::hash_password(&new_user.password)?;

        // Create user with enhanced fields
        let mut user = User {
            id: Uuid::new_v4(),
            username: new_user.username,
            email: new_user.email,
            password_hash,
            created_at: now_utc(),
            updated_at: now_utc(),
            display_name: new_user.display_name,
            bio: None,
            avatar_url: None,
            friends: Vec::new(),
            followers: Vec::new(),
        };

        self.user_repo.create(&mut user).await?;

        // Generate JWT token
        let token = self.generate_token(user.id)?;

        Ok((user, token))
    }

    /// Authenticates a user with enhanced security checks
    pub async fn login(&self, email: &str, password: &str) -> Result<(User, String)> {
        // Find user by email
        let user = self
            .user_repo
            .find_by_email(email)
            .await?
            .ok_or_else(|| anyhow!("Invalid credentials"))?;

        // Verify password with timing attack protection
        if !password::verify_password(password, &user.password_hash)? {
            // Add artificial delay to prevent timing attacks
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            return Err(anyhow!("Invalid credentials"));
        }

        // Generate JWT token
        let token = self.generate_token(user.id)?;

        Ok((user, token))
    }

    /// Generates a JWT token with enhanced security
    fn generate_token(&self, user_id: Uuid) -> Result<String> {
        let now = now_utc();
        let expiration = (now + Duration::days(30)).timestamp() as usize;
        let issued_at = now.timestamp() as usize;

        let claims = Claims {
            sub: user_id,
            exp: expiration,
            iat: issued_at,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )
        .map_err(|e| anyhow!("Failed to generate token: {}", e))
    }

    /// Validates a JWT token with enhanced security checks
    pub fn validate_token(&self, token: &str) -> Result<Uuid> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;
        validation.validate_nbf = false;

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_ref()),
            &validation,
        )?;

        Ok(token_data.claims.sub)
    }

    /// Creates or updates a user profile with privacy controls
    pub async fn update_profile(
        &self,
        user_id: Uuid,
        display_name: Option<String>,
        bio: Option<String>,
        avatar_url: Option<String>,
        privacy_settings: Option<PrivacySettings>,
    ) -> Result<UserProfile> {
        let mut user = self
            .user_repo
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| anyhow!("User not found"))?;

        // Update user fields with validation
        if let Some(name) = display_name {
            if name.len() > 50 {
                return Err(anyhow!("Display name must be 50 characters or less"));
            }
            user.display_name = Some(name);
        }

        if let Some(bio_text) = bio {
            if bio_text.len() > 500 {
                return Err(anyhow!("Bio must be 500 characters or less"));
            }
            user.bio = Some(bio_text);
        }

        if let Some(avatar) = avatar_url {
            // Basic URL validation
            if !avatar.starts_with("http://") && !avatar.starts_with("https://") {
                return Err(anyhow!("Invalid avatar URL format"));
            }
            user.avatar_url = Some(avatar);
        }

        user.updated_at = now_utc();
        self.user_repo.update(&user).await?;

        // Create or get existing profile
        let profile = UserProfile {
            user_id,
            display_name: user.display_name.unwrap_or(user.username),
            bio: user.bio,
            avatar_url: user.avatar_url,
            cooperative_score: CooperativeScore::default(), // Will be calculated separately
            created_at: user.created_at,
            updated_at: user.updated_at,
        };

        Ok(profile)
    }

    /// Records a contribution with fraud detection (ported from Android ContributionService)
    pub async fn record_contribution(
        &self,
        user_id: Uuid,
        contribution_type: String,
        description: String,
        points: i32,
    ) -> Result<ContributionRecord> {
        // Fraud detection checks
        self.detect_contribution_fraud(user_id, &description, points).await?;

        let contribution = ContributionRecord {
            id: Uuid::new_v4(),
            user_id,
            contribution_type,
            description,
            points,
            verified: false,
            verified_by: None,
            created_at: now_utc(),
            verified_at: None,
        };

        // In a full implementation, this would be saved to a contributions repository
        // For now, we'll return the contribution record
        Ok(contribution)
    }

    /// Calculates cooperative score based on contributions (ported from Android UBICalculator)
    pub async fn calculate_cooperative_score(
        &self,
        user_id: Uuid,
        contributions: Vec<ContributionRecord>,
    ) -> Result<CooperativeScore> {
        let mut score = CooperativeScore::new();

        // Calculate contribution factors
        let mut contribution_types: HashMap<String, (f64, i32)> = HashMap::new();

        for contribution in contributions.iter().filter(|c| c.verified) {
            let (total_points, count) = contribution_types
                .entry(contribution.contribution_type.clone())
                .or_insert((0.0, 0));
            *total_points += contribution.points as f64;
            *count += 1;
        }

        // Add contribution factors with weights
        for (contribution_type, (total_points, count)) in contribution_types {
            let weight = self.get_contribution_weight(&contribution_type);
            let average_points = total_points / count as f64;
            
            score.update_factor(
                contribution_type.clone(),
                weight,
                average_points,
                Some(format!("Average points from {} contributions", count)),
            );
        }

        // Add time-based factors
        let recent_activity = self.calculate_recent_activity_factor(user_id, &contributions).await?;
        score.update_factor(
            "recent_activity".to_string(),
            0.2,
            recent_activity,
            Some("Recent activity multiplier".to_string()),
        );

        // Add consistency factor
        let consistency = self.calculate_consistency_factor(&contributions);
        score.update_factor(
            "consistency".to_string(),
            0.15,
            consistency,
            Some("Contribution consistency over time".to_string()),
        );

        Ok(score)
    }

    /// Manages user relationships (following, blocking, muting)
    pub async fn manage_relationship(
        &self,
        user_id: Uuid,
        target_user_id: Uuid,
        relationship_type: UserRelationshipType,
        action: RelationshipAction,
    ) -> Result<Option<UserRelationship>> {
        if user_id == target_user_id {
            return Err(anyhow!("Cannot create relationship with yourself"));
        }

        // Verify target user exists
        self.user_repo
            .find_by_id(target_user_id)
            .await?
            .ok_or_else(|| anyhow!("Target user not found"))?;

        match action {
            RelationshipAction::Create => {
                let relationship = match relationship_type {
                    UserRelationshipType::Following => UserRelationship::follow(user_id, target_user_id),
                    UserRelationshipType::Blocked => UserRelationship::block(user_id, target_user_id),
                    UserRelationshipType::Muted => UserRelationship::mute(user_id, target_user_id),
                };
                
                // In a full implementation, this would be saved to a relationships repository
                Ok(Some(relationship))
            }
            RelationshipAction::Remove => {
                // In a full implementation, this would remove the relationship from the repository
                Ok(None)
            }
        }
    }

    /// Gets user profile with privacy controls applied
    pub async fn get_user_profile(
        &self,
        user_id: Uuid,
        requesting_user_id: Option<Uuid>,
    ) -> Result<Option<UserProfile>> {
        let user = match self.user_repo.find_by_id(user_id).await? {
            Some(user) => user,
            None => return Ok(None),
        };

        // Create profile
        let mut profile = UserProfile {
            user_id,
            display_name: user.display_name.unwrap_or(user.username),
            bio: user.bio,
            avatar_url: user.avatar_url,
            cooperative_score: CooperativeScore::default(), // Would be calculated from contributions
            created_at: user.created_at,
            updated_at: user.updated_at,
        };

        // Apply privacy controls
        if let Some(requesting_id) = requesting_user_id {
            if requesting_id != user_id {
                // In a full implementation, we would check privacy settings
                // For now, we'll apply basic privacy rules
                profile.bio = None; // Hide bio from other users by default
            }
        } else {
            // Anonymous access - hide sensitive information
            profile.bio = None;
            profile.cooperative_score = CooperativeScore::default();
        }

        Ok(Some(profile))
    }

    // Private helper methods

    /// Detects fraud in contribution submissions (ported from Android ContributionService)
    async fn detect_contribution_fraud(
        &self,
        user_id: Uuid,
        description: &str,
        points: i32,
    ) -> Result<()> {
        // Rate limiting: max 5 contributions per minute
        let now = now_utc();
        let mut timestamps = self.user_contribution_timestamps.lock().unwrap();
        let user_contributions = timestamps.entry(user_id).or_insert_with(Vec::new);
        
        // Remove old timestamps (older than 5 minutes)
        user_contributions.retain(|&timestamp| now - timestamp < Duration::minutes(5));
        
        let recent_count = user_contributions.len();
        if recent_count >= 5 {
            return Err(anyhow!("Too many contributions in a short time. Please try again later."));
        }
        
        user_contributions.push(now);

        // Pattern analysis for fraud detection
        let fraud_patterns = vec!["spam", "fake", "test", "xyz"];
        let lower_desc = description.to_lowercase();
        if fraud_patterns.iter().any(|pattern| lower_desc.contains(pattern)) {
            return Err(anyhow!("Contribution contains suspicious patterns"));
        }

        // Point validation
        if points <= 0 || points > 1000 {
            return Err(anyhow!("Invalid point value: {}. Points must be between 1 and 1000", points));
        }

        // Artificial delay to prevent rapid automated submissions
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        Ok(())
    }

    /// Gets the weight for a contribution type
    fn get_contribution_weight(&self, contribution_type: &str) -> f64 {
        match contribution_type {
            "post" => 0.3,
            "comment" => 0.2,
            "like" => 0.1,
            "share" => 0.15,
            "verification" => 0.5,
            "moderation" => 0.4,
            _ => 0.1,
        }
    }

    /// Calculates recent activity factor
    async fn calculate_recent_activity_factor(
        &self,
        _user_id: Uuid,
        contributions: &[ContributionRecord],
    ) -> Result<f64> {
        let now = now_utc();
        let recent_threshold = now - Duration::days(30);
        
        let recent_contributions = contributions
            .iter()
            .filter(|c| c.created_at > recent_threshold)
            .count();

        // Scale factor based on recent activity (0.0 to 2.0)
        let factor = (recent_contributions as f64 / 10.0).min(2.0);
        Ok(factor)
    }

    /// Calculates consistency factor based on contribution patterns
    fn calculate_consistency_factor(&self, contributions: &[ContributionRecord]) -> f64 {
        if contributions.len() < 2 {
            return 0.5; // Neutral for new users
        }

        // Calculate standard deviation of time intervals between contributions
        let mut intervals = Vec::new();
        for i in 1..contributions.len() {
            let interval = contributions[i].created_at - contributions[i-1].created_at;
            intervals.push(interval.num_days() as f64);
        }

        if intervals.is_empty() {
            return 0.5;
        }

        let mean = intervals.iter().sum::<f64>() / intervals.len() as f64;
        let variance = intervals
            .iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>() / intervals.len() as f64;
        let std_dev = variance.sqrt();

        // Lower standard deviation = higher consistency
        // Scale to 0.0-1.0 range
        let consistency = (1.0 / (1.0 + std_dev / 7.0)).max(0.0).min(1.0);
        consistency
    }
}

/// Actions that can be performed on user relationships
#[derive(Debug, Clone)]
pub enum RelationshipAction {
    Create,
    Remove,
}

impl Default for PrivacySettings {
    fn default() -> Self {
        Self {
            profile_visibility: ProfileVisibility::Public,
            show_cooperative_score: true,
            show_contribution_factors: false,
            allow_direct_messages: true,
            show_activity_status: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::user_repository::UserRepository;
    use async_trait::async_trait;
    use std::collections::HashMap;
    use std::sync::Mutex;

    // Mock repository for testing
    struct MockUserRepository {
        users: Mutex<HashMap<Uuid, User>>,
    }

    impl MockUserRepository {
        fn new() -> Self {
            Self {
                users: Mutex::new(HashMap::new()),
            }
        }
    }

    #[async_trait]
    impl UserRepository for MockUserRepository {
        async fn create(&self, user: &mut User) -> Result<()> {
            let mut users = self.users.lock().unwrap();
            users.insert(user.id, user.clone());
            Ok(())
        }

        async fn find_by_id(&self, user_id: Uuid) -> Result<Option<User>> {
            let users = self.users.lock().unwrap();
            Ok(users.get(&user_id).cloned())
        }

        async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
            let users = self.users.lock().unwrap();
            Ok(users.values().find(|u| u.email == email).cloned())
        }

        async fn update(&self, user: &User) -> Result<()> {
            let mut users = self.users.lock().unwrap();
            users.insert(user.id, user.clone());
            Ok(())
        }

        async fn delete(&self, user_id: Uuid) -> Result<()> {
            let mut users = self.users.lock().unwrap();
            users.remove(&user_id);
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_user_registration() {
        let repo = Box::new(MockUserRepository::new());
        let service = IdentityService::new(repo, "test_secret".to_string());

        let new_user = NewUser {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            display_name: Some("Test User".to_string()),
        };

        let result = service.register(new_user).await;
        assert!(result.is_ok());

        let (user, token) = result.unwrap();
        assert_eq!(user.username, "testuser");
        assert_eq!(user.email, "test@example.com");
        assert!(!token.is_empty());
    }

    #[tokio::test]
    async fn test_contribution_fraud_detection() {
        let repo = Box::new(MockUserRepository::new());
        let service = IdentityService::new(repo, "test_secret".to_string());
        let user_id = Uuid::new_v4();

        // Test invalid points
        let result = service.record_contribution(
            user_id,
            "test".to_string(),
            "Valid description".to_string(),
            1001, // Too many points
        ).await;
        assert!(result.is_err());

        // Test fraud pattern detection
        let result = service.record_contribution(
            user_id,
            "test".to_string(),
            "This is spam content".to_string(),
            10,
        ).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_cooperative_score_calculation() {
        let repo = Box::new(MockUserRepository::new());
        let service = IdentityService::new(repo, "test_secret".to_string());
        let user_id = Uuid::new_v4();

        let contributions = vec![
            ContributionRecord {
                id: Uuid::new_v4(),
                user_id,
                contribution_type: "post".to_string(),
                description: "Great post".to_string(),
                points: 10,
                verified: true,
                verified_by: Some(Uuid::new_v4()),
                created_at: now_utc(),
                verified_at: Some(now_utc()),
            },
            ContributionRecord {
                id: Uuid::new_v4(),
                user_id,
                contribution_type: "comment".to_string(),
                description: "Helpful comment".to_string(),
                points: 5,
                verified: true,
                verified_by: Some(Uuid::new_v4()),
                created_at: now_utc(),
                verified_at: Some(now_utc()),
            },
        ];

        let result = service.calculate_cooperative_score(user_id, contributions).await;
        assert!(result.is_ok());

        let score = result.unwrap();
        assert!(score.value > 0.0);
        assert!(!score.contribution_factors.is_empty());
    }
}