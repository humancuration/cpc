use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    body::Body,
    http::{Request, StatusCode},
    routing::post,
    Router,
};
use tower::ServiceExt;
use cpc_protos::metrics::AggregatedMetrics;
use serde_json::{json, Value};
use crate::services::metrics_service::MetricsService;
use cpc_protos::metrics::metrics_service_server::MetricsService as MetricsServiceTrait;
use std::sync::Arc;
use tokio::sync::Mutex;

async fn graphql_handler(schema: async_graphql::Schema<crate::graphql::QueryRoot, async_graphql::EmptyMutation, async_graphql::EmptySubscription>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[tokio::test]
async fn test_graphql_metrics_day_range() {
    let schema = crate::graphql::create_schema().await;
    
    let query = r#"
        query {
            getAggregatedMetrics(timeRange: "day") {
                totalMembers
                activeMembers
            }
        }
    "#;
    
    let response = schema.execute(query).await;
    assert!(response.errors.is_empty());
    
    let data = response.data.into_json().unwrap();
    assert_eq!(data["getAggregatedMetrics"]["totalMembers"], 150);
    assert_eq!(data["getAggregatedMetrics"]["activeMembers"], 120);
}

#[tokio::test]
async fn test_graphql_metrics_week_range() {
    let schema = crate::graphql::create_schema().await;
    
    let query = r#"
        query {
            getAggregatedMetrics(timeRange: "week") {
                totalProducts
                totalSales
            }
        }
    "#;
    
    let response = schema.execute(query).await;
    assert!(response.errors.is_empty());
    
    let data = response.data.into_json().unwrap();
    assert_eq!(data["getAggregatedMetrics"]["totalProducts"], 85);
    assert_eq!(data["getAggregatedMetrics"]["totalSales"], 125000.0);
}

#[tokio::test]
async fn test_graphql_metrics_month_range() {
    let schema = crate::graphql::create_schema().await;
    
    let query = r#"
        query {
            getAggregatedMetrics(timeRange: "month") {
                totalProfit
                totalCarbonSaved
            }
        }
    "#;
    
    let response = schema.execute(query).await;
    assert!(response.errors.is_empty());
    
    let data = response.data.into_json().unwrap();
    assert_eq!(data["getAggregatedMetrics"]["totalProfit"], 25000.0);
    assert_eq!(data["getAggregatedMetrics"]["totalCarbonSaved"], 4500.5);
}

#[tokio::test]
async fn test_graphql_metrics_single_role() {
    let schema = crate::graphql::create_schema().await;
    
    let query = r#"
        query {
            getAggregatedMetrics(timeRange: "day", memberRoles: ["admin"]) {
                avgProfitPerMember
                memberEngagement
            }
        }
    "#;
    
    let response = schema.execute(query).await;
    assert!(response.errors.is_empty());
    
    let data = response.data.into_json().unwrap();
    assert_eq!(data["getAggregatedMetrics"]["avgProfitPerMember"], 208.33);
    assert_eq!(data["getAggregatedMetrics"]["memberEngagement"], 0.85);
}

#[tokio::test]
async fn test_graphql_metrics_multiple_roles() {
    let schema = crate::graphql::create_schema().await;
    
    let query = r#"
        query {
            getAggregatedMetrics(timeRange: "day", memberRoles: ["admin", "member"]) {
                totalMembers
            }
        }
    "#;
    
    let response = schema.execute(query).await;
    assert!(response.errors.is_empty());
    
    let data = response.data.into_json().unwrap();
    assert_eq!(data["getAggregatedMetrics"]["totalMembers"], 150);
}

#[tokio::test]
async fn test_graphql_metrics_no_roles() {
    let schema = crate::graphql::create_schema().await;
    
    let query = r#"
        query {
            getAggregatedMetrics(timeRange: "day") {
                activeMembers
            }
        }
    "#;
    
    let response = schema.execute(query).await;
    assert!(response.errors.is_empty());
    
    let data = response.data.into_json().unwrap();
    assert_eq!(data["getAggregatedMetrics"]["activeMembers"], 120);
}

#[tokio::test]
async fn test_graphql_invalid_date_format() {
    let schema = crate::graphql::create_schema().await;
    
    let query = r#"
        query {
            getAggregatedMetrics(timeRange: "invalid-date") {
                totalMembers
            }
        }
    "#;
    
    let response = schema.execute(query).await;
    assert!(response.errors.is_empty(), "Should handle invalid date formats gracefully");
    
    let data = response.data.into_json().unwrap();
    assert_eq!(data["getAggregatedMetrics"]["totalMembers"], 150);
}

#[tokio::test]
async fn test_graphql_nonexistent_role() {
    let schema = crate::graphql::create_schema().await;
    
    let query = r#"
        query {
            getAggregatedMetrics(timeRange: "day", memberRoles: ["nonexistent-role"]) {
                totalMembers
            }
        }
    "#;
    
    let response = schema.execute(query).await;
    assert!(response.errors.is_empty(), "Should handle nonexistent roles gracefully");
    
    let data = response.data.into_json().unwrap();
    assert_eq!(data["getAggregatedMetrics"]["totalMembers"], 150);
}