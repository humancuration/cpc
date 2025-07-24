//! GraphQL API for financial forecasting with async job processing
use async_graphql::{Context, Object, SimpleObject, InputObject, Result, Subscription};
use async_graphql::futures_util::Stream;
use chrono::{Date, Utc};
use tokio_stream::{StreamExt, wrappers::BroadcastStream};
use uuid::Uuid;
use std::collections::HashMap;
use crate::auth::permissions::Permission;
use crate::services::calendar::CalendarService;
use crate::auth::AuthError;
use crate::grpc::job_server::{create_job, dispatch_job};
use crate::notifications::NotificationType;

/// Input for creating a financial forecast job
#[derive(InputObject)]
pub struct CreateForecastJobInput {
    /// Forecast parameters
    pub parameters: ForecastParametersInput,
}

/// Input for creating a sensitivity analysis job
#[derive(InputObject)]
pub struct CreateSensitivityAnalysisJobInput {
    /// Base scenario name to analyze
    pub base_scenario_name: String,
    /// New scenario name for sensitivity results
    pub new_scenario_name: String,
    /// Sensitivity parameters
    pub sensitivity_parameters: SensitivityParametersInput,
    /// Forecast parameters
    pub parameters: ForecastParametersInput,
}

/// Sensitivity analysis parameters
#[derive(InputObject)]
pub struct SensitivityParametersInput {
    pub revenue_growth: f64,
    pub expense_change: f64,
    pub interest_rate: f64,
}

/// Forecast parameters input
#[derive(InputObject)]
pub struct ForecastParametersInput {
    pub start_date: Date<Utc>,
    pub end_date: Date<Utc>,
    pub interval: String,
    pub scenario_parameters: HashMap<String, f64>,
    /// Algorithm to use for forecasting
    pub algorithm: Option<String>,
}

/// Response for creating a forecast job
#[derive(SimpleObject)]
pub struct CreateForecastJobPayload {
    /// The job ID for tracking the async task
    pub job_id: Uuid,
    /// Initial status of the job
    pub status: String,
}

/// Financial forecast result type
#[derive(SimpleObject)]
pub struct ForecastResult {
    pub job_id: Uuid,
    pub status: String,
    pub scenarios: Vec<ScenarioOutput>,
    pub error_message: Option<String>,
}

/// Input for scheduling a training session
#[derive(InputObject)]
pub struct ScheduleTrainingInput {
    pub scenario_id: Uuid,
    pub title: String,
    pub description: String,
    pub collaborators: Vec<Uuid>,
}

/// Output for scheduling a training session
#[derive(SimpleObject)]
pub struct ScheduleTrainingPayload {
    pub event_id: Uuid,
}

/// Scenario output type
#[derive(SimpleObject)]
pub struct ScenarioOutput {
    pub name: String,
    pub projections: Vec<ProjectionOutput>,
}

/// Cash flow projection output
#[derive(SimpleObject)]
pub struct ProjectionOutput {
    pub date: Date<Utc>,
    pub inflow: f64,
    pub outflow: f64,
    pub net_cash_flow: f64,
}

#[derive(Default)]
pub struct FinancialForecastingMutation;

#[Object]
impl FinancialForecastingMutation {
    /// Create a new financial forecast job (async)
    async fn create_forecast_job(
        &self,
        ctx: &Context<'_>,
        input: CreateForecastJobInput,
    ) -> Result<CreateForecastJobPayload> {
        let auth = ctx.data::<crate::auth::AuthContext>()?;
        
        // Check permission
        if !auth.has_permission(Permission::ManageFinancialForecasting) {
            return Err(AuthError::PermissionDenied.into());
        }
        
        let db = ctx.data::<crate::db::DbPool>()?;
        let notification_service = ctx.data::<crate::notifications::NotificationService>()?;
        
        // Prepare job payload
        let mut parameters = input.parameters;
        if let Some(algorithm) = parameters.algorithm {
            parameters.scenario_parameters.insert("algorithm".to_string(), algorithm.parse().unwrap_or(0.0));
        }
        
        let payload = serde_json::json!({
            "parameters": parameters,
            "user_id": auth.user_id.to_string(),
        });
        
        // Create job in database
        let job_id = create_job(
            db,
            "financial_forecast",
            auth.user_id,
            payload,
            Some(serde_json::json!({
                "start_date": parameters.start_date,
                "end_date": parameters.end_date,
                "algorithm": parameters.algorithm,
            }))
        )
        .await
        .map_err(|e| async_graphql::Error::new(format!("Failed to create job: {}", e)))?;
        
        // Create job protobuf for dispatch
        let job = cpc_protos::job_service::Job {
            job_id: job_id.to_string(),
            job_type: cpc_protos::job_service::JobType::FinancialForecast as i32,
            status: cpc_protos::job_service::JobStatus::Pending as i32,
            user_id: auth.user_id.to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            started_at: String::new(),
            completed_at: String::new(),
            payload: serde_json::to_vec(&payload).unwrap_or_default(),
            error_message: String::new(),
            metadata: HashMap::new(),
        };
        
        // Dispatch job to worker
        let job_server = ctx.data::<crate::grpc::job_server::JobServerState>()?;
        dispatch_job(job_server, job).await
            .map_err(|e| async_graphql::Error::new(format!("Failed to dispatch job: {}", e)))?;
        
        // Send notification
        notification_service.send_notification(
            auth.user_id,
            NotificationType::JobStarted {
                job_id,
                job_type: "financial_forecast".to_string(),
            },
            serde_json::json!({
                "job_id": job_id,
                "job_type": "financial_forecast",
            })
        ).await.ok();
        
        Ok(CreateForecastJobPayload {
            job_id,
            status: "pending".to_string(),
        })
    }

