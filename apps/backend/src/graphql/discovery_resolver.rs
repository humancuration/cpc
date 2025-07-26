use async_graphql::*;
use std::sync::Arc;
use tokio::sync::RwLock;

use cpc_core::recommendation::{DiscoveryItem, RecommendationEngine, UserPreferences, ScoringWeights};
use crate::AppState;

pub struct DiscoveryService {
    pub state: Arc<RwLock<AppState>>,
    pub engine: Arc<RwLock<RecommendationEngine>>,
}

impl DiscoveryService {
    pub fn new(state: Arc<RwLock<AppState>>) -> Self {
        let engine = Arc::new(RwLock::new(RecommendationEngine::new(1000)));
        Self { state, engine }
    }
}

#[Object]
impl DiscoveryService {
    async fn discovery_feed(
        &self,
        ctx: &Context<'_>,
        user_id: String,
        limit: Option<i32>,
        cursor: Option<String>,
        categories: Option<Vec<String>>,
        price_range: Option<PriceRangeInput>,
        vendors: Option<Vec<String>>,
    ) -> Result<DiscoveryFeed> {
        let state = self.state.read().await;
        let mut engine = self.engine.write().await;
        
        // Convert limit to usize
        let limit = limit.unwrap_or(20) as usize;
        
        // Fetch discovery items from database
        let items = self.fetch_discovery_items(&state, &categories, &price_range, &vendors).await?;
        
        // Get user preferences
        if let Some(preferences) = self.fetch_user_preferences(&state, &user_id).await {
            engine.update_user_preferences(&user_id, preferences);
        }
        
        // Get recommendations using the engine
        let recommendations = engine.get_recommendations(
            &user_id,
            items,
            limit + 1, // Get one extra to determine has_more
        );
        
        // Determine pagination
        let has_more = recommendations.len() > limit;
        let items = recommendations.into_iter().take(limit).collect();
        
        Ok(DiscoveryFeed {
            items,
            has_more,
            next_cursor: has_more.then(|| items.last().map(|item| item.id.clone()).unwrap_or_default()),
        })
    }
    
    async fn discovery_item(&self, ctx: &Context<'_>, id: String) -> Result<Option<DiscoveryItem>> {
        let state = self.state.read().await;
        self.fetch_discovery_item_by_id(&state, &id).await
    }
    
    async fn trending_items(&self, ctx: &Context<'_>, limit: Option<i32>) -> Result<Vec<DiscoveryItem>> {
        let state = self.state.read().await;
        let limit = limit.unwrap_or(10) as usize;
        
        let mut items = self.fetch_all_discovery_items(&state).await?;
        
        // Sort by engagement score
        items.sort_by(|a, b| {
            let score_a = self.calculate_engagement_score(a);
            let score_b = self.calculate_engagement_score(b);
            score_b.partial_cmp(&score_a).unwrap()
        });
        
        Ok(items.into_iter().take(limit).collect())
    }
    
    async fn like_discovery_item(&self, ctx: &Context<'_>, item_id: String, user_id: String) -> Result<DiscoveryEngagement> {
        let mut state = self.state.write().await;
        
        let engagement = self.update_engagement(&mut state, &item_id, &user_id, "like", true).await?;
        
        // Invalidate cache
        let mut engine = self.engine.write().await;
        engine.invalidate_cache(&user_id);
        
        Ok(engagement)
    }
    
    async fn unlike_discovery_item(&self, ctx: &Context<'_>, item_id: String, user_id: String) -> Result<DiscoveryEngagement> {
        let mut state = self.state.write().await;
        
        let engagement = self.update_engagement(&mut state, &item_id, &user_id, "like", false).await?;
        
        let mut engine = self.engine.write().await;
        engine.invalidate_cache(&user_id);
        
        Ok(engagement)
    }
    
    async fn save_discovery_item(&self, ctx: &Context<'_>, item_id: String, user_id: String) -> Result<DiscoveryEngagement> {
        let mut state = self.state.write().await;
        
        let engagement = self.update_engagement(&mut state, &item_id, &user_id, "save", true).await?;
        
        let mut engine = self.engine.write().await;
        engine.invalidate_cache(&user_id);
        
        Ok(engagement)
    }
    
    async fn unsave_discovery_item(&self, ctx: &Context<'_>, item_id: String, user_id: String) -> Result<DiscoveryEngagement> {
        let mut state = self.state.write().await;
        
        let engagement = self.update_engagement(&mut state, &item_id, &user_id, "save", false).await?;
        
        let mut engine = self.engine.write().await;
        engine.invalidate_cache(&user_id);
        
        Ok(engagement)
    }
    
    async fn increment_view_count(&self, ctx: &Context<'_>, item_id: String) -> Result<bool> {
        let mut state = self.state.write().await;
        self.increment_view_count_in_db(&mut state, &item_id).await?;
        Ok(true)
    }
    
