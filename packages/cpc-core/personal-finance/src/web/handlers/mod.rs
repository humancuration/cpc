//! HTTP handlers for REST API endpoints
//!
//! Provides RESTful API endpoints as an alternative to GraphQL
//! for clients that prefer traditional REST over GraphQL

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    application::finance_service::FinanceService,
    web::dto::{ApiResponse, CreateExpenseRequest, CreateBudgetRequest, CreateSavingsGoalRequest},
};

/// Query parameters for filtering
#[derive(Debug, Deserialize)]
pub struct FilterParams {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub category: Option<String>,
    pub limit: Option<usize>,
}

/// Health check endpoint
pub async fn health_check() -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(serde_json::json!({
        "status": "healthy",
        "service": "personal-finance",
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

/// Get financial overview
pub async fn get_financial_overview(
    State(service): State<FinanceService>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<ApiResponse<crate::web::dto::FinancialOverviewDto>>, StatusCode> {
    match service.get_financial_overview(user_id).await {
        Ok(overview) => Ok(Json(ApiResponse::success(overview))),
        Err(e) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Create expense
pub async fn create_expense(
    State(service): State<FinanceService>,
    Json(request): Json<CreateExpenseRequest>,
) -> Result<Json<ApiResponse<crate::web::dto::ExpenseDto>>, StatusCode> {
    match service.create_expense(request).await {
        Ok(expense) => Ok(Json(ApiResponse::success(expense))),
        Err(e) => Err(StatusCode::BAD_REQUEST),
    }
}

/// Create budget
pub async fn create_budget(
    State(service): State<FinanceService>,
    Json(request): Json<CreateBudgetRequest>,
) -> Result<Json<ApiResponse<crate::domain::models::Budget>>, StatusCode> {
    match service.create_budget(request).await {
        Ok(budget) => Ok(Json(ApiResponse::success(budget))),
        Err(e) => Err(StatusCode::BAD_REQUEST),
    }
}

/// Create savings goal
pub async fn create_savings_goal(
    State(service): State<FinanceService>,
    Json(request): Json<CreateSavingsGoalRequest>,
) -> Result<Json<ApiResponse<crate::web::dto::SavingsGoalDto>>, StatusCode> {
    match service.create_savings_goal(request).await {
        Ok(goal) => Ok(Json(ApiResponse::success(goal))),
        Err(e) => Err(StatusCode::BAD_REQUEST),
    }
}

/// Get expenses with filtering
pub async fn get_expenses(
    State(service): State<FinanceService>,
    Path(user_id): Path<Uuid>,
    Query(params): Query<FilterParams>,
) -> Result<Json<ApiResponse<Vec<crate::web::dto::ExpenseDto>>>, StatusCode> {
    match service.get_expenses(user_id, params).await {
        Ok(expenses) => Ok(Json(ApiResponse::success(expenses))),
        Err(e) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Get budgets for user
pub async fn get_budgets(
    State(service): State<FinanceService>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<crate::domain::models::Budget>>>, StatusCode> {
    match service.get_budgets(user_id).await {
        Ok(budgets) => Ok(Json(ApiResponse::success(budgets))),
        Err(e) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Get savings goals for user
pub async fn get_savings_goals(
    State(service): State<FinanceService>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<crate::web::dto::SavingsGoalDto>>>, StatusCode> {
    match service.get_savings_goals(user_id).await {
        Ok(goals) => Ok(Json(ApiResponse::success(goals))),
        Err(e) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Process receipt from image
pub async fn process_receipt_image(
    State(service): State<FinanceService>,
    Path(user_id): Path<Uuid>,
    axum::extract::Multipart(mut multipart): axum::extract::Multipart,
) -> Result<Json<ApiResponse<crate::infrastructure::services::ocr_service::ReceiptData>>, StatusCode> {
    // Extract image data from multipart
    let mut image_data = Vec::new();
    let mut mime_type = String::from("image/jpeg");
    
    while let Some(field) = multipart.next_field().await.unwrap() {
        if field.name() == Some("image") {
            image_data = field.bytes().await.unwrap().to_vec();
            if let Some(content_type) = field.content_type() {
                mime_type = content_type.to_string();
            }
        }
    }

    let request = crate::web::dto::ReceiptProcessingRequest {
        user_id,
        image_data: Some(image_data),
        image_url: None,
        mime_type: Some(mime_type),
    };

    match service.process_receipt(request).await {
        Ok(receipt) => Ok(Json(ApiResponse::success(receipt))),
        Err(e) => Err(StatusCode::BAD_REQUEST),
    }
}

/// Process receipt from URL
pub async fn process_receipt_url(
    State(service): State<FinanceService>,
    Path(user_id): Path<Uuid>,
    Json(request): Json<crate::web::dto::ReceiptProcessingRequest>,
) -> Result<Json<ApiResponse<crate::infrastructure::services::ocr_service::ReceiptData>>, StatusCode> {
    let mut req = request;
    req.user_id = user_id;
    
    match service.process_receipt(req).await {
        Ok(receipt) => Ok(Json(ApiResponse::success(receipt))),
        Err(e) => Err(StatusCode::BAD_REQUEST),
    }
}