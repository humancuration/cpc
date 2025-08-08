//! Financial Impact Admin Dashboard
//!
//! Web-based administration interface for managing and visualizing financial impact data.

use axum::{
    routing::{get, post},
    Router, Json, Extension,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use financial_impact_tracker::{
    FinancialImpactTracker, 
    FinancialAnalytics, 
    FinancialReportGenerator,
    FinancialIntegration,
    FinancialIntegrationConfig,
};
use cpay_core::CPayCore;
use cpc_financial_core::CPCFinancialCore;
use sqlx::PgPool;
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Configuration for the financial impact admin dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialImpactAdminConfig {
    pub database_url: String,
    pub server_port: u16,
    pub enable_cors: bool,
}

/// Main application state
#[derive(Clone)]
pub struct AppState {
    pub tracker: FinancialImpactTracker,
    pub analytics: FinancialAnalytics,
    pub report_generator: FinancialReportGenerator,
    pub integration: FinancialIntegration,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    env_logger::init();

    // Load configuration from environment variables
    let config = FinancialImpactAdminConfig {
        database_url: std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgresql://localhost/cpc".to_string()),
        server_port: std::env::var("SERVER_PORT").unwrap_or_else(|_| "3002".to_string()).parse().unwrap_or(3002),
        enable_cors: std::env::var("ENABLE_CORS").map(|v| v == "true").unwrap_or(true),
    };

    // Initialize database connection
    let db_pool = PgPool::connect(&config.database_url).await?;

    // Run database migrations
    sqlx::migrate!("./migrations").run(&db_pool).await?;

    // Initialize financial impact components
    let tracker = FinancialImpactTracker::new(db_pool.clone());
    let analytics = FinancialAnalytics::new(tracker.clone());
    let report_generator = FinancialReportGenerator::new(analytics.clone());

    // Initialize integration components
    let cpay_core = CPayCore::new(/* config */);
    let financial_core = CPCFinancialCore::new(/* config */);
    
    let integration_config = FinancialIntegrationConfig {
        enable_realtime_tracking: true,
        enable_cause_linking: true,
        enable_volunteer_linking: true,
        enable_learning_linking: true,
        auto_generate_reports: true,
        report_frequency_days: 7,
    };

    let integration = FinancialIntegration::new(
        tracker.clone(),
        cpay_core,
        financial_core,
        integration_config,
    );

    // Create application state
    let app_state = AppState {
        tracker,
        analytics,
        report_generator,
        integration,
    };

    // Build router
    let mut app = Router::new()
        .route("/", get(health_check))
        .route("/api/health", get(health_check))
        .route("/api/analytics", get(get_financial_analytics))
        .route("/api/report", get(generate_report))
        .route("/api/sync", post(sync_with_cpay))
        .route("/api/transactions", get(list_transactions))
        .layer(Extension(app_state));

    // Add CORS layer if enabled
    if config.enable_cors {
        app = app.layer(CorsLayer::permissive());
    }

    // Run server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));
    let listener = TcpListener::bind(addr).await?;
    
    println!("Financial Impact Admin Dashboard listening on {}", addr);
    
    axum::serve(listener, app).await?;

    Ok(())
}

/// Health check endpoint
async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "service": "financial-impact-admin",
        "timestamp": Utc::now().to_rfc3339()
    }))
}

/// Get financial analytics
async fn get_financial_analytics(
    Extension(state): Extension<AppState>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    // For this example, we'll use the last 30 days
    let end_time = Utc::now();
    let start_time = end_time - chrono::Duration::days(30);
    
    match state.analytics.generate_impact_analytics(start_time, end_time).await {
        Ok(analytics) => Ok(Json(serde_json::to_value(analytics).map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Generate financial impact report
async fn generate_report(
    Extension(state): Extension<AppState>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    // For this example, we'll use the last 30 days
    let end_time = Utc::now();
    let start_time = end_time - chrono::Duration::days(30);
    
    match state.report_generator.generate_report(start_time, end_time).await {
        Ok(report) => Ok(Json(serde_json::to_value(report).map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Synchronize with cpay_core
async fn sync_with_cpay(
    Extension(state): Extension<AppState>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    match state.integration.synchronize_with_cpay().await {
        Ok(tracked_count) => Ok(Json(serde_json::json!({
            "success": true,
            "tracked_transactions": tracked_count
        }))),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// List recent financial transactions
async fn list_transactions(
    Extension(state): Extension<AppState>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let end_time = Utc::now();
    let start_time = end_time - chrono::Duration::days(7);
    
    match state.tracker.get_impact_records(start_time, end_time, None).await {
        Ok(records) => Ok(Json(serde_json::to_value(records).map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}