    /// Create a new sensitivity analysis job (async)
    async fn create_sensitivity_analysis_job(
        &self,
        ctx: &Context<'_>,
        input: CreateSensitivityAnalysisJobInput,
    ) -> Result<CreateForecastJobPayload> {
        let auth = ctx.data::<crate::auth::AuthContext>()?;
        
        // Check permission
        if !auth.has_permission(Permission::ManageFinancialForecasting) {
            return Err(AuthError::PermissionDenied.into());
        }
        
        let db = ctx.data::<crate::db::DbPool>()?;
        let notification_service = ctx.data::<crate::notifications::NotificationService>()?;
        
        // Prepare job payload
        let mut parameters = input.parameters;
        if let Some(algorithm) = parameters.algorithm {
            parameters.scenario_parameters.insert("algorithm".to_string(), algorithm.parse().unwrap_or(0.0));
        }
        
        let payload = serde_json::json!({
            "base_scenario_name": input.base_scenario_name,
            "new_scenario_name": input.new_scenario_name,
            "sensitivity_parameters": {
                "revenue_growth": input.sensitivity_parameters.revenue_growth,
                "expense_change": input.sensitivity_parameters.expense_change,
                "interest_rate": input.sensitivity_parameters.interest_rate,
            },
            "parameters": parameters,
            "user_id": auth.user_id.to_string(),
        });
        
        // Create job in database
        let job_id = create_job(
            db,
            "sensitivity_analysis",
            auth.user_id,
            payload,
            Some(serde_json::json!({
                "base_scenario_name": input.base_scenario_name,
                "new_scenario_name": input.new_scenario_name,
                "start_date": parameters.start_date,
                "end_date": parameters.end_date,
            }))
        )
        .await
        .map_err(|e| async_graphql::Error::new(format!("Failed to create job: {}", e)))?;
        
        // Create job protobuf for dispatch
        let job = cpc_protos::job_service::Job {
            job_id: job_id.to_string(),
            job_type: cpc_protos::job_service::JobType::SensitivityAnalysis as i32,
            status: cpc_protos::job_service::JobStatus::Pending as i32,
            user_id: auth.user_id.to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            started_at: String::new(),
            completed_at: String::new(),
            payload: serde_json::to_vec(&payload).unwrap_or_default(),
            error_message: String::new(),
            metadata: HashMap::new(),
        };
        
        // Dispatch job to worker
        let job_server = ctx.data::<crate::grpc::job_server::JobServerState>()?;
        dispatch_job(job_server, job).await
            .map_err(|e| async_graphql::Error::new(format!("Failed to dispatch job: {}", e)))?;
        
        // Send notification
        notification_service.send_notification(
            auth.user_id,
            NotificationType::JobStarted {
                job_id,
                job_type: "sensitivity_analysis".to_string(),
            },
            serde_json::json!({
                "job_id": job_id,
                "job_type": "sensitivity_analysis",
            })
        ).await.ok();
        
        Ok(CreateForecastJobPayload {
            job_id,
            status: "pending".to_string(),
        })
    }
    
    /// Schedule a training session for a financial forecast scenario
    async fn schedule_training(
        &self,
        ctx: &Context<'_>,
        input: ScheduleTrainingInput,
    ) -> Result<ScheduleTrainingPayload> {
        let auth = ctx.data::<crate::auth::AuthContext>()?;
        
        // Check permission
        if !auth.has_permission(Permission::ManageTrainingSchedule) {
            return Err(AuthError::PermissionDenied.into());
        }
        
        let calendar_service = ctx.data::<crate::services::calendar::CalendarService>()?;
        let event = calendar_service.create_event(
            input.title,
            input.description,
            Utc::now(), // Default start time - should be parameterized in real use
            Utc::now() + chrono::Duration::hours(2), // Default end time
            None, // Location
            input.collaborators,
            None, // Training doc ID
            auth.user_id,
        ).await?;
        
        Ok(ScheduleTrainingPayload {
            event_id: event.id,
        })
    }
}

#[derive(Default)]
pub struct FinancialForecastingQuery;

