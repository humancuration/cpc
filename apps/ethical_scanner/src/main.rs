//! Main entry point for the EthicalScanner application

use ethical_scanner::{
    scanner::ScannerService,
    health_engine::HealthEngine,
    supply_chain::SupplyChainService,
    suggestions::SuggestionsService,
    api::{graphql::build_schema, grpc::EthicalScannerService},
};
use shared_packages::consent_manager::application::service::{ConsentService, ConsentStorage};
use shared_packages::consent_manager::domain::consent::{Domain, DataSharingLevel};
use std::sync::Arc;

/// Mock storage implementation for consent manager
/// In a real implementation, this would connect to a database
#[derive(Debug, Clone)]
struct MockConsentStorage;

#[async_trait::async_trait]
impl ConsentStorage for MockConsentStorage {
    async fn get_consent_profile(
        &self,
        _user_id: &str,
        _domain: &shared_packages::consent_manager::domain::consent::Domain,
    ) -> Result<Option<shared_packages::consent_manager::domain::consent::ConsentProfile>, shared_packages::consent_manager::domain::errors::ConsentError> {
        // For demo purposes, return a mock profile with Standard consent level
        let profile = shared_packages::consent_manager::domain::consent::ConsentProfile::new(
            "user123".to_string(),
            Domain::ScmData,
            DataSharingLevel::Standard,
        );
        Ok(Some(profile))
    }

    async fn save_consent_profile(
        &self,
        _profile: &shared_packages::consent_manager::domain::consent::ConsentProfile,
    ) -> Result<(), shared_packages::consent_manager::domain::errors::ConsentError> {
        Ok(())
    }

    async fn revoke_domain(
        &self,
        _user_id: &str,
        _domain: &shared_packages::consent_manager::domain::consent::Domain,
    ) -> Result<(), shared_packages::consent_manager::domain::errors::ConsentError> {
        Ok(())
    }

    async fn get_audit_events(
        &self,
        _user_id: &str,
    ) -> Result<Vec<shared_packages::consent_manager::domain::audit::AuditEvent>, shared_packages::consent_manager::domain::errors::ConsentError> {
        Ok(vec![])
    }

    async fn save_audit_event(
        &self,
        _event: &shared_packages::consent_manager::domain::audit::AuditEvent,
    ) -> Result<(), shared_packages::consent_manager::domain::errors::ConsentError> {
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting EthicalScanner application...");

    // Initialize services
    let scanner_service = ScannerService::new();
    let health_engine = HealthEngine::new();
    
    // Initialize consent service with mock storage
    let consent_storage = Box::new(MockConsentStorage);
    let consent_service = ConsentService::new(consent_storage);
    let supply_chain_service = SupplyChainService::new(consent_service);
    
    let suggestions_service = SuggestionsService::new();
    let graphql_schema = build_schema();
    let grpc_service = EthicalScannerService::new();

    // Demo the services
    println!("Initializing camera...");
    scanner_service.init_camera().await?;

    println!("Scanning barcode...");
    let barcode = scanner_service.scan_barcode().await?;
    println!("Scanned barcode: {}", barcode);

    println!("Processing scan...");
    let product = scanner_service.process_scan(&barcode).await?;
    println!("Product: {} by {}", product.name, product.brand);

    println!("Calculating health score...");
    let health_score = health_engine.calculate_health_score(&product)?;
    println!("Health score: {:.2}", health_score);

    println!("Building GraphQL schema...");
    let _schema = graphql_schema;

    println!("EthicalScanner application initialized successfully!");

    // Keep the application running
    println!("Press Ctrl+C to exit");
    tokio::signal::ctrl_c().await?;
    println!("Shutting down EthicalScanner application...");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_application_startup() {
        // This is a placeholder test
        assert!(true);
    }
}