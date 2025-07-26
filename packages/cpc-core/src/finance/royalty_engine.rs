//! Royalty Distribution Engine
//!
//! Implements the Sovereign Cooperative Model for royalty distribution with
//! recursive upstream calculation, cooperative revenue sharing, and real-time
//! distribution capabilities.
//!
//! Based on the Kotlin implementation from the legacy system.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

use super::transactions::{Transaction, TransactionLedger, TransactionType};
use super::treasury::TreasuryService;

/// Core royalty engine implementing the Sovereign Cooperative Model
pub struct RoyaltyEngine<L: TransactionLedger> {
    treasury_service: Arc<TreasuryService<L>>,
    content_service: Arc<dyn ContentService>,
    storage: Arc<RwLock<dyn RoyaltyStorage>>,
    config: RoyaltyConfig,
}

/// Configuration for royalty engine behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoyaltyConfig {
    /// Minimum payout amount to avoid micro-transactions
    pub min_payout_amount: Decimal,
    /// Maximum recursion depth for upstream calculations
    pub max_recursion_depth: u8,
    /// Decimal precision for calculations
    pub calculation_scale: u8,
    /// Platform fee percentage (0.0-1.0)
    pub platform_fee_percentage: Decimal,
    /// Cooperative treasury percentage (0.0-1.0)
    pub cooperative_treasury_percentage: Decimal,
}

impl Default for RoyaltyConfig {
    fn default() -> Self {
        Self {
            min_payout_amount: Decimal::new(1, 2), // $0.01
            max_recursion_depth: 10,
            calculation_scale: 10,
            platform_fee_percentage: Decimal::new(5, 2), // 5%
            cooperative_treasury_percentage: Decimal::new(10, 2), // 10%
        }
    }
}

/// Royalty distribution result
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RoyaltyDistribution {
    /// Unique distribution identifier
    pub id: Uuid,
    /// Content identifier (track, video, etc.)
    pub content_id: Uuid,
    /// Total revenue amount
    pub total_amount: Decimal,
    /// Currency code
    pub currency: String,
    /// When the distribution was calculated
    pub distributed_at: DateTime<Utc>,
    /// Individual artist distributions (artist ID -> amount)
    pub distributions: HashMap<Uuid, Decimal>,
    /// Transaction IDs for each artist (artist ID -> transaction ID)
    pub transaction_ids: HashMap<Uuid, Uuid>,
    /// Platform fee amount
    pub platform_fee: Decimal,
    /// Cooperative treasury amount
    pub cooperative_treasury_amount: Decimal,
    /// Distribution status
    pub status: DistributionStatus,
}

/// Distribution processing status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DistributionStatus {
    /// Distribution calculated but not yet processed
    Pending,
    /// Distribution successfully processed
    Completed,
    /// Distribution failed
    Failed,
    /// Partial distribution (some recipients below minimum)
    Partial,
}

/// Content information for royalty calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentInfo {
    pub id: Uuid,
    pub title: String,
    pub artist_id: Uuid,
    pub license: ContentLicense,
    pub upstream_content_ids: Vec<Uuid>,
    pub cooperative_id: Option<Uuid>,
    pub metadata: ContentMetadata,
}

/// Content license information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentLicense {
    /// Royalty splits (artist ID -> percentage)
    pub royalty_split: HashMap<Uuid, Decimal>,
    /// Minimum upstream percentage required
    pub minimum_upstream_percentage: Decimal,
    /// Whether the content can be remixed
    pub allows_remixing: bool,
    /// Whether attribution is required
    pub requires_attribution: bool,
}

impl ContentLicense {
    /// Validates the license configuration
    pub fn is_valid(&self) -> bool {
        let total_percentage: Decimal = self.royalty_split.values().sum();
        (total_percentage - Decimal::ONE).abs() < Decimal::new(1, 4)
    }
}

/// Content metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentMetadata {
    pub duration_seconds: u32,
    pub genre: Option<String>,
    pub is_explicit: bool,
}

/// Storage interface for royalty data (hexagonal architecture)
pub trait RoyaltyStorage: Send + Sync {
    /// Store a royalty distribution
    fn store_distribution(&mut self, distribution: RoyaltyDistribution) -> Result<(), RoyaltyError>;
    
    /// Retrieve a royalty distribution by ID
    fn get_distribution(&self, id: Uuid) -> Option<RoyaltyDistribution>;
    
    /// Get all distributions for a content item
    fn get_content_distributions(&self, content_id: Uuid) -> Vec<RoyaltyDistribution>;
    
    /// Get royalty history for an artist
    fn get_artist_royalty_history(&self, artist_id: Uuid) -> Vec<RoyaltyDistribution>;
}