#[Object]
impl FinancialForecastingQuery {
    /// Get forecast job by ID
    async fn forecast_job(
        &self,
        ctx: &Context<'_>,
        job_id: Uuid,
    ) -> Result<Option<ForecastResult>> {
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
        
        let result = if let Some(result_data) = job.result {
            let forecast_data: serde_json::Value = serde_json::from_value(result_data)
                .unwrap_or(serde_json::json!({}));
            
            // Parse the result data into the expected format
            let scenarios = forecast_data.get("scenarios")
                .and_then(|v| v.as_array())
                .map(|scenarios| {
                    scenarios.iter().map(|scenario| {
                        ScenarioOutput {
                            name: scenario.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                            projections: scenario.get("projections")
                                .and_then(|v| v.as_array())
                                .map(|projections| {
                                    projections.iter().map(|proj| {
                                        ProjectionOutput {
                                            date: chrono::DateTime::parse_from_rfc3339(
                                                proj.get("date").and_then(|v| v.as_str()).unwrap_or("")
                                            )
                                            .map(|dt| dt.date_naive())
                                            .unwrap_or_else(|_| chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
                                            inflow: proj.get("inflow").and_then(|v| v.as_f64()).unwrap_or(0.0),
                                            outflow: proj.get("outflow").and_then(|v| v.as_f64()).unwrap_or(0.0),
                                            net_cash_flow: proj.get("net_cash_flow").and_then(|v| v.as_f64()).unwrap_or(0.0),
                                        }
                                    }).collect()
                                })
                                .unwrap_or_default(),
                        }
                    }).collect()
                })
                .unwrap_or_default()
        } else {
            vec![]
        };
        
        Ok(Some(ForecastResult {
            job_id,
            status: job.status,
            scenarios: result,
            error_message: job.error_message,
        }))
    }
}

/// Subscription for forecast job updates
#[derive(Default)]
pub struct FinancialForecastingSubscription;

#[Subscription]
impl FinancialForecastingSubscription {
    /// Subscribe to updates for a specific forecast job
    async fn on_forecast_result(
        &self,
        ctx: &Context<'_>,
        job_id: Uuid,
    ) -> Result<impl Stream<Item = ForecastResult>> {
        let auth = ctx.data::<crate::auth::AuthContext>()?;
        
        // Check permission
        if !auth.has_permission(Permission::ManageFinancialForecasting) {
            return Err(AuthError::PermissionDenied.into());
        }
        
        let notification_service = ctx.data::<crate::notifications::NotificationService>()?;
        
        // Subscribe to notifications for this user
        let mut rx = notification_service.subscribe(auth.user_id).await;
        
        // Filter for this specific job
        let stream = async_stream::stream! {
            while let Ok(notification) = rx.recv().await {
                if let NotificationType::JobCompleted { job_id: notified_job_id, .. } = notification.notification_type {
                    if notified_job_id == job_id {
                        // Fetch the updated job
                        let db = ctx.data::<crate::db::DbPool>().unwrap();
                        let mut conn = db.get().await.unwrap();
                        
                        let job = sqlx::query!(
                            r#"
                            SELECT id, status, result, error_message
                            FROM jobs
                            WHERE id = $1 AND user_id = $2
                            "#,
                            job_id,
                            auth.user_id
                        )
                        .fetch_one(&mut *conn)
                        .await;
                        
                        if let Ok(job) = job {
                            let result = if let Some(result_data) = job.result {
                                let forecast_data: serde_json::Value = serde_json::from_value(result_data)
                                    .unwrap_or(serde_json::json!({}));
                                
                                let scenarios = forecast_data.get("scenarios")
                                    .and_then(|v| v.as_array())
                                    .map(|scenarios| {
                                        scenarios.iter().map(|scenario| {
                                            ScenarioOutput {
                                                name: scenario.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                                                projections: scenario.get("projections")
                                                    .and_then(|v| v.as_array())
                                                    .map(|projections| {
                                                        projections.iter().map(|proj| {
                                                            ProjectionOutput {
                                                                date: chrono::DateTime::parse_from_rfc3339(
                                                                    proj.get("date").and_then(|v| v.as_str()).unwrap_or("")
                                                                )
                                                                .map(|dt| dt.date_naive())
                                                                .unwrap_or_else(|_| chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
                                                                inflow: proj.get("inflow").and_then(|v| v.as_f64()).unwrap_or(0.0),
                                                                outflow: proj.get("outflow").and_then(|v| v.as_f64()).unwrap_or(0.0),
                                                                net_cash_flow: proj.get("net_cash_flow").and_then(|v| v.as_f64()).unwrap_or(0.0),
                                                            }
                                                        }).collect()
                                                    })
                                                    .unwrap_or_default(),
                                            }
                                        }).collect()
                                    })
                                    .unwrap_or_default()
                            } else {
                                vec![]
                            };
                            
                            yield ForecastResult {
                                job_id,
                                status: job.status,
                                scenarios: result,
                                error_message: job.error_message,
                            };
                        }
                    }
                }
            }
        };
        
        Ok(stream)
    }
}