//! gRPC Job Server for asynchronous task processing
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tonic::{Request, Response, Status};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde_json::json;

use crate::db::DbPool;
use cpc_protos::job_service::{
    job_service_server::JobService,
    Job,
    JobSubscriptionRequest,
    JobStatusUpdate,
    JobStatusResponse,
    GetJobRequest,
    JobType,
    JobStatus,
};

/// Shared state for the job server
#[derive(Debug, Clone)]
pub struct JobServerState {
    /// Database connection pool
    pub db: DbPool,
    /// Active job subscribers
    pub subscribers: Arc<RwLock<HashMap<String, mpsc::Sender<Job>>>>,
}

impl JobServerState {
    pub fn new(db: DbPool) -> Self {
        Self {
            db,
            subscribers: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

/// gRPC Job Service implementation
#[derive(Debug)]
pub struct JobServer {
    state: JobServerState,
}

impl JobServer {
    pub fn new(state: JobServerState) -> Self {
        Self { state }
    }
}

#[tonic::async_trait]
impl JobService for JobServer {
    /// Server-streaming RPC for workers to receive jobs
    async fn subscribe_to_jobs(
        &self,
        request: Request<JobSubscriptionRequest>,
    ) -> Result<Response<tonic::Streaming<Job>>, Status> {
        let req = request.into_inner();
        let worker_id = req.worker_id.clone();
        
        tracing::info!("Worker {} subscribing to jobs", worker_id);
        
        // Create channel for streaming jobs to this worker
        let (tx, rx) = mpsc::channel(100);
        
        // Store the sender in subscribers
        {
            let mut subscribers = self.state.subscribers.write().await;
            subscribers.insert(worker_id.clone(), tx);
        }
        
        // Create a stream from the receiver
        let stream = tokio_stream::wrappers::ReceiverStream::new(rx);
        
        Ok(Response::new(stream))
    }
    
    /// Unary RPC for workers to report job completion
    async fn update_job_status(
        &self,
        request: Request<JobStatusUpdate>,
    ) -> Result<Response<JobStatusResponse>, Status> {
        let req = request.into_inner();
        
        tracing::info!("Updating job {} status to {:?}", req.job_id, req.status);
        
        // Update job in database
        let mut conn = self.state.db.get()
            .await
            .map_err(|e| Status::internal(format!("Database connection error: {}", e)))?;
        
        let job_id = Uuid::parse_str(&req.job_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid job ID: {}", e)))?;
        
        let status_str = match req.status {
            JobStatus::Pending => "pending",
            JobStatus::Running => "running",
            JobStatus::Completed => "completed",
            JobStatus::Failed => "failed",
            JobStatus::Cancelled => "cancelled",
        };
        
        let query = sqlx::query!(
            r#"
            UPDATE jobs 
            SET status = $1, 
                completed_at = CASE 
                    WHEN $1 = 'completed' OR $1 = 'failed' THEN NOW() 
                    ELSE completed_at 
                END,
                error_message = $2,
                result = $3
            WHERE id = $4
            RETURNING id
            "#,
            status_str,
            req.error_message.as_deref(),
            req.result.as_deref(),
            job_id
        );
        
        let result = query.fetch_optional(&mut *conn)
            .await
            .map_err(|e| Status::internal(format!("Database update error: {}", e)))?;
        
        if result.is_none() {
            return Err(Status::not_found("Job not found"));
        }
        
        Ok(Response::new(JobStatusResponse {
            success: true,
            message: "Job status updated successfully".to_string(),
        }))
    }
    
    /// Get job details by ID
    async fn get_job(
        &self,
        request: Request<GetJobRequest>,
    ) -> Result<Response<Job>, Status> {
        let req = request.into_inner();
        
        let mut conn = self.state.db.get()
            .await
            .map_err(|e| Status::internal(format!("Database connection error: {}", e)))?;
        
        let job_id = Uuid::parse_str(&req.job_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid job ID: {}", e)))?;
        
        let job = sqlx::query!(
            r#"
            SELECT id, job_type, status, user_id, payload, result, error_message, 
                   created_at, started_at, completed_at, metadata
            FROM jobs
            WHERE id = $1
            "#,
            job_id
        )
        .fetch_optional(&mut *conn)
        .await
        .map_err(|e| Status::internal(format!("Database query error: {}", e)))?;
        
        let job = job.ok_or_else(|| Status::not_found("Job not found"))?;
        
        // Convert database job to protobuf job
        let job_type = match job.job_type.as_str() {
            "financial_forecast" => JobType::FinancialForecast,
            "sensitivity_analysis" => JobType::SensitivityAnalysis,
            _ => JobType::DataProcessing,
        };
        
        let status = match job.status.as_str() {
            "pending" => JobStatus::Pending,
            "running" => JobStatus::Running,
            "completed" => JobStatus::Completed,
            "failed" => JobStatus::Failed,
            "cancelled" => JobStatus::Cancelled,
            _ => JobStatus::Pending,
        };
        
        let payload = serde_json::to_vec(&job.payload)
            .map_err(|e| Status::internal(format!("Serialization error: {}", e)))?;
        
        let result = job.result.map(|r| serde_json::to_vec(&r))
            .transpose()
            .map_err(|e| Status::internal(format!("Serialization error: {}", e)))?
            .unwrap_or_default();
        
        Ok(Response::new(Job {
            job_id: job.id.to_string(),
            job_type: job_type as i32,
            status: status as i32,
            user_id: job.user_id.to_string(),
            created_at: job.created_at.to_rfc3339(),
            started_at: job.started_at.map(|d| d.to_rfc3339()).unwrap_or_default(),
            completed_at: job.completed_at.map(|d| d.to_rfc3339()).unwrap_or_default(),
            payload,
            error_message: job.error_message.unwrap_or_default(),
            metadata: job.metadata.as_object()
                .map(|obj| obj.iter().map(|(k, v)| (k.clone(), v.to_string())).collect())
                .unwrap_or_default(),
        }))
    }
}

/// Helper function to dispatch a job to available workers
pub async fn dispatch_job(
    state: &JobServerState,
    job: Job,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let subscribers = state.subscribers.read().await;
    
    // Find a suitable worker (for now, just use the first available)
    if let Some((worker_id, sender)) = subscribers.iter().next() {
        tracing::info!("Dispatching job {} to worker {}", job.job_id, worker_id);
        
        sender.send(job).await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
    } else {
        tracing::warn!("No available workers to dispatch job {}", job.job_id);
    }
    
    Ok(())
}

/// Helper function to create a new job in the database
pub async fn create_job(
    db: &DbPool,
    job_type: &str,
    user_id: Uuid,
    payload: serde_json::Value,
    metadata: Option<serde_json::Value>,
) -> Result<Uuid, Box<dyn std::error::Error + Send + Sync>> {
    let mut conn = db.get().await?;
    
    let job_id = sqlx::query!(
        r#"
        INSERT INTO jobs (job_type, user_id, payload, metadata)
        VALUES ($1, $2, $3, $4)
        RETURNING id
        "#,
        job_type,
        user_id,
        payload,
        metadata.unwrap_or_else(|| json!({}))
    )
    .fetch_one(&mut *conn)
    .await?
    .id;
    
    Ok(job_id)
}