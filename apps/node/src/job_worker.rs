//! Job processing worker for Cooperative Node

use cpc_protos::job_service::{Job, JobStatus, JobType, JobStatusUpdate};
use cpc_protos::job_service::job_service_client::JobServiceClient;
use cpc_core::business::financial_forecasting::{FinancialForecast, ForecastParameters};
use cpc_core::accounting::Transaction;
use uuid::Uuid;
use serde_json;
use sqlx::PgPool;
use chrono::{DateTime, Utc, NaiveDate};

/// Job worker that processes incoming jobs
#[derive(Clone)]
pub struct JobWorker {
    node_id: String,
    db_pool: PgPool,
}

impl JobWorker {
    pub fn new(node_id: String, db_pool: PgPool) -> Self {
        Self { node_id, db_pool }
    }

    async fn process_job(
        &self,
        job: Job,
        client: &mut JobServiceClient<tonic::transport::Channel>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let job_id = Uuid::parse_str(&job.job_id)?;
        
        println!("Processing job: {} of type: {:?}", job_id, job.job_type());
        
        match job.job_type() {
            JobType::FinancialForecast => {
                self.process_financial_forecast(job, client).await?;
            }
            JobType::SensitivityAnalysis => {
                self.process_sensitivity_analysis(job, client).await?;
            }
            JobType::DataProcessing => {
                println!("Data processing not implemented yet");
            }
            JobType::Unspecified => {
                eprintln!("Unknown job type for job: {}", job_id);
            }
        }
        
        Ok(())
    }

    async fn process_financial_forecast(
        &self,
        job: Job,
        client: &mut JobServiceClient<tonic::transport::Channel>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let job_id = Uuid::parse_str(&job.job_id)?;
        
        // Parse job payload
        let payload: serde_json::Value = serde_json::from_slice(&job.payload)?;
        
        let parameters = payload.get("parameters")
            .ok_or("Missing parameters in job payload")?;
        
        // Parse dates properly
        let start_date_str = parameters.get("start_date").and_then(|v| v.as_str()).unwrap_or("2024-01-01");
        let end_date_str = parameters.get("end_date").and_then(|v| v.as_str()).unwrap_or("2024-12-31");
        
        let start_date = DateTime::parse_from_rfc3339(start_date_str)
            .or_else(|_| DateTime::parse_from_str(start_date_str, "%Y-%m-%d"))
            .unwrap_or_else(|_| DateTime::parse_from_str("2024-01-01", "%Y-%m-%d").unwrap())
            .with_timezone(&Utc)
            .date();
            
        let end_date = DateTime::parse_from_rfc3339(end_date_str)
            .or_else(|_| DateTime::parse_from_str(end_date_str, "%Y-%m-%d"))
            .unwrap_or_else(|_| DateTime::parse_from_str("2024-12-31", "%Y-%m-%d").unwrap())
            .with_timezone(&Utc)
            .date();
        
        let forecast_params = ForecastParameters {
            start_date,
            end_date,
            interval: parameters.get("interval").and_then(|v| v.as_str()).unwrap_or("monthly").to_string(),
            scenario_parameters: parameters.get("scenario_parameters")
                .and_then(|v| serde_json::from_value(v.clone()).ok())
                .unwrap_or_default(),
        };
        
        // Create forecast
        let mut forecast = FinancialForecast::new(forecast_params);
        
        // Add base scenario
        forecast.add_scenario("Base Scenario".to_string(), forecast.base_parameters.clone());
        
        // Fetch historical transactions from database
        let historical_transactions = self.fetch_historical_transactions(&job.user_id).await?;
        
        // Project cash flow
        match forecast.project_cash_flow("Base Scenario", &historical_transactions) {
            Ok(_) => {
                // Convert results to JSON
                let result = serde_json::json!({
                    "scenarios": forecast.scenarios.iter().map(|s| {
                        serde_json::json!({
                            "name": s.name,
                            "projections": s.projections.iter().map(|p| {
                                serde_json::json!({
                                    "date": p.date.format("%Y-%m-%d").to_string(),
                                    "inflow": p.inflow,
                                    "outflow": p.outflow,
                                    "net_cash_flow": p.net_cash_flow,
                                })
                            }).collect::<Vec<_>>(),
                        })
                    }).collect::<Vec<_>>(),
                });
                
                // Update job status in backend
                self.update_job_status(job_id, JobStatus::Completed, Some(result), None, client).await?;
            }
            Err(e) => {
                let error_msg = format!("Forecast failed: {}", e);
                self.update_job_status(job_id, JobStatus::Failed, None, Some(error_msg), client).await?;
            }
        }
        
        Ok(())
    }

    async fn process_sensitivity_analysis(
        &self,
        job: Job,
        client: &mut JobServiceClient<tonic::transport::Channel>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let job_id = Uuid::parse_str(&job.job_id)?;
        
        // Parse job payload
        let payload: serde_json::Value = serde_json::from_slice(&job.payload)?;
        
        let parameters = payload.get("parameters")
            .ok_or("Missing parameters in job payload")?;
        
        let base_scenario_name = parameters.get("base_scenario_name")
            .and_then(|v| v.as_str())
            .unwrap_or("Base Scenario");
            
        let new_scenario_name = parameters.get("new_scenario_name")
            .and_then(|v| v.as_str())
            .unwrap_or("Sensitivity Scenario");
            
        let sensitivity_params = parameters.get("sensitivity_parameters")
            .ok_or("Missing sensitivity parameters")?;
        
        let revenue_growth = sensitivity_params.get("revenue_growth")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);
            
