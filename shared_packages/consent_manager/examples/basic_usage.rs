//! Basic usage example of the consent manager.

use consent_manager::{
    domain::{
        consent::{DataSharingLevel, Domain},
        audit::Actor,
    },
    application::service::ConsentService,
    infrastructure::storage::sled_adapter::SledAdapter,
};
use sled::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Consent Manager Basic Usage Example");
    println!("===================================");
    
    // Create a temporary sled database for demonstration
    let config = Config::new().temporary(true);
    let db = config.open()?;
    
    // Create the sled adapter
    let sled_adapter = SledAdapter::new(db);
    
    // Create the consent service
    let consent_service = ConsentService::new(Box::new(sled_adapter));
    
    // Example user ID
    let user_id = "example_user";
    
    println!("\n1. Setting initial consent levels...");
    
    // Set consent level for financial data
    consent_service
        .update_consent_level(
            user_id,
            Domain::FinancialData,
            DataSharingLevel::Standard,
            Actor::User(user_id.to_string()),
        )
        .await?;
    
    println!("   ✓ Set financial data consent to Standard");
    
    // Set consent level for health data
    consent_service
        .update_consent_level(
            user_id,
            Domain::HealthData,
            DataSharingLevel::Minimal,
            Actor::User(user_id.to_string()),
        )
        .await?;
    
    println!("   ✓ Set health data consent to Minimal");
    
    println!("\n2. Checking current consent levels...");
    
    // Get current consent levels
    let financial_level = consent_service
        .get_consent_level(user_id, Domain::FinancialData)
        .await?;
        
    let health_level = consent_service
        .get_consent_level(user_id, Domain::HealthData)
        .await?;
    
    println!("   Financial data consent level: {:?}", financial_level);
    println!("   Health data consent level: {:?}", health_level);
    
    println!("\n3. Checking if data sharing is allowed...");
    
    // Check if a requested level is allowed for financial data
    let can_share_financial = match financial_level {
        DataSharingLevel::None => false,
        _ => true,  // For this example, any level above None allows sharing
    };
    
    println!("   Can share financial data: {}", can_share_financial);
    
    // Check if a requested level is allowed for health data
    let can_share_health = match health_level {
        DataSharingLevel::None => false,
        DataSharingLevel::Minimal => {
            // Minimal level might only allow basic sharing
            println!("   Health data sharing limited to minimal details");
            true
        },
        _ => true,
    };
    
    println!("   Can share health data: {}", can_share_health);
    
    println!("\n4. Retrieving audit trail...");
    
    // Get audit events
    let audit_events = consent_service
        .get_audit_events(user_id)
        .await?;
    
    println!("   Found {} audit events:", audit_events.len());
    for (i, event) in audit_events.iter().enumerate() {
        println!("     {}. {:?} {:?} by {:?}", i+1, event.domain, event.action, event.actor);
    }
    
    println!("\n5. Updating consent level...");
    
    // Update consent level for health data
    consent_service
        .update_consent_level(
            user_id,
            Domain::HealthData,
            DataSharingLevel::Standard,
            Actor::User(user_id.to_string()),
        )
        .await?;
    
    println!("   ✓ Updated health data consent to Standard");
    
    // Check updated level
    let updated_health_level = consent_service
        .get_consent_level(user_id, Domain::HealthData)
        .await?;
    
    println!("   Updated health data consent level: {:?}", updated_health_level);
    
    println!("\n6. Revoking consent...");
    
    // Revoke financial data consent
    consent_service
        .revoke_domain(
            user_id,
            Domain::FinancialData,
            Actor::User(user_id.to_string()),
        )
        .await?;
    
    println!("   ✓ Revoked financial data consent");
    
    // Check revoked level
    let revoked_financial_level = consent_service
        .get_consent_level(user_id, Domain::FinancialData)
        .await?;
    
    println!("   Revoked financial data consent level: {:?}", revoked_financial_level);
    
    println!("\nExample completed successfully!");
    Ok(())
}