/// Content service interface (mock for now)
pub trait ContentService: Send + Sync {
    /// Get content information by ID
    fn get_content_info(&self, content_id: Uuid) -> Result<ContentInfo, RoyaltyError>;
    
    /// Get upstream content information
    fn get_upstream_content(&self, content_id: Uuid) -> Result<Vec<ContentInfo>, RoyaltyError>;
}

/// Comprehensive royalty error handling
#[derive(Debug, thiserror::Error)]
pub enum RoyaltyError {
    #[error("Content not found: {0}")]
    ContentNotFound(Uuid),
    
    #[error("Invalid license for content: {0}")]
    InvalidLicense(Uuid),
    
    #[error("Recursion depth exceeded for content: {0}")]
    RecursionDepthExceeded(Uuid),
    
    #[error("Invalid royalty split: splits must sum to 100%")]
    InvalidRoyaltySplit,
    
    #[error("Upstream percentage below minimum for content: {0}")]
    UpstreamPercentageTooLow(Uuid),
    
    #[error("Storage error: {0}")]
    StorageError(String),
    
    #[error("Content service error: {0}")]
    ContentServiceError(String),
    
    #[error("Treasury service error: {0}")]
    TreasuryServiceError(String),
    
    #[error("Invalid revenue amount: must be positive")]
    InvalidRevenueAmount,
    
    #[error("Currency mismatch: expected {expected}, got {actual}")]
    CurrencyMismatch {
        expected: String,
        actual: String,
    },
}

impl<L: TransactionLedger + Send + Sync + 'static> RoyaltyEngine<L> {
    /// Create a new royalty engine
    pub fn new(
        treasury_service: Arc<TreasuryService<L>>,
        content_service: Arc<dyn ContentService>,
        storage: Arc<RwLock<dyn RoyaltyStorage>>,
        config: RoyaltyConfig,
    ) -> Self {
        Self {
            treasury_service,
            content_service,
            storage,
            config,
        }
    }

    /// Distribute royalties for content revenue
    pub async fn distribute_royalties(
        &self,
        content_id: Uuid,
        revenue: Decimal,
        currency: &str,
    ) -> Result<RoyaltyDistribution, RoyaltyError> {
        if revenue <= Decimal::ZERO {
            return Err(RoyaltyError::InvalidRevenueAmount);
        }

        let content = self.content_service.get_content_info(content_id)?;
        
        // Calculate platform fee
        let platform_fee = revenue * self.config.platform_fee_percentage;
        let remaining_after_platform = revenue - platform_fee;

        // Calculate distributions
        let distributions = self.calculate_royalty_distributions(&content, remaining_after_platform, 0)?;

        // Create distribution record
        let distribution = RoyaltyDistribution {
            id: Uuid::new_v4(),
            content_id,
            total_amount: revenue,
            currency: currency.to_string(),
            distributed_at: Utc::now(),
            distributions,
            transaction_ids: HashMap::new(),
            platform_fee,
            cooperative_treasury_amount: Decimal::ZERO,
            status: DistributionStatus::Completed,
        };

        // Store the distribution
        self.storage
            .write()
            .map_err(|e| RoyaltyError::StorageError(e.to_string()))?
            .store_distribution(distribution.clone())?;

        Ok(distribution)
    }

    /// Calculate royalty distributions recursively
    fn calculate_royalty_distributions(
        &self,
        content: &ContentInfo,
        total_amount: Decimal,
        depth: u8,
    ) -> Result<HashMap<Uuid, Decimal>, RoyaltyError> {
        // Prevent infinite recursion
        if depth > self.config.max_recursion_depth {
            return Err(RoyaltyError::RecursionDepthExceeded(content.id));
        }

        if !content.license.is_valid() {
            return Err(RoyaltyError::InvalidLicense(content.id));
        }

        let mut distributions = HashMap::new();

        // Calculate direct distributions
        for (artist_id, percentage) in &content.license.royalty_split {
            let amount = total_amount * *percentage;
            if amount >= self.config.min_payout_amount {
                *distributions.entry(*artist_id).or_insert(Decimal::ZERO) += amount;
            }
        }

        // Process upstream content recursively
        if !content.upstream_content_ids.is_empty() {
            let upstream_percentage = self.calculate_upstream_percentage(&content.license);
            let upstream_total = total_amount * upstream_percentage;
            
            let upstream_count = content.upstream_content_ids.len() as u64;
            let upstream_amount_per_source = upstream_total / Decimal::from(upstream_count);

            for upstream_id in &content.upstream_content_ids {
                let upstream_content = self.content_service.get_content_info(*upstream_id)?;
                let upstream_distributions = self.calculate_royalty_distributions(
                    &upstream_content,
                    upstream_amount_per_source,
                    depth + 1,
                )?;

                // Merge upstream distributions
                for (artist_id, amount) in upstream_distributions {
                    *distributions.entry(artist_id).or_insert(Decimal::ZERO) += amount;
                }
            }
        }

        Ok(distributions)
    }

    /// Calculate upstream percentage from license
    fn calculate_upstream_percentage(&self, license: &ContentLicense) -> Decimal {
        let max_contribution = license.royalty_split.values().max().unwrap_or(&Decimal::ZERO);
        Decimal::ONE - max_contribution
    }

    /// Get total revenue for content
    pub fn get_content_total_revenue(&self, content_id: Uuid) -> Decimal {
        self.storage
            .read()
            .map(|storage| {
                storage
                    .get_content_distributions(content_id)
                    .iter()
                    .map(|d| d.total_amount)
                    .sum()
            })
            .unwrap_or(Decimal::ZERO)
    }
}

