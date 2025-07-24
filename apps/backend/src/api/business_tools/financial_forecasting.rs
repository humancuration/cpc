use crate::{
    api::auth::AuthenticatedUser,
    error::ApiError,
    permissions::{Permission, PermissionCheck},
};
use axum::{
    extract::{Path, State},
    Json,
};
use cpc_core::business::financial_forecasting::{
    FinancialForecast, FinancialForecastingParams, SensitivityParameters,
};
use sqlx::PgPool;

/// Run sensitivity analysis endpoint
#[axum_macros::debug_handler]
pub async fn run_sensitivity_analysis(
    State(pool): State<PgPool>,
    user: AuthenticatedUser,
    Path(scenario_name): Path<String>,
    Json(params): Json<SensitivityParameters>,
) -> Result<Json<FinancialForecast>, ApiError> {
    // Check permission
    if !user.has_permission(Permission::RunSensitivityAnalysis) {
        return Err(ApiError::Forbidden(
            "Insufficient permissions for sensitivity analysis".into(),
        ));
    }

    let mut forecast = FinancialForecast::new(FinancialForecastingParams::default());
    forecast
        .run_sensitivity_analysis(&scenario_name, &params)
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

    Ok(Json(forecast))
}

/// Update projections endpoint with consistent parameter naming
#[axum_macros::debug_handler]
pub async fn update_projections(
    State(pool): State<PgPool>,
    user: AuthenticatedUser,
    Json(params): Json<FinancialForecastingParams>,
) -> Result<Json<FinancialForecast>, ApiError> {
    // Check permission
    if !user.has_permission(Permission::EditFinancialScenarios) {
        return Err(ApiError::Forbidden(
            "Insufficient permissions to edit scenarios".into(),
        ));
    }

    let mut forecast = FinancialForecast::new(params);
    forecast
        .update_projections(&[])
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

    Ok(Json(forecast))
}