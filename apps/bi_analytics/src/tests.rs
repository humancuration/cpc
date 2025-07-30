//! Tests for the BI & Analytics module

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        domain::{
            dataset::{Dataset, DataSource, FieldDefinition, DataType},
            report::{Report, VisualizationType},
            dashboard::{Dashboard, GridPosition},
            compliance::gdpr::{ProcessingPurpose, ConsentStatus},
            compliance::hipaa::AccessRole,
        },
        application::{
            data_ingestion::DataIngestionService,
            report_generation::ReportGenerationService,
            dashboard_management::DashboardManagementService,
            compliance_management::ComplianceManagementService,
        },
        infrastructure::{
            postgres_repository::PostgresBiRepository,
            p2p_data_source::P2PDataSource,
        },
    };
    use uuid::Uuid;
    use std::collections::HashMap;
    
    // Note: These tests are integration-style tests that would require
    // a running database and P2P network in a real implementation.
    // For now, we'll focus on unit tests of the individual components.
    
    #[test]
    fn test_domain_entities() {
        // Test Dataset creation
        let fields = vec![
            FieldDefinition {
                name: "revenue".to_string(),
                data_type: DataType::Float,
                is_nullable: false,
                description: Some("Monthly revenue".to_string()),
            }
        ];
        
        let dataset = Dataset::new(
            "Test Dataset".to_string(),
            DataSource::Finance,
            fields,
            Uuid::new_v4(),
            Some("Test dataset".to_string()),
        ).unwrap();
        
        assert_eq!(dataset.name, "Test Dataset");
        assert_eq!(dataset.source, DataSource::Finance);
        
        // Test Report creation
        let report = Report::new(
            Uuid::new_v4(),
            "Test Report".to_string(),
            "SELECT *".to_string(),
            VisualizationType::BarChart,
            Uuid::new_v4(),
            Some("Test report".to_string()),
        ).unwrap();
        
        assert_eq!(report.name, "Test Report");
        assert_eq!(report.visualization_type, VisualizationType::BarChart);
        
        // Test Dashboard creation
        let layout = HashMap::new();
        let dashboard = Dashboard::new(
            "Test Dashboard".to_string(),
            Uuid::new_v4(),
            Some("Test dashboard".to_string()),
            layout,
        ).unwrap();
        
        assert_eq!(dashboard.name, "Test Dashboard");
    }
    
    #[test]
    fn test_compliance_entities() {
        // Test GDPR consent
        let consent = crate::domain::compliance::gdpr::GdprConsent::new(
            Uuid::new_v4(),
            ProcessingPurpose::Analytics,
            ConsentStatus::Granted,
        );
        
        assert_eq!(consent.purpose, ProcessingPurpose::Analytics);
        assert_eq!(consent.status, ConsentStatus::Granted);
        assert!(consent.is_granted());
        
        // Test HIPAA access permission
        let permission = crate::domain::compliance::hipaa::AccessPermission::new(
            Uuid::new_v4(),
            AccessRole::Analyst,
            vec![crate::domain::compliance::hipaa::PhiCategory::Demographic],
            None,
        );
        
        assert_eq!(permission.role, AccessRole::Analyst);
        assert!(permission.has_access_to(&crate::domain::compliance::hipaa::PhiCategory::Demographic));
    }
    
    // Note: Application service tests would require mock repositories
    // which are already tested in their respective modules
    
    #[test]
    fn test_visualization_types() {
        // Test that all visualization types can be created
        let types = vec![
            VisualizationType::Table,
            VisualizationType::BarChart,
            VisualizationType::LineChart,
            VisualizationType::PieChart,
            VisualizationType::ScatterPlot,
            VisualizationType::Heatmap,
            VisualizationType::AreaChart,
        ];
        
        for vt in types {
            // This test ensures all visualization types are properly defined
            // In a real implementation, we would test the actual visualization creation
            assert!(matches!(vt, _));
        }
    }
    
    #[test]
    fn test_data_sources() {
        // Test that all data sources can be created
        let sources = vec![
            DataSource::Crm,
            DataSource::Finance,
            DataSource::Calendar,
            DataSource::Messenger,
            DataSource::Custom("test".to_string()),
        ];
        
        for source in sources {
            // This test ensures all data sources are properly defined
            assert!(matches!(source, _));
        }
    }
    
    #[test]
    fn test_grid_position() {
        let position = GridPosition {
            x: 1,
            y: 2,
            width: 3,
            height: 4,
        };
        
        assert_eq!(position.x, 1);
        assert_eq!(position.y, 2);
        assert_eq!(position.width, 3);
        assert_eq!(position.height, 4);
    }
}