        let expense_change = sensitivity_params.get("expense_change")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);
            
        let interest_rate = sensitivity_params.get("interest_rate")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);
        
        // Parse dates properly
        let start_date_str = parameters.get("start_date").and_then(|v| v.as_str()).unwrap_or("2024-01-01");
        let end_date_str = parameters.get("end_date").and_then(|v| v.as_str()).unwrap_or("2024-12-31");
        
        let start_date = DateTime::parse_from_rfc3339(start_date_str)
            .or_else(|_| DateTime::parse_from_str(start_date_str, "%Y-%m-%d"))
            .unwrap_or_else(|_| DateTime::parse_from_str("2024-01-01", "%Y-%m-%d").unwrap())
            .with_timezone(&Utc)
            .date();
            
        let end_date = DateTime::parse_from_rfc3339(end_date_str)
            .or_else(|_| DateTime::parse_from_str(end_date_str, "%Y-%m-%d"))
            .unwrap_or_else(|_| DateTime::parse_from_str("2024-12-31", "%Y-%m-%d").unwrap())
            .with_timezone(&Utc)
            .date();
        
        let forecast_params = ForecastParameters {
            start_date,
            end_date,
            interval: parameters.get("interval").and_then(|v| v.as_str()).unwrap_or("monthly").to_string(),
            scenario_parameters: parameters.get("scenario_parameters")
                .and_then(|v| serde_json::from_value(v.clone()).ok())
                .unwrap_or_default(),
        };
        
        // Create forecast
        let mut forecast = FinancialForecast::new(forecast_params);
        
        // Add base scenario
        forecast.add_scenario("Base Scenario".to_string(), forecast.base_parameters.clone());
        
        // Fetch historical transactions from database
        let historical_transactions = self.fetch_historical_transactions(&job.user_id).await?;
        
        // Project cash flow for base scenario
        if let Err(e) = forecast.project_cash_flow("Base Scenario", &historical_transactions) {
            let error_msg = format!("Base forecast failed: {}", e);
            self.update_job_status(job_id, JobStatus::Failed, None, Some(error_msg), client).await?;
            return Ok(());
        }
        
        // Run sensitivity analysis
        use cpc_core::business::financial_forecasting::SensitivityParameters;
        let sensitivity_params = SensitivityParameters {
            revenue_growth,
            expense_change,
            interest_rate,
        };
        
        match forecast.run_sensitivity_analysis(
            base_scenario_name,
            new_scenario_name,
            &sensitivity_params,
            &historical_transactions
        ) {
            Ok(_) => {
                // Convert results to JSON
                let result = serde_json::json!({
                    "scenarios": forecast.scenarios.iter().map(|s| {
                        serde_json::json!({
                            "name": s.name,
                            "projections": s.projections.iter().map(|p| {
                                serde_json::json!({
                                    "date": p.date.format("%Y-%m-%d").to_string(),
                                    "inflow": p.inflow,
                                    "outflow": p.outflow,
                                    "net_cash_flow": p.net_cash_flow,
                                })
                            }).collect::<Vec<_>>(),
                        })
                    }).collect::<Vec<_>>(),
                });
                
                // Update job status in backend
                self.update_job_status(job_id, JobStatus::Completed, Some(result), None, client).await?;
            }
            Err(e) => {
                let error_msg = format!("Sensitivity analysis failed: {}", e);
                self.update_job_status(job_id, JobStatus::Failed, None, Some(error_msg), client).await?;
            }
        }
        
        Ok(())
    }

    async fn fetch_historical_transactions(
        &self,
        user_id: &str,
    ) -> Result<Vec<Transaction>, Box<dyn std::error::Error>> {
        println!("Fetching historical transactions for user: {}", user_id);
        
        // Query historical transactions from database
        let transactions = sqlx::query!(
            r#"
            SELECT
                id,
                account_id,
                amount,
                description,
                transaction_date as "transaction_date: chrono::NaiveDateTime",
                category,
                transaction_type
            FROM transactions
            WHERE account_id IN (
                SELECT id FROM accounts WHERE user_id = $1
            )
            AND transaction_date >= NOW() - INTERVAL '2 years'
            ORDER BY transaction_date ASC
            "#,
            user_id
        )
        .fetch_all(&self.db_pool)
        .await?
        .into_iter()
        .map(|row| Transaction {
            id: row.id,
            account_id: row.account_id,
            amount: row.amount,
            description: row.description,
            date: row.transaction_date.date(),
            category: row.category,
            transaction_type: row.transaction_type,
        })
        .collect();
        
        println!("Found {} historical transactions", transactions.len());
        Ok(transactions)
    }

    async fn update_job_status(
        &self,
        job_id: Uuid,
        status: JobStatus,
        result: Option<serde_json::Value>,
        error_message: Option<String>,
        client: &mut JobServiceClient<tonic::transport::Channel>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("Updating job {} status to: {:?}", job_id, status);
        
        let result_bytes = result.map(|r| serde_json::to_vec(&r)).transpose()?;
        
        let request = JobStatusUpdate {
            job_id: job_id.to_string(),
            status: status as i32,
            error_message: error_message.unwrap_or_default(),
            result: result_bytes.unwrap_or_default(),
            metadata: Default::default(),
        };
        
        let response = client.update_job_status(request).await?;
        let response = response.into_inner();
        
        if !response.success {
            return Err(format!("Failed to update job status: {}", response.message).into());
        }
        
        println!("Successfully updated job status for: {}", job_id);
        Ok(())
    }
}