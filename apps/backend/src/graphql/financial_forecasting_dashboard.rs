//! GraphQL API for financial forecasting dashboard with enhanced result viewing
use async_graphql::{Context, Object, SimpleObject, Result, Subscription, InputObject};
use async_graphql::futures_util::Stream;
use tokio_stream::{StreamExt, wrappers::BroadcastStream};
use uuid::Uuid;
use serde_json::Value as JsonValue;
use crate::auth::permissions::Permission;
use crate::auth::AuthError;

/// Enhanced Financial Forecast Result with dashboard-specific data
#[derive(SimpleObject)]
pub struct FinancialForecastResult {
    pub job_id: Uuid,
    pub projections: Vec<CashFlowProjection>,
    pub base_scenario: Scenario,
    pub sensitivity_scenarios: Vec<Scenario>,
}

/// Cash flow projection for dashboard display
#[derive(SimpleObject)]
pub struct CashFlowProjection {
    pub period: String,
    pub income: f64,
    pub expenses: f64,
    pub net_cash_flow: f64,
}

/// Scenario analysis result
#[derive(SimpleObject)]
pub struct Scenario {
    pub id: Uuid,
    pub name: String,
    pub parameters: JsonValue,
    pub final_net_worth: f64,
}

/// Input for updating dashboard preferences
#[derive(InputObject)]
pub struct DashboardPreferencesInput {
    pub default_date_range: Option<String>,
    pub favorite_metrics: Option<Vec<String>>,
    pub chart_settings: Option<JsonValue>,
}

/// User preferences for dashboard configuration
#[derive(SimpleObject)]
pub struct UserPreferences {
    pub user_id: Uuid,
    pub dashboard_preferences: JsonValue,
}

/// Query extensions for financial forecasting dashboard
#[derive(Default)]
pub struct FinancialForecastingDashboardQuery;

#[Object]
impl FinancialForecastingDashboardQuery {
    /// Get detailed forecast result by job ID
    async fn get_forecast_result(
        &self,
        ctx: &Context<'_>,
        job_id: Uuid,
    ) -> Result<Option<FinancialForecastResult>> {
        let auth = ctx.data::<crate::auth::AuthContext>()?;
        
        // Check permission
        if !auth.has_permission(Permission::ManageFinancialForecasting) {
            return Err(AuthError::PermissionDenied.into());
        }
        
        let db = ctx.data::<crate::db::DbPool>()?;
        
        let mut conn = db.get().await
            .map_err(|e| async_graphql::Error::new(format!("Database connection error: {}", e)))?;
        
        let job = sqlx::query!(
            r#"
            SELECT id, status, result, error_message
            FROM jobs
            WHERE id = $1 AND user_id = $2
            "#,
            job_id,
            auth.user_id
        )
        .fetch_optional(&mut *conn)
        .await
        .map_err(|e| async_graphql::Error::new(format!("Database query error: {}", e)))?;
        
        let Some(job) = job else {
            return Ok(None);
        };
        
        if job.status != "completed" || job.result.is_none() {
            return Ok(None);
        }
        
        let result_data: serde_json::Value = serde_json::from_value(job.result.unwrap())
            .unwrap_or(serde_json::json!({}));
        
        // Parse the protobuf-compatible result data
        let base_scenario = result_data.get("base_scenario")
            .map(|scenario| Scenario {
                id: Uuid::new_v4(),
                name: scenario.get("name").and_then(|v| v.as_str()).unwrap_or("Base Scenario").to_string(),
                parameters: scenario.get("parameters").cloned().unwrap_or(serde_json::json!({})),
                final_net_worth: scenario.get("final_net_worth").and_then(|v| v.as_f64()).unwrap_or(0.0),
            })
            .unwrap_or_else(|| Scenario {
                id: Uuid::new_v4(),
                name: "Base Scenario".to_string(),
                parameters: serde_json::json!({}),
                final_net_worth: 0.0,
            });
        
        let sensitivity_scenarios = result_data.get("sensitivity_scenarios")
            .and_then(|v| v.as_array())
            .map(|scenarios| {
                scenarios.iter().enumerate().map(|(idx, scenario)| Scenario {
                    id: Uuid::new_v4(),
                    name: scenario.get("name").and_then(|v| v.as_str()).unwrap_or(&format!("Scenario {}", idx + 1)).to_string(),
                    parameters: scenario.get("parameters").cloned().unwrap_or(serde_json::json!({})),
                    final_net_worth: scenario.get("final_net_worth").and_then(|v| v.as_f64()).unwrap_or(0.0),
                }).collect()
            })
            .unwrap_or_default();
        
        let projections = result_data.get("projections")
            .and_then(|v| v.as_array())
            .map(|projections| {
                projections.iter().map(|proj| CashFlowProjection {
                    period: proj.get("period").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    income: proj.get("income").and_then(|v| v.as_f64()).unwrap_or(0.0),
                    expenses: proj.get("expenses").and_then(|v| v.as_f64()).unwrap_or(0.0),
                    net_cash_flow: proj.get("net_cash_flow").and_then(|v| v.as_f64()).unwrap_or(0.0),
                }).collect()
            })
            .unwrap_or_default();
        
        Ok(Some(FinancialForecastResult {
            job_id,
            projections,
            base_scenario,
            sensitivity_scenarios,
        }))
    }
}