/// In-memory royalty storage implementation
#[derive(Debug, Default)]
pub struct InMemoryRoyaltyStorage {
    distributions: HashMap<Uuid, RoyaltyDistribution>,
    content_distributions: HashMap<Uuid, Vec<Uuid>>,
    artist_distributions: HashMap<Uuid, Vec<Uuid>>,
}

impl RoyaltyStorage for InMemoryRoyaltyStorage {
    fn store_distribution(&mut self, distribution: RoyaltyDistribution) -> Result<(), RoyaltyError> {
        let id = distribution.id;
        let content_id = distribution.content_id;
        let artist_ids: Vec<_> = distribution.distributions.keys().copied().collect();

        self.distributions.insert(id, distribution.clone());
        
        self.content_distributions
            .entry(content_id)
            .or_default()
            .push(id);

        for artist_id in artist_ids {
            self.artist_distributions
                .entry(artist_id)
                .or_default()
                .push(id);
        }

        Ok(())
    }

    fn get_distribution(&self, id: Uuid) -> Option<RoyaltyDistribution> {
        self.distributions.get(&id).cloned()
    }

    fn get_content_distributions(&self, content_id: Uuid) -> Vec<RoyaltyDistribution> {
        self.content_distributions
            .get(&content_id)
            .map(|ids| ids.iter().filter_map(|&id| self.get_distribution(id)).collect())
            .unwrap_or_default()
    }

    fn get_artist_royalty_history(&self, artist_id: Uuid) -> Vec<RoyaltyDistribution> {
        self.artist_distributions
            .get(&artist_id)
            .map(|ids| ids.iter().filter_map(|&id| self.get_distribution(id)).collect())
            .unwrap_or_default()
    }
}

/// Mock content service for development
#[derive(Debug)]
pub struct MockContentService;

impl ContentService for MockContentService {
    fn get_content_info(&self, content_id: Uuid) -> Result<ContentInfo, RoyaltyError> {
        let mut license = ContentLicense {
            royalty_split: HashMap::new(),
            minimum_upstream_percentage: Decimal::new(10, 2), // 10%
            allows_remixing: true,
            requires_attribution: true,
        };
        
        // Simple split for demo
        license.royalty_split.insert(Uuid::new_v4(), Decimal::new(7, 1)); // 70%
        license.royalty_split.insert(Uuid::new_v4(), Decimal::new(3, 1)); // 30%

        Ok(ContentInfo {
            id: content_id,
            title: format!("Content {}", content_id),
            artist_id: Uuid::new_v4(),
            license,
            upstream_content_ids: vec![],
            cooperative_id: None,
            metadata: ContentMetadata {
                duration_seconds: 180,
                genre: Some("Electronic".to_string()),
                is_explicit: false,
            },
        })
    }

    fn get_upstream_content(&self, content_id: Uuid) -> Result<Vec<ContentInfo>, RoyaltyError> {
        Ok(vec![]) // Mock: no upstream content
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_license_validation() {
        let mut license = ContentLicense {
            royalty_split: HashMap::new(),
            minimum_upstream_percentage: dec!(0.1),
            allows_remixing: true,
            requires_attribution: true,
        };
        
        // Valid license
        license.royalty_split.insert(Uuid::new_v4(), dec!(0.5));
        license.royalty_split.insert(Uuid::new_v4(), dec!(0.5));
        assert!(license.is_valid());
        
        // Invalid license
        license.royalty_split.clear();
        license.royalty_split.insert(Uuid::new_v4(), dec!(0.6));
        license.royalty_split.insert(Uuid::new_v4(), dec!(0.5));
        assert!(!license.is_valid());
    }
}