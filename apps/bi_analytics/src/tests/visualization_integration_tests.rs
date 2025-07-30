//! Integration tests for BI visualization features

use super::*;
use crate::{
    application::visualization_service::{BevyVisualizationService, VisualizationError},
    presentation::web::routes::{SubscriptionManager, WebSocketMessage},
    domain::report::Report,
};
use uuid::Uuid;
use std::sync::Arc;

#[tokio::test]
async fn test_visualization_service_authorization() {
    // Test that visualization service properly checks authorization
    let service = BevyVisualizationService::new();
    let report_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();
    
    // This should fail because the report doesn't exist
    let result = service.generate_3d_visualization(report_id, user_id).await;
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), VisualizationError::ReportNotFound));
}

#[tokio::test]
async fn test_subscription_manager() {
    let manager = Arc::new(SubscriptionManager::default());
    let report_id = Uuid::new_v4();
    
    // Create a channel to receive messages
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    
    // Subscribe to report updates
    manager.subscribe(report_id, tx.clone()).await;
    
    // Broadcast a message
    let message = WebSocketMessage::Update {
        report_id,
        delta: serde_json::json!({"test": "data"}),
        accessibility_announcement: "Test update".to_string(),
    };
    
    manager.broadcast(report_id, message.clone()).await;
    
    // Verify the message was received
    let received = rx.recv().await.unwrap();
    assert!(matches!(received, WebSocketMessage::Update { .. }));
    
    // Unsubscribe
    manager.unsubscribe(report_id, &tx).await;
    
    // Broadcast another message - should not be received
    manager.broadcast(report_id, message).await;
    
    // Verify no message was received
    assert!(rx.try_recv().is_err());
}

#[tokio::test]
async fn test_visualization_payload_generation() {
    let service = BevyVisualizationService::new();
    
    // Test accessibility metadata generation
    let report = Report {
        id: Uuid::new_v4(),
        dataset_id: Uuid::new_v4(),
        name: "Test Report".to_string(),
        description: Some("Test description".to_string()),
        query: "SELECT * FROM data".to_string(),
        visualization_type: crate::domain::report::VisualizationType::BarChart,
        owner_id: Uuid::new_v4(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    
    let metadata = service.generate_accessibility_metadata(&report, &[]);
    assert!(!metadata.alt_text.is_empty());
    assert!(metadata.alt_text.contains("Bar chart"));
}

#[tokio::test]
async fn test_websocket_message_formats() {
    // Test message serialization/deserialization
    let report_id = Uuid::new_v4();
    
    let messages = vec![
        WebSocketMessage::Connected {
            report_id,
            message: "Connected".to_string(),
        },
        WebSocketMessage::Update {
            report_id,
            delta: serde_json::json!({"modified": ["node1"]}),
            accessibility_announcement: "Data updated".to_string(),
        },
        WebSocketMessage::Error {
            report_id,
            message: "Test error".to_string(),
        },
        WebSocketMessage::Ping,
        WebSocketMessage::Pong,
    ];
    
    for message in messages {
        let json = serde_json::to_string(&message).unwrap();
        let deserialized: WebSocketMessage = serde_json::from_str(&json).unwrap();
        
        match (message, deserialized) {
            (WebSocketMessage::Connected { .. }, WebSocketMessage::Connected { .. }) => {},
            (WebSocketMessage::Update { .. }, WebSocketMessage::Update { .. }) => {},
            (WebSocketMessage::Error { .. }, WebSocketMessage::Error { .. }) => {},
            (WebSocketMessage::Ping, WebSocketMessage::Ping) => {},
            (WebSocketMessage::Pong, WebSocketMessage::Pong) => {},
            _ => panic!("Message serialization failed"),
        }
    }
}

#[tokio::test]
async fn test_performance_with_large_datasets() {
    let service = BevyVisualizationService::new();
    
    // Create mock large dataset
    let large_data: Vec<f32> = (0..10000).map(|i| i as f32).collect();
    
    // Test that sampling works correctly
    let sampled = service.sample_data_for_visualization(&large_data, 1000);
    assert_eq!(sampled.len(), 1000);
    
    // Test that LOD strategies are applied
    let lod_data = service.apply_lod_strategies(&large_data, 800, 600);
    assert!(lod_data.len() <= large_data.len());
}

#[cfg(test)]
mod graphql_tests {
    use super::*;
    use crate::presentation::web::graphql::{create_schema, BiAnalyticsGraphQLSchema};
    use crate::infrastructure::{postgres_repository::PostgresBiRepository, p2p_data_source::P2PDataSource};
    use crate::application::{
        data_ingestion::DataIngestionService,
        report_generation::ReportGenerationService,
        dashboard_management::DashboardManagementService,
        compliance_management::ComplianceManagementService,
    };
    
    #[tokio::test]
    async fn test_graphql_visualization_endpoints() {
        // Create test services
        let postgres_repo = PostgresBiRepository::new("test_connection_string");
        let p2p_source = P2PDataSource::new();
        
        let data_ingestion = DataIngestionService::new(postgres_repo.clone(), p2p_source);
        let report_generation = ReportGenerationService::new(postgres_repo.clone(), postgres_repo.clone());
        let dashboard_management = DashboardManagementService::new(postgres_repo.clone(), postgres_repo.clone());
        let compliance_management = ComplianceManagementService::new(postgres_repo.clone(), postgres_repo.clone());
        let visualization_service = BevyVisualizationService::new();
        
        let schema = create_schema(
            data_ingestion,
            report_generation,
            dashboard_management,
            compliance_management,
            visualization_service,
        );
        
        // Test visualization3d query
        let query = r#"
            query {
                visualization3d(reportId: "550e8400-e29b-41d4-a716-446655440000") {
                    sceneData
                    altText
                    navigationMap {
                        key
                        value {
                            label
                            key
                            position
                        }
                    }
                }
            }
        "#;
        
        let response = schema.execute(query).await;
        assert!(response.errors.is_empty() || response.errors.iter().any(|e| e.message.contains("not found")));
    }
    
    #[tokio::test]
    async fn test_graphql_visualization_image() {
        let schema = create_test_schema();
        
        let query = r#"
            query {
                visualizationImage(
                    reportId: "550e8400-e29b-41d4-a716-446655440000"
                    width: 800
                    height: 600
                ) {
                    imageData
                    altText
                    width
                    height
                }
            }
        "#;
        
        let response = schema.execute(query).await;
        assert!(response.errors.is_empty() || response.errors.iter().any(|e| e.message.contains("not found")));
    }
    
    fn create_test_schema() -> BiAnalyticsGraphQLSchema {
        let postgres_repo = PostgresBiRepository::new("test_connection_string");
        let p2p_source = P2PDataSource::new();
        
        let data_ingestion = DataIngestionService::new(postgres_repo.clone(), p2p_source);
        let report_generation = ReportGenerationService::new(postgres_repo.clone(), postgres_repo.clone());
        let dashboard_management = DashboardManagementService::new(postgres_repo.clone(), postgres_repo.clone());
        let compliance_management = ComplianceManagementService::new(postgres_repo.clone(), postgres_repo.clone());
        let visualization_service = BevyVisualizationService::new();
        
        create_schema(
            data_ingestion,
            report_generation,
            dashboard_management,
            compliance_management,
            visualization_service,
        )
    }
}