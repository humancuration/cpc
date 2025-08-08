//! Privacy-aware data processing example

use bi_analytics::{
    AnalyticsEngine, 
    privacy::{PrivacySettings, ConsentAwareProcessor},
    pipeline::FinancialDataAdapter
};
use consent_manager::domain::consent::DataSharingLevel;
use polars::df;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Privacy-Aware Data Processing Example");
    
    // Create privacy settings
    let privacy_settings = PrivacySettings {
        minimum_consent_level: DataSharingLevel::Standard,
        apply_differential_privacy: true,
        differential_privacy_epsilon: 1.0,
        anonymize_by_default: true,
    };
    
    println!("✓ Created privacy settings");
    
    // Create consent-aware processor
    let processor = ConsentAwareProcessor::new(privacy_settings);
    println!("✓ Created consent-aware processor");
    
    // Create analytics engine
    let engine = AnalyticsEngine::new();
    println!("✓ Created analytics engine");
    
    // Check consent (simulated)
    let user_id = "user_123";
    let has_consent = processor.check_consent(user_id, consent_manager::domain::consent::Domain::FinancialData).await?;
    println!("User {} has consent: {}", user_id, has_consent);
    
    // Create sample financial data
    let financial_data = df![
        "transaction_id" => ["tx_001", "tx_002", "tx_003", "tx_004"],
        "amount" => [100.50, 250.75, 50.25, 1000.00],
        "category" => ["donation", "fundraising", "donation", "grant"],
        "user_id" => ["user_123", "user_123", "user_123", "user_123"]
    ]?;
    
    println!("✓ Created financial data with {} rows", financial_data.height());
    
    // Apply anonymization based on consent level
    let anonymized_data = processor.apply_anonymization(financial_data, DataSharingLevel::Standard)?;
    println!("✓ Applied anonymization - resulting data has {} rows", anonymized_data.height());
    
    // Apply differential privacy
    let private_data = processor.apply_differential_privacy(anonymized_data)?;
    println!("✓ Applied differential privacy");
    
    // Process with analytics engine
    let normalized_data = engine.normalize_data(&private_data)?;
    println!("✓ Normalized data with {} rows", normalized_data.height());
    
    println!("Privacy-aware processing completed successfully!");
    Ok(())
}