/// Mutation extensions for financial forecasting dashboard
#[derive(Default)]
pub struct FinancialForecastingDashboardMutation;

#[Object]
impl FinancialForecastingDashboardMutation {
    /// Update dashboard preferences for the authenticated user
    async fn update_dashboard_preferences(
        &self,
        ctx: &Context<'_>,
        input: DashboardPreferencesInput,
    ) -> Result<UserPreferences> {
        let auth = ctx.data::<crate::auth::AuthContext>()?;
        let db = ctx.data::<crate::db::DbPool>()?;
        
        // Check permission
        if !auth.has_permission(Permission::ManageFinancialForecasting) {
            return Err(AuthError::PermissionDenied.into());
        }
        
        let mut conn = db.get().await
            .map_err(|e| async_graphql::Error::new(format!("Database connection error: {}", e)))?;
        
        // Get existing preferences or create new ones
        let existing_preferences = sqlx::query!(
            r#"
            SELECT dashboard_preferences
            FROM user_preferences
            WHERE user_id = $1
            "#,
            auth.user_id
        )
        .fetch_optional(&mut *conn)
        .await
        .map_err(|e| async_graphql::Error::new(format!("Database query error: {}", e)))?;
        
        let mut preferences = existing_preferences
            .and_then(|r| r.dashboard_preferences)
            .unwrap_or_else(|| serde_json::json!({}));
        
        // Update preferences based on input
        if let Some(date_range) = input.default_date_range {
            preferences["default_date_range"] = serde_json::Value::String(date_range);
        }
        
        if let Some(metrics) = input.favorite_metrics {
            preferences["favorite_metrics"] = serde_json::Value::Array(
                metrics.into_iter().map(serde_json::Value::String).collect()
            );
        }
        
        if let Some(settings) = input.chart_settings {
            preferences["chart_settings"] = settings;
        }
        
        // Upsert the preferences
        let _ = sqlx::query!(
            r#"
            INSERT INTO user_preferences (user_id, dashboard_preferences)
            VALUES ($1, $2)
            ON CONFLICT (user_id) DO UPDATE SET
                dashboard_preferences = $2,
                updated_at = NOW()
            "#,
            auth.user_id,
            serde_json::json!(preferences)
        )
        .execute(&mut *conn)
        .await
        .map_err(|e| async_graphql::Error::new(format!("Database query error: {}", e)))?;
        
        Ok(UserPreferences {
            user_id: auth.user_id,
            dashboard_preferences: preferences,
        })
    }
}

/// Subscription extensions for financial forecasting dashboard
#[derive(Default)]
pub struct FinancialForecastingDashboardSubscription;

#[Subscription]
impl FinancialForecastingDashboardSubscription {
    /// Subscribe to job completion notifications for a user
    async fn job_completed(
        &self,
        ctx: &Context<'_>,
        user_id: Uuid,
    ) -> Result<impl Stream<Item = crate::notifications::Notification>> {
        let auth = ctx.data::<crate::auth::AuthContext>()?;
        
        // Users can only subscribe to their own notifications
        if auth.user_id != user_id {
            return Err(AuthError::PermissionDenied.into());
        }
        
        // Check permission
        if !auth.has_permission(Permission::ManageFinancialForecasting) {
            return Err(AuthError::PermissionDenied.into());
        }
        
        let notification_service = ctx.data::<crate::notifications::NotificationService>()?;
        let rx = notification_service.subscribe(user_id).await;
        
        // Convert the broadcast receiver to a Stream
        let stream = BroadcastStream::new(rx)
            .filter_map(|result| async {
                match result {
                    Ok(notification) => Some(notification),
                    Err(_) => None,
                }
            });
        
        Ok(stream)
    }
}