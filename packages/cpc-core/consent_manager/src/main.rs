//! Example usage of the consent manager.

use consent_manager::{
    domain::{
        consent::{ConsentProfile, DataSharingLevel, Domain},
        audit::Actor,
    },
    application::service::{ConsentService, ConsentStorage},
    infrastructure::storage::sled_adapter::SledAdapter,
};
use sled::Config;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Consent Manager Example");
    
    // Create a temporary sled database for demonstration
    let config = Config::new().temporary(true);
    let db = config.open()?;
    
    // Create the sled adapter
    let sled_adapter = SledAdapter::new(db);
    
    // Create the consent service
    let consent_service = ConsentService::new(Box::new(sled_adapter));
    
    // Example user ID
    let user_id = "user123";
    
    // Set consent level for financial data
    consent_service
        .update_consent_level(
            user_id,
            Domain::FinancialData,
            DataSharingLevel::Standard,
            Actor::User(user_id.to_string()),
        )
        .await?;
    
    println!("Set financial data consent to Standard");
    
    // Set consent level for health data
    consent_service
        .update_consent_level(
            user_id,
            Domain::HealthData,
            DataSharingLevel::Minimal,
            Actor::User(user_id.to_string()),
        )
        .await?;
    
    println!("Set health data consent to Minimal");
    
    // Get current consent levels
    let financial_level = consent_service
        .get_consent_level(user_id, Domain::FinancialData)
        .await?;
        
    let health_level = consent_service
        .get_consent_level(user_id, Domain::HealthData)
        .await?;
    
    println!("Financial data consent level: {:?}", financial_level);
    println!("Health data consent level: {:?}", health_level);
    
    // Check if a requested level is allowed
    let can_share_financial = match financial_level {
        DataSharingLevel::None => false,
        DataSharingLevel::Minimal => true,  // For this example, minimal is sufficient
        DataSharingLevel::Standard => true,
        DataSharingLevel::Full => true,
    };
    
    println!("Can share financial data: {}", can_share_financial);
    
    // Get audit events
    let audit_events = consent_service
        .get_audit_events(user_id)
        .await?;
    
    println!("Audit events:");
    for event in audit_events {
        println!("  - {:?} {:?} by {:?}", event.domain, event.action, event.actor);
    }
    
    println!("Example completed successfully!");
    Ok(())
}