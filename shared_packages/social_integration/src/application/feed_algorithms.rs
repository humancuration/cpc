//! Feed algorithms for generating personalized feeds

use crate::domain::post::{UnifiedPost, AppSource};
use uuid::Uuid;

/// Trait for feed generation algorithms
pub trait FeedAlgorithm: Send + Sync {
    /// Generate a feed from a collection of posts for a specific user
    fn generate_feed(
        &self, 
        posts: Vec<UnifiedPost>, 
        user_id: Uuid
    ) -> Vec<UnifiedPost>;
}

/// Chronological feed algorithm - newest posts first
pub struct ChronologicalFeedAlgorithm;
impl FeedAlgorithm for ChronologicalFeedAlgorithm {
    fn generate_feed(&self, mut posts: Vec<UnifiedPost>, _user_id: Uuid) -> Vec<UnifiedPost> {
        posts.sort_by(|a, b| b.metadata.created_at.cmp(&a.metadata.created_at));
        posts
    }
}

/// Engagement-based feed algorithm - most engaged posts first
pub struct EngagementFeedAlgorithm;
impl FeedAlgorithm for EngagementFeedAlgorithm {
    fn generate_feed(&self, mut posts: Vec<UnifiedPost>, user_id: Uuid) -> Vec<UnifiedPost> {
        posts.sort_by(|a, b| {
            let a_score = calculate_post_score(a, user_id);
            let b_score = calculate_post_score(b, user_id);
            b_score.cmp(&a_score)
        });
        posts
    }
}

/// Calculate engagement score for a post with social context
fn calculate_post_score(post: &UnifiedPost, user_id: Uuid) -> i32 {
    let mut score = post.metadata.engagement.upvotes +
                   post.metadata.engagement.comments +
                   post.metadata.engagement.shares;
    
    // Boost opportunities from followed organizations
    if post.source == AppSource::Volunteering &&
       is_followed_organization(post.author_id, user_id) {
        score *= 2;
    }
    
    // Boost activities from connections
    if is_connection(post.author_id, user_id) {
        score = (score as f32 * 1.5) as i32;
    }
    
    score
}

/// Check if an organization is followed by the user
/// This is a placeholder for the actual implementation
fn is_followed_organization(organization_id: Uuid, user_id: Uuid) -> bool {
    // TODO: Implement actual follow relationship check
    // For now, return false to maintain backward compatibility
    false
}

/// Check if a user is a connection
/// This is a placeholder for the actual implementation
fn is_connection(user_id: Uuid, target_user_id: Uuid) -> bool {
    // TODO: Implement actual connection relationship check
    // For now, return false to maintain backward compatibility
    false
}

/// Custom feed algorithm that can be extended
pub struct CustomFeedAlgorithm {
    algorithm_name: String,
}

impl CustomFeedAlgorithm {
    pub fn new(algorithm_name: String) -> Self {
        Self { algorithm_name }
    }
}

impl FeedAlgorithm for CustomFeedAlgorithm {
    fn generate_feed(&self, posts: Vec<UnifiedPost>, user_id: Uuid) -> Vec<UnifiedPost> {
        // Default implementation falls back to chronological
        // This can be overridden by specific implementations
        let chronological = ChronologicalFeedAlgorithm;
        chronological.generate_feed(posts, user_id)
    }
}

/// Feed algorithm registry for managing available algorithms
pub struct FeedAlgorithmRegistry {
    algorithms: std::collections::HashMap<String, Box<dyn FeedAlgorithm + Send + Sync>>,
}

impl FeedAlgorithmRegistry {
    pub fn new() -> Self {
        let mut algorithms = std::collections::HashMap::new();
        algorithms.insert("chronological".to_string(), Box::new(ChronologicalFeedAlgorithm));
        algorithms.insert("engagement".to_string(), Box::new(EngagementFeedAlgorithm));
        
        Self { algorithms }
    }
    
    pub fn add_algorithm(
        &mut self,
        name: String,
        algorithm: Box<dyn FeedAlgorithm + Send + Sync>
    ) {
        self.algorithms.insert(name, algorithm);
    }
    
    pub fn get_algorithm(&self, name: &str) -> Option<&Box<dyn FeedAlgorithm + Send + Sync>> {
        self.algorithms.get(name)
    }
}