    async fn share_discovery_item(&self, ctx: &Context<'_>, item_id: String, user_id: String) -> Result<DiscoveryEngagement> {
        let mut state = self.state.write().await;
        
        let engagement = self.update_engagement(&mut state, &item_id, &user_id, "share", true).await?;
        
        let mut engine = self.engine.write().await;
        engine.invalidate_cache(&user_id);
        
        Ok(engagement)
    }
    
    async fn create_discovery_item(&self, ctx: &Context<'_>, input: CreateDiscoveryItemInput) -> Result<DiscoveryItem> {
        let mut state = self.state.write().await;
        self.create_discovery_item_in_db(&mut state, input).await
    }
    
    async fn update_discovery_item(&self, ctx: &Context<'_>, id: String, input: UpdateDiscoveryItemInput) -> Result<DiscoveryItem> {
        let mut state = self.state.write().await;
        self.update_discovery_item_in_db(&mut state, &id, input).await
    }
}

// Type definitions for GraphQL
#[derive(SimpleObject, Clone)]
pub struct DiscoveryFeed {
    pub items: Vec<DiscoveryItem>,
    pub has_more: bool,
    pub next_cursor: Option<String>,
}

#[derive(SimpleObject, Clone)]
pub struct DiscoveryEngagement {
    pub item_id: String,
    pub likes: u32,
    pub saves: u32,
    pub comments: u32,
    pub shares: u32,
    pub is_liked: bool,
    pub is_saved: bool,
}

#[derive(InputObject, Clone)]
pub struct PriceRangeInput {
    pub min: f64,
    pub max: f64,
}

#[derive(InputObject, Clone)]
pub struct CreateDiscoveryItemInput {
    pub product_id: String,
    pub title: String,
    pub description: String,
    pub price: MoneyInput,
    pub vendor_id: String,
    pub video_url: String,
    pub thumbnail_url: String,
    pub tags: Vec<String>,
    pub category: String,
}

#[derive(InputObject, Clone)]
pub struct UpdateDiscoveryItemInput {
    pub title: Option<String>,
    pub description: Option<String>,
    pub price: Option<MoneyInput>,
    pub video_url: Option<String>,
    pub thumbnail_url: Option<String>,
    pub tags: Option<Vec<String>>,
    pub category: Option<String>,
}

#[derive(InputObject, Clone)]
pub struct MoneyInput {
    pub amount: f64,
    pub currency: String,
}

impl DiscoveryService {
    // Database interaction methods
    async fn fetch_discovery_items(
        &self,
        state: &AppState,
        categories: &Option<Vec<String>>,
        price_range: &Option<PriceRangeInput>,
        vendors: &Option<Vec<String>>,
    ) -> Result<Vec<DiscoveryItem>, Error> {
        // TODO: Implement actual database query
        // This is a mock implementation
        Ok(vec![
            DiscoveryItem {
                id: "1".to_string(),
                product_id: "p1".to_string(),
                title: "Handcrafted Ceramic Mug".to_string(),
                description: "Beautiful handcrafted ceramic mug made by local artisans".to_string(),
                price: cpc_core::models::Money { amount: 25.0, currency: "USD".to_string() },
                vendor_id: "v1".to_string(),
                vendor_name: "Local Artisans Co-op".to_string(),
                vendor_reputation: 4.8,
                video_url: "https://videos.cpc.local/mug-demo.webm".to_string(),
                thumbnail_url: "https://images.cpc.local/mug-thumb.jpg".to_string(),
                created_at: 1609459200,
                likes: 142,
                saves: 89,
                comments: 23,
                shares: 45,
                view_count: 1247,
                tags: vec!["ceramic".to_string(), "handmade".to_string(), "mug".to_string()],
                category: "Home & Kitchen".to_string(),
            },
            DiscoveryItem {
                id: "2".to_string(),
                product_id: "p2".to_string(),
                title: "Organic Cotton T-Shirt".to_string(),
                description: "Sustainably sourced organic cotton t-shirt from worker-owned coop".to_string(),
                price: cpc_core::models::Money { amount: 35.0, currency: "USD".to_string() },
                vendor_id: "v2".to_string(),
                vendor_name: "Sustainable Threads Co-op".to_string(),
                vendor_reputation: 4.9,
                video_url: "https://videos.cpc.local/tshirt-demo.webm".to_string(),
                thumbnail_url: "https://images.cpc.local/tshirt-thumb.jpg".to_string(),
                created_at: 1609459200,
                likes: 267,
                saves: 134,
                comments: 42,
                shares: 89,
                view_count: 2341,
                tags: vec!["organic".to_string(), "cotton".to_string(), "tshirt".to_string()],
                category: "Clothing".to_string(),
            },
        ])
    }
    
