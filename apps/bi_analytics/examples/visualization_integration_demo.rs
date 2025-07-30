//! Example demonstrating the new BI visualization web integration

use cpc_bi_analytics::{
    application::{
        visualization_service::BevyVisualizationService,
        compliance_management::ComplianceManagementService,
        data_ingestion::DataIngestionService,
        report_generation::ReportGenerationService,
        dashboard_management::DashboardManagementService,
    },
    presentation::{
        web::{
            create_schema,
            create_router,
            BiAnalyticsState,
        },
        bevy_visualization::BiVisualizationApp,
    },
    infrastructure::{
        postgres_repository::PostgresBiRepository,
        p2p_data_source::P2PDataSource,
    },
};
use axum::{
    Server,
    Router,
};
use std::sync::Arc;
use tracing::{info, Level};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("Starting BI Analytics Visualization Integration Demo");

    // Create services
    let postgres_repo = PostgresBiRepository::new("postgresql://localhost/cpc_bi");
    let p2p_source = P2PDataSource::new();
    
    let data_ingestion = DataIngestionService::new(postgres_repo.clone(), p2p_source);
    let report_generation = ReportGenerationService::new(postgres_repo.clone(), postgres_repo.clone());
    let dashboard_management = DashboardManagementService::new(postgres_repo.clone(), postgres_repo.clone());
    let compliance_management = ComplianceManagementService::new(postgres_repo.clone(), postgres_repo.clone());
    let visualization_service = BevyVisualizationService::new();

    // Create GraphQL schema
    let graphql_schema = create_schema(
        data_ingestion,
        report_generation,
        dashboard_management,
        compliance_management,
        visualization_service,
    );

    // Create state
    let state = BiAnalyticsState {
        graphql_schema,
        subscription_manager: Arc::new(cpc_bi_analytics::presentation::web::routes::SubscriptionManager::default()),
    };

    // Create router
    let app = create_router(state);

    // Start server
    let addr = "127.0.0.1:3000";
    info!("Server listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    Server::from_tcp(listener.into_std()?)?
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

#[cfg(test)]
mod demo_tests {
    use super::*;
    use cpc_bi_analytics::{
        presentation::web::graphql::{VisualizationPayload, Base64Image},
    };

    #[tokio::test]
    async fn test_visualization_endpoints() {
        // This would be an integration test in a real environment
        // For now, we'll test the service creation
        
        let service = BevyVisualizationService::new();
        assert!(service.is_ready());
    }

    #[test]
    fn test_bevy_headless_rendering() {
        let mut app = BiVisualizationApp::new_headless();
        assert!(app.is_headless);
        
        // Test rendering
        let image_data = app.render_to_image(400, 300).unwrap();
        assert!(!image_data.is_empty());
        assert!(image_data.len() > 100); // Should be more than just header
    }

    #[test]
    fn test_scene_export() {
        let mut app = BiVisualizationApp::new_headless();
        
        // Add a simple visualization
        let report = cpc_bi_analytics::domain::report::Report {
            id: uuid::Uuid::new_v4(),
            dataset_id: uuid::Uuid::new_v4(),
            name: "Test Report".to_string(),
            description: Some("Test data".to_string()),
            query: "SELECT * FROM test".to_string(),
            visualization_type: cpc_bi_analytics::domain::report::VisualizationType::BarChart,
            owner_id: uuid::Uuid::new_v4(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        
        app.add_report_visualization(&report);
        
        // Test scene export
        let scene_data = app.export_scene_data().unwrap();
        assert!(scene_data.is_object());
        assert!(scene_data.get("type").unwrap().as_str().unwrap() == "gltf");
    }
}