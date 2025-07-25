#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use uuid::Uuid;
    use crate::accounting::money::Money;

    fn create_test_diversity_stats() -> DiversityStats {
        let mut ethnicity_breakdown = HashMap::new();
        ethnicity_breakdown.insert("Asian".to_string(), 35.2);
        ethnicity_breakdown.insert("Black".to_string(), 18.7);
        ethnicity_breakdown.insert("Hispanic".to_string(), 22.1);
        ethnicity_breakdown.insert("White".to_string(), 19.4);
        ethnicity_breakdown.insert("Other".to_string(), 4.6);

        DiversityStats {
            gender_balance: 0.48,
            ethnicity_breakdown,
            pay_equity: 0.95,
        }
    }

    #[test]
    fn test_impact_report_creation_valid() {
        let org_id = Uuid::new_v4();
        let diversity_stats = create_test_diversity_stats();
        
        let report = ImpactReport::new(
            org_id,
            2024,
            MetricTonnes(125.5),
            MetricTonnes(89.3),
            45.7,
            diversity_stats,
            150,
            Money::new(2_500_000_00, "USD"),
            Money::new(45_000_00, "USD"),
            Money::new(175_000_00, "USD"),
        );

        assert!(report.is_ok());
        let report = report.unwrap();
        assert_eq!(report.org_id, org_id);
        assert_eq!(report.year, 2024);
        assert_eq!(report.total_carbon_sequestered.0, 125.5);
        assert_eq!(report.supplier_count, 150);
    }

    #[test]
    fn test_impact_report_invalid_year() {
        let org_id = Uuid::new_v4();
        let diversity_stats = create_test_diversity_stats();
        
        let report = ImpactReport::new(
            org_id,
            1999, // Invalid year
            MetricTonnes(125.5),
            MetricTonnes(89.3),
            45.7,
            diversity_stats.clone(),
            150,
            Money::new(2_500_000_00, "USD"),
            Money::new(45_000_00, "USD"),
            Money::new(175_000_00, "USD"),
        );

        assert!(report.is_err());
        match report {
            Err(ImpactError::Validation(msg)) => {
                assert!(msg.contains("Invalid year"));
            }
            _ => panic!("Expected validation error for invalid year"),
        }
    }

    #[test]
    fn test_impact_report_invalid_renewable_percentage() {
        let org_id = Uuid::new_v4();
        let diversity_stats = create_test_diversity_stats();
        
        let report = ImpactReport::new(
            org_id,
            2024,
            MetricTonnes(125.5),
            MetricTonnes(89.3),
            150.0, // Invalid percentage
            diversity_stats,
            150,
            Money::new(2_500_000_00, "USD"),
            Money::new(45_000_00, "USD"),
            Money::new(175_000_00, "USD"),
        );

        assert!(report.is_err());
        match report {
            Err(ImpactError::Validation(msg)) => {
                assert!(msg.contains("Renewable energy percentage"));
            }
            _ => panic!("Expected validation error for invalid percentage"),
        }
    }

    #[test]
    fn test_diversity_stats_invalid_ethnicity_breakdown() {
        let mut ethnicity_breakdown = HashMap::new();
        ethnicity_breakdown.insert("Asian".to_string(), 50.0);
        ethnicity_breakdown.insert("Black".to_string(), 30.0);
        // Missing 20% to reach 100%

        let diversity_stats = DiversityStats {
            gender_balance: 0.48,
            ethnicity_breakdown,
            pay_equity: 0.95,
        };

        let validation_result = diversity_stats.validate();
        assert!(validation_result.is_err());
        match validation_result {
            Err(ImpactError::Validation(msg)) => {
                assert!(msg.contains("must sum to 100%"));
            }
            _ => panic!("Expected validation error for invalid ethnicity breakdown"),
        }
    }

    #[test]
    fn test_diversity_stats_invalid_gender_balance() {
        let mut ethnicity_breakdown = HashMap::new();
        ethnicity_breakdown.insert("Asian".to_string(), 100.0);

        let diversity_stats = DiversityStats {
            gender_balance: 1.5, // Invalid (> 1.0)
            ethnicity_breakdown,
            pay_equity: 0.95,
        };

        let validation_result = diversity_stats.validate();
        assert!(validation_result.is_err());
        match validation_result {
            Err(ImpactError::Validation(msg)) => {
                assert!(msg.contains("Gender balance must be between"));
            }
            _ => panic!("Expected validation error for invalid gender balance"),
        }
    }

    #[test]
    fn test_impact_report_zero_suppliers() {
        let org_id = Uuid::new_v4();
        let diversity_stats = create_test_diversity_stats();
        
        let report = ImpactReport::new(
            org_id,
            2024,
            MetricTonnes(125.5),
            MetricTonnes(89.3),
            45.7,
            diversity_stats,
            0, // Zero suppliers
            Money::new(2_500_000_00, "USD"),
            Money::new(45_000_00, "USD"),
            Money::new(175_000_00, "USD"),
        );

        assert!(report.is_err());
        match report {
            Err(ImpactError::Validation(msg)) => {
                assert!(msg.contains("Supplier count must be greater than 0"));
            }
            _ => panic!("Expected validation error for zero suppliers"),
        }
    }

    // Note: Async service tests are in service.rs module
}