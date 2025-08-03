//! Integration tests for EthicalScanner

use ethical_scanner::{
    scanner::ScannerService,
    health_engine::HealthEngine,
    supply_chain::SupplyChainService,
    suggestions::SuggestionsService,
    data_models::Product,
};

// Mock consent service for testing
struct MockConsentService;

impl MockConsentService {
    fn new() -> Self {
        Self
    }
}

// Note: These are placeholder tests since we can't easily test the full integration
// without a real database and consent manager

#[test]
fn test_module_creation() {
    let scanner = ScannerService::new();
    let health_engine = HealthEngine::new();
    let consent_service = MockConsentService::new();
    let supply_chain = SupplyChainService::new(consent_service);
    let suggestions = SuggestionsService::new();
    
    assert!(true); // Simple test to ensure all modules can be created
}

#[test]
fn test_data_models() {
    let product = Product {
        id: uuid::Uuid::new_v4(),
        barcode: "123456789012".to_string(),
        name: "Test Product".to_string(),
        brand: "Test Brand".to_string(),
        category: "Food".to_string(),
        ingredients: vec![],
        nutritional_info: ethical_scanner::data_models::NutritionalFacts {
            calories: 100.0,
            protein: 5.0,
            carbs: 20.0,
            fats: 2.0,
            sugars: 10.0,
            fiber: 3.0,
            sodium: 0.1,
        },
        ethical_score: 0.75,
        supply_chain: vec![],
    };
    
    // Test validation
    let validation_result = product.validate();
    assert!(validation_result.is_ok());
}