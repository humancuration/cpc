use std::collections::{HashMap, VecDeque};
use std::time::{SystemTime, UNIX_EPOCH};
use lru::LruCache;
use serde::{Deserialize, Serialize};

use crate::models::Money;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryItem {
    pub id: String,
    pub product_id: String,
    pub title: String,
    pub description: String,
    pub price: Money,
    pub vendor_id: String,
    pub vendor_name: String,
    pub vendor_reputation: f64,
    pub video_url: String,
    pub thumbnail_url: String,
    pub created_at: u64,
    pub likes: u32,
    pub saves: u32,
    pub comments: u32,
    pub shares: u32,
    pub view_count: u32,
    pub tags: Vec<String>,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub liked_items: Vec<String>,
    pub saved_items: Vec<String>,
    pub viewed_items: Vec<String>,
    pub liked_vendors: Vec<String>,
    pub category_weights: HashMap<String, f64>,
    pub price_range: (Money, Money),
}

#[derive(Debug, Clone)]
pub struct ScoringWeights {
    pub user_preferences: f64,    // 40%
    pub social_engagement: f64,   // 30%
    pub vendor_reputation: f64,   // 20%
    pub recency: f64,            // 10%
}

impl Default for ScoringWeights {
    fn default() -> Self {
        Self {
            user_preferences: 0.40,
            social_engagement: 0.30,
            vendor_reputation: 0.20,
            recency: 0.10,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RecommendationEngine {
    weights: ScoringWeights,
    cache: LruCache<String, Vec<DiscoveryItem>>,
    user_preferences: HashMap<String, UserPreferences>,
}

impl RecommendationEngine {
    pub fn new(cache_size: usize) -> Self {
        Self {
            weights: ScoringWeights::default(),
            cache: LruCache::new(cache_size.try_into().unwrap()),
            user_preferences: HashMap::new(),
        }
    }
    
    pub fn set_weights(&mut self, weights: ScoringWeights) {
        self.weights = weights;
    }
    
    pub fn update_user_preferences(&mut self, user_id: &str, preferences: UserPreferences) {
        self.user_preferences.insert(user_id.to_string(), preferences);
    }
    
    pub fn get_recommendations(
        &mut self,
        user_id: &str,
        items: Vec<DiscoveryItem>,
        limit: usize,
    ) -> Vec<DiscoveryItem> {
        let cache_key = format!("{}:{}", user_id, items.len());
        
        // Check cache first
        if let Some(cached) = self.cache.get(&cache_key) {
            return cached.clone();
        }
        
        let preferences = self.user_preferences.get(user_id).cloned();
        
        // Score all items
        let mut scored_items: Vec<(DiscoveryItem, f64)> = items
            .into_iter()
            .map(|item| {
                let score = self.calculate_score(&item, &preferences);
                (item, score)
            })
            .collect();
        
        // Sort by score descending
        scored_items.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        // Take top items
        let recommendations: Vec<DiscoveryItem> = scored_items
            .into_iter()
            .take(limit)
            .map(|(item, _)| item)
            .collect();
        
        // Cache the result
        self.cache.put(cache_key, recommendations.clone());
        
        recommendations
    }
    
    fn calculate_score(
        &self,
        item: &DiscoveryItem,
        preferences: &Option<UserPreferences>,
    ) -> f64 {
        let mut score = 0.0;
        
        // User preferences score (40%)
        let user_pref_score = if let Some(pref) = preferences {
            self.calculate_user_preference_score(item, pref)
        } else {
            0.5 // Neutral score if no preferences
        };
        score += user_pref_score * self.weights.user_preferences;
        
        // Social engagement score (30%)
        let engagement_score = self.calculate_engagement_score(item);
        score += engagement_score * self.weights.social_engagement;
        
        // Vendor reputation score (20%)
        let vendor_score = item.vendor_reputation / 5.0; // Normalize to 0-1
        score += vendor_score * self.weights.vendor_reputation;
        
        // Recency score (10%)
        let recency_score = self.calculate_recency_score(item.created_at);
        score += recency_score * self.weights.recency;
        
        score
    }
    
    fn calculate_user_preference_score(
        &self,
        item: &DiscoveryItem,
        preferences: &UserPreferences,
    ) -> f64 {
        let mut score = 0.0;
        
        // Boost if user liked similar items
        if preferences.liked_items.contains(&item.id) {
            score += 0.1;
        }
        
        // Boost if user saved similar items
        if preferences.saved_items.contains(&item.id) {
            score += 0.15;
        }
        
        // Boost if user likes this vendor
        if preferences.liked_vendors.contains(&item.vendor_id) {
            score += 0.2;
        }
        
        // Category preference
        if let Some(weight) = preferences.category_weights.get(&item.category) {
            score += weight * 0.3;
        }
        
        // Price range preference
        let price = item.price.amount;
        let (min_price, max_price) = &preferences.price_range;
        
        if price >= min_price.amount && price <= max_price.amount {
            score += 0.15;
        } else {
            // Penalize items outside price range
            let distance = if price < min_price.amount {
                min_price.amount - price
            } else {
                price - max_price.amount
            };
            let penalty = (distance / max_price.amount).min(0.3);
            score -= penalty;
        }
        
        // Tag similarity
        let tag_matches = item.tags.iter()
            .filter(|tag| {
                preferences.category_weights.iter()
                    .any(|(cat, _)| cat.contains(*tag))
            })
            .count();
        
        score += (tag_matches as f64 * 0.05).min(0.2);
        
        score.max(0.0).min(1.0)
    }
    
    fn calculate_engagement_score(&self, item: &DiscoveryItem) -> f64 {
        let total_engagement = (item.likes + item.saves + item.comments + item.shares) as f64;
        let views = item.view_count as f64;
        
        if views == 0.0 {
            return 0.0;
        }
        
        // Engagement rate
        let engagement_rate = total_engagement / views;
        
        // Logarithmic scaling to prevent dominance by viral items
        let log_engagement = (total_engagement + 1.0).ln();
        let log_views = (views + 1.0).ln();
        
        let engagement_score = (log_engagement / log_views.max(1.0)).min(1.0);
        
        // Combine engagement rate and volume
        engagement_rate * 0.7 + engagement_score * 0.3
    }
    
    fn calculate_recency_score(&self, created_at: u64) -> f64 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let age_seconds = now - created_at;
        let age_days = age_seconds as f64 / 86400.0;
        
        // Exponential decay based on age
        let decay_factor = (-age_days / 7.0).exp(); // Half-life of 7 days
        
        decay_factor.max(0.0).min(1.0)
    }
    
    pub fn invalidate_cache(&mut self, user_id: &str) {
        let keys_to_remove: Vec<String> = self.cache
            .iter()
            .filter_map(|(key, _)| {
                if key.starts_with(user_id) {
                    Some(key.clone())
                } else {
                    None
                }
            })
            .collect();
        
        for key in keys_to_remove {
            self.cache.pop(&key);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationRequest {
    pub user_id: String,
    pub limit: usize,
    pub offset: usize,
    pub filters: Option<RecommendationFilters>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationFilters {
    pub categories: Option<Vec<String>>,
    pub price_range: Option<(Money, Money)>,
    pub vendors: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_engagement_score_calculation() {
        let engine = RecommendationEngine::new(100);
        
        let item = DiscoveryItem {
            id: "1".to_string(),
            product_id: "p1".to_string(),
            title: "Test".to_string(),
            description: "Test".to_string(),
            price: Money { amount: 10.0, currency: "USD".to_string() },
            vendor_id: "v1".to_string(),
            vendor_name: "Test".to_string(),
            vendor_reputation: 4.5,
            video_url: "test.mp4".to_string(),
            thumbnail_url: "test.jpg".to_string(),
            created_at: 1609459200,
            likes: 100,
            saves: 50,
            comments: 25,
            shares: 10,
            view_count: 1000,
            tags: vec!["test".to_string()],
            category: "test".to_string(),
        };
        
        let score = engine.calculate_engagement_score(&item);
        assert!(score > 0.0 && score <= 1.0);
    }
    
    #[test]
    fn test_recency_score_calculation() {
        let engine = RecommendationEngine::new(100);
        
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let recent_item = DiscoveryItem {
            created_at: now - 86400, // 1 day ago
            ..Default::default()
        };
        
        let old_item = DiscoveryItem {
            created_at: now - 86400 * 30, // 30 days ago
            ..Default::default()
        };
        
        let recent_score = engine.calculate_recency_score(recent_item.created_at);
        let old_score = engine.calculate_recency_score(old_item.created_at);
        
        assert!(recent_score > old_score);
        assert!(recent_score > 0.8);
        assert!(old_score < 0.5);
    }
}

impl Default for DiscoveryItem {
    fn default() -> Self {
        Self {
            id: String::new(),
            product_id: String::new(),
            title: String::new(),
            description: String::new(),
            price: Money { amount: 0.0, currency: "USD".to_string() },
            vendor_id: String::new(),
            vendor_name: String::new(),
            vendor_reputation: 0.0,
            video_url: String::new(),
            thumbnail_url: String::new(),
            created_at: 0,
            likes: 0,
            saves: 0,
            comments: 0,
            shares: 0,
            view_count: 0,
            tags: Vec::new(),
            category: String::new(),
        }
    }
}