    async fn fetch_all_discovery_items(&self, state: &AppState) -> Result<Vec<DiscoveryItem>, Error> {
        self.fetch_discovery_items(&state, &None, &None, &None).await
    }
    
    async fn fetch_user_preferences(
        &self,
        state: &AppState,
        user_id: &str,
    ) -> Option<UserPreferences> {
        // TODO: Implement actual database query
        Some(UserPreferences {
            liked_items: vec!["1".to_string()],
            saved_items: vec![],
            viewed_items: vec![],
            liked_vendors: vec!["v1".to_string()],
            category_weights: {
                let mut map = std::collections::HashMap::new();
                map.insert("Home & Kitchen".to_string(), 0.8);
                map.insert("Clothing".to_string(), 0.6);
                map
            },
            price_range: (
                cpc_core::models::Money { amount: 0.0, currency: "USD".to_string() },
                cpc_core::models::Money { amount: 100.0, currency: "USD".to_string() },
            ),
        })
    }
    
    async fn fetch_discovery_item_by_id(
        &self,
        state: &AppState,
        id: &str,
    ) -> Result<Option<DiscoveryItem>, Error> {
        // TODO: Implement actual database query
        Ok(Some(DiscoveryItem {
            id: id.to_string(),
            product_id: "p1".to_string(),
            title: "Handcrafted Ceramic Mug".to_string(),
            description: "Beautiful handcrafted ceramic mug made by local artisans".to_string(),
            price: cpc_core::models::Money { amount: 25.0, currency: "USD".to_string() },
            vendor_id: "v1".to_string(),
            vendor_name: "Local Artisans Co-op".to_string(),
            vendor_reputation: 4.8,
            video_url: "https://videos.cpc.local/mug-demo.webm".to_string(),
            thumbnail_url: "https://images.cpc.local/mug-thumb.jpg".to_string(),
            created_at: 1609459200,
            likes: 142,
            saves: 89,
            comments: 23,
            shares: 45,
            view_count: 1247,
            tags: vec!["ceramic".to_string(), "handmade".to_string(), "mug".to_string()],
            category: "Home & Kitchen".to_string(),
        }))
    }
    
    fn calculate_engagement_score(&self, item: &DiscoveryItem) -> f64 {
        let total_engagement = (item.likes + item.saves + item.comments + item.shares) as f64;
        let views = item.view_count as f64;
        
        if views == 0.0 {
            return 0.0;
        }
        
        let engagement_rate = total_engagement / views;
        let log_engagement = (total_engagement + 1.0).ln();
        let log_views = (views + 1.0).ln();
        
        let engagement_score = (log_engagement / log_views.max(1.0)).min(1.0);
        
        engagement_rate * 0.7 + engagement_score * 0.3
    }
    
    async fn update_engagement(
        &self,
        state: &mut AppState,
        item_id: &str,
        user_id: &str,
        action: &str,
        value: bool,
    ) -> Result<DiscoveryEngagement, Error> {
        // TODO: Implement actual database update
        Ok(DiscoveryEngagement {
            item_id: item_id.to_string(),
            likes: 143,
            saves: 90,
            comments: 23,
            shares: 45,
            is_liked: action == "like" && value,
            is_saved: action == "save" && value,
        })
    }
    
    async fn increment_view_count_in_db(
        &self,
        state: &mut AppState,
        item_id: &str,
    ) -> Result<(), Error> {
        // TODO: Implement actual database update
        Ok(())
    }
    
    async fn create_discovery_item_in_db(
        &self,
        state: &mut AppState,
        input: CreateDiscoveryItemInput,
    ) -> Result<DiscoveryItem, Error> {
        // TODO: Implement actual database insert
        Ok(DiscoveryItem {
            id: "3".to_string(),
            product_id: input.product_id,
            title: input.title,
            description: input.description,
            price: cpc_core::models::Money {
                amount: input.price.amount,
                currency: input.price.currency,
            },
            vendor_id: input.vendor_id,
            vendor_name: "New Vendor".to_string(),
            vendor_reputation: 4.5,
            video_url: input.video_url,
            thumbnail_url: input.thumbnail_url,
            created_at: 1609459200,
            likes: 0,
            saves: 0,
            comments: 0,
            shares: 0,
            view_count: 0,
            tags: input.tags,
            category: input.category,
        })
    }
    
    async fn update_discovery_item_in_db(
        &self,
        state: &mut AppState,
        id: &str,
        input: UpdateDiscoveryItemInput,
    ) -> Result<DiscoveryItem, Error> {
        // TODO: Implement actual database update
        self.fetch_discovery_item_by_id(state, id).await?
            .ok_or_else(|| Error::new("Item not found"))
    }
}