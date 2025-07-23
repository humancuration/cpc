use crate::{
    models::impact::{ImpactCategory, ImpactTimelinePoint, ImpactBreakdownItem, UnifiedImpactReport},
    repositories::impact_repository::ImpactRepository,
    utils::datetime::now_utc,
};
use anyhow::{anyhow, Result};
use uuid::Uuid;
use std::sync::Arc;
use std::collections::HashMap;
use chrono::Utc;

/// Services for impact calculation and reporting
use ed25519_dalek::{Signer, Keypair};
use std::env;

/// Unified service for impact reporting
pub struct ImpactService {
    calculator: ImpactCalculator,
    aggregator: ImpactAggregator,
}

impl ImpactService {
    /// Creates a new ImpactService instance
    pub fn new(repository: Arc<dyn ImpactRepository>) -> Self {
        Self {
            calculator: ImpactCalculator::new(repository.clone()),
            aggregator: ImpactAggregator::new(repository),
        }
    }

    /// Gets a comprehensive impact report for a user
    pub async fn get_user_impact_report(&self, user_id: Uuid) -> Result<UnifiedImpactReport> {
        // Calculate overall score and ethical distribution
        let end_date = Utc::now();
        let start_date = end_date - chrono::Duration::days(30);
        let calculator_report = self.calculator.calculate_impact(user_id, start_date, end_date).await?;

        // Get timeline data
        let timeline = self.aggregator.aggregate_user_impact(user_id).await?;

        // Build unified report
        let report = UnifiedImpactReport {
            user_id,
            generated_at: Utc::now(),
            overall_score: calculator_report.overall_score,
            ethical_distribution: calculator_report.category_distribution,
            timeline,
            breakdown: Vec::new(), // Will be implemented in follow-up
            signature: calculator_report.signature,
        };

        Ok(report)
    }
}

/// Internal service for impact calculation
struct ImpactCalculator {
    repository: Arc<dyn ImpactRepository>,
    signing_key: Keypair,
}

impl ImpactCalculator {
    /// Creates a new ImpactCalculator instance
    fn new(repository: Arc<dyn ImpactRepository>) -> Self {
        let signing_key = Self::load_signing_key();
        Self { repository, signing_key }
    }

    fn load_signing_key() -> Keypair {
        let key_bytes = base64::decode(env::var("IMPACT_SIGNING_KEY")
            .expect("IMPACT_SIGNING_KEY must be set"))
            .expect("Failed to decode IMPACT_SIGNING_KEY");
        Keypair::from_bytes(&key_bytes).expect("Invalid signing key")
    }

    /// Calculates impact based on raw data
    async fn calculate_impact(&self, user_id: Uuid, start_date: chrono::DateTime<Utc>, end_date: chrono::DateTime<Utc>) -> Result<UnifiedImpactReport> {
        // Fetch raw impact data
        let raw_data = self.repository.get_impact_data(user_id, start_date, end_date).await?;
        
        // Calculate overall score and category distribution
        let mut category_totals: HashMap<ImpactCategory, f64> = HashMap::new();
        let mut overall_score = 0.0;
        
        for data in raw_data {
            let weight = self.get_category_weight(data.category).await?;
            let value = data.value * weight;
            
            overall_score += value;
            *category_totals.entry(data.category).or_insert(0.0) += value;
        }
        
        // Create impact report
        let mut report = UnifiedImpactReport {
            user_id,
            generated_at: Utc::now(),
            overall_score,
            ethical_distribution: category_totals,
            timeline: Vec::new(),
            breakdown: Vec::new(),
            signature: String::new(),
        };
        
        // Sign the report
        report.signature = self.sign_report(&report)?;
        
        Ok(report)
    }

    /// Signs the impact report
    fn sign_report(&self, report: &UnifiedImpactReport) -> Result<String> {
        #[derive(serde::Serialize)]
        struct CanonicalReport<'a> {
            user_id: Uuid,
            generated_at: chrono::DateTime<Utc>,
            overall_score: f64,
            ethical_distribution: &'a HashMap<ImpactCategory, f64>,
        }
        
        let canonical = CanonicalReport {
            user_id: report.user_id,
            generated_at: report.generated_at,
            overall_score: report.overall_score,
            ethical_distribution: &report.ethical_distribution,
        };
        
        let serialized = serde_json::to_vec(&canonical)?;
        let signature = self.signing_key.sign(&serialized);
        Ok(base64::encode(signature.to_bytes()))
    }

    /// Gets weight for a given impact category
    async fn get_category_weight(&self, category: ImpactCategory) -> Result<f64> {
        match category {
            ImpactCategory::Environmental => Ok(1.2),
            ImpactCategory::Social => Ok(1.0),
            ImpactCategory::Economic => Ok(0.8),
        }
    }
}

/// Internal service for impact data aggregation
struct ImpactAggregator {
    repository: Arc<dyn ImpactRepository>,
}

impl ImpactAggregator {
    /// Creates a new ImpactAggregator instance
    fn new(repository: Arc<dyn ImpactRepository>) -> Self {
        Self { repository }
    }

    /// Aggregates impact data for a user
    async fn aggregate_user_impact(&self, user_id: Uuid) -> Result<Vec<ImpactTimelinePoint>> {
        let data = self.repository.get_user_impact_timeline(user_id).await?;
        
        // Group data by time period and category
        let mut aggregated = HashMap::new();
        
        for point in data {
            let key = (point.timestamp.date(), point.category);
            *aggregated.entry(key).or_insert(0.0) += point.value;
        }
        
        // Convert to timeline points
        let timeline = aggregated.into_iter().map(|((date, category), value)| {
            ImpactTimelinePoint {
                timestamp: date.and_hms_opt(0, 0, 0).unwrap().and_utc(),
                value,
                category,
            }
        }).collect();
        
        Ok(timeline)
    }
}

// Signature verification remains public
pub fn verify_signature_internal(report_json: &str, public_key: &str) -> Result<(), anyhow::Error> {
    use ed25519_dalek::{Verifier, PublicKey, Signature};
    
    #[derive(serde::Serialize)]
    struct CanonicalReport {
        user_id: uuid::Uuid,
        generated_at: chrono::DateTime<chrono::Utc>,
        overall_score: f64,
        ethical_distribution: std::collections::HashMap<crate::models::impact::ImpactCategory, f64>,
    }

    let report: crate::models::impact::UnifiedImpactReport = serde_json::from_str(report_json)?;
    let signature_bytes = base64::decode(&report.signature)?;
    let signature = Signature::from_bytes(&signature_bytes)?;

    let public_key_bytes = base64::decode(public_key)?;
    let public_key = PublicKey::from_bytes(&public_key_bytes)?;

    let canonical = CanonicalReport {
        user_id: report.user_id,
        generated_at: report.generated_at,
        overall_score: report.overall_score,
        ethical_distribution: report.ethical_distribution,
    };

    let serialized = serde_json::to_vec(&canonical)?;

    public_key.verify_strict(&serialized, &signature)?;

    Ok(())
}