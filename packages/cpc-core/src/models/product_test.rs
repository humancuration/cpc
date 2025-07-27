#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_product_creation() {
        let product = Product::new(
            "Test Product".to_string(),
            Some("Test Description".to_string()),
            "Electronics".to_string(),
            Some("Test Brand".to_string()),
            Some("TEST-SKU-123".to_string()),
            Some("1234567890128".to_string()),
        );

        assert_eq!(product.name, "Test Product");
        assert_eq!(product.description, Some("Test Description".to_string()));
        assert_eq!(product.category, "Electronics");
        assert_eq!(product.brand, Some("Test Brand".to_string()));
        assert_eq!(product.sku, Some("TEST-SKU-123".to_string()));
        assert_eq!(product.barcode, Some("1234567890128".to_string()));
        assert!(!product.id.is_nil());
    }

    #[test]
    fn test_product_origin_creation() {
        let product_id = Uuid::new_v4();
        let mut origin = ProductOrigin::new(
            product_id,
            "United States".to_string(),
            Some("California".to_string()),
            Some("Los Angeles".to_string()),
            "Test Manufacturer".to_string(),
            Some("123 Manufacturing St".to_string()),
            Some("ISO 9001 Certified".to_string()),
        );

        assert_eq!(origin.product_id, product_id);
        assert_eq!(origin.country_of_origin, "United States");
        assert_eq!(origin.region, Some("California".to_string()));
        assert_eq!(origin.city, Some("Los Angeles".to_string()));
        assert_eq!(origin.manufacturer, "Test Manufacturer");
        assert_eq!(origin.manufacturer_address, Some("123 Manufacturing St".to_string()));
        assert_eq!(origin.certification_info, Some("ISO 9001 Certified".to_string()));
        assert!(!origin.verified);
        assert!(origin.verification_date.is_none());

        // Test verification
        origin.verify();
        assert!(origin.verified);
        assert!(origin.verification_date.is_some());
    }

    #[test]
    fn test_supply_chain_creation() {
        let product_id = Uuid::new_v4();
        let timestamp = Utc::now();
        
        let mut supply_chain = SupplyChain::new(
            product_id,
            SupplyChainStage::Manufacturing,
            "Factory Location".to_string(),
            "Manufacturer Inc.".to_string(),
            timestamp,
            Some("Initial manufacturing".to_string()),
            None,
        );

        assert_eq!(supply_chain.product_id, product_id);
        assert_eq!(supply_chain.stage, SupplyChainStage::Manufacturing);
        assert_eq!(supply_chain.location, "Factory Location");
        assert_eq!(supply_chain.organization, "Manufacturer Inc.");
        assert_eq!(supply_chain.timestamp, timestamp);
        assert_eq!(supply_chain.description, Some("Initial manufacturing".to_string()));
        assert_eq!(supply_chain.verification_status, VerificationStatus::Unverified);
        assert!(supply_chain.previous_stage_id.is_none());
        assert!(supply_chain.next_stage_id.is_none());

        // Test verification status update
        supply_chain.set_verification_status(VerificationStatus::Verified);
        assert_eq!(supply_chain.verification_status, VerificationStatus::Verified);

        // Test linking to next stage
        let next_stage_id = Uuid::new_v4();
        supply_chain.link_next_stage(next_stage_id);
        assert_eq!(supply_chain.next_stage_id, Some(next_stage_id));
    }

    #[test]
    fn test_supply_chain_stage_serialization() {
        let stages = vec![
            SupplyChainStage::RawMaterial,
            SupplyChainStage::Manufacturing,
            SupplyChainStage::Processing,
            SupplyChainStage::Packaging,
            SupplyChainStage::Distribution,
            SupplyChainStage::Wholesale,
            SupplyChainStage::Retail,
            SupplyChainStage::Consumer,
        ];

        for stage in stages {
            let json = serde_json::to_string(&stage).unwrap();
            let deserialized: SupplyChainStage = serde_json::from_str(&json).unwrap();
            assert_eq!(stage, deserialized);
        }
    }

    #[test]
    fn test_verification_status_serialization() {
        let statuses = vec![
            VerificationStatus::Unverified,
            VerificationStatus::Pending,
            VerificationStatus::Verified,
            VerificationStatus::Disputed,
            VerificationStatus::Rejected,
        ];

        for status in statuses {
            let json = serde_json::to_string(&status).unwrap();
            let deserialized: VerificationStatus = serde_json::from_str(&json).unwrap();
            assert_eq!(status, deserialized);
        }
    }
}