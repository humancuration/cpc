use std::sync::Arc;
use tokio::sync::mpsc;
use futures::StreamExt;
use uuid::Uuid;
use cpc_core::models::social::post::{MediaType, ProcessingStatus};
use crate::models::media::{MediaProcessingJob, ProcessingResult};
use crate::services::media_service::{MediaProcessingEvent, MediaServiceError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GrpcAdapterError {
    #[error("gRPC connection error: {0}")]
    Connection(String),
    #[error("gRPC request error: {0}")]
    Request(String),
    #[error("Timeout error: {0}")]
    Timeout(String),
    #[error("Invalid response: {0}")]
    InvalidResponse(String),
}

pub struct GrpcMediaProcessor {
    client: Arc<crate::grpc::media_processor_client::MediaProcessorClient>,
    retry_config: RetryConfig,
    timeout_config: TimeoutConfig,
    event_tx: mpsc::UnboundedSender<MediaProcessingEvent>,
}

#[derive(Clone)]
pub struct RetryConfig {
    pub max_attempts: u32,
    pub initial_delay_ms: u64,
    pub max_delay_ms: u64,
    pub backoff_multiplier: f64,
}

#[derive(Clone)]
pub struct TimeoutConfig {
    pub connection_timeout: std::time::Duration,
    pub request_timeout: std::time::Duration,
    pub streaming_timeout: std::time::Duration,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay_ms: 1000,
            max_delay_ms: 10000,
            backoff_multiplier: 2.0,
        }
    }
}

impl Default for TimeoutConfig {
    fn default() -> Self {
        Self {
            connection_timeout: std::time::Duration::from_secs(10),
            request_timeout: std::time::Duration::from_secs(30),
            streaming_timeout: std::time::Duration::from_secs(300),
        }
    }
}

impl GrpcMediaProcessor {
    pub fn new(
        client: Arc<crate::grpc::media_processor_client::MediaProcessorClient>,
        event_tx: mpsc::UnboundedSender<MediaProcessingEvent>,
    ) -> Self {
        Self {
            client,
            retry_config: RetryConfig::default(),
            timeout_config: TimeoutConfig::default(),
            event_tx,
        }
    }

    pub fn with_retry_config(mut self, config: RetryConfig) -> Self {
        self.retry_config = config;
        self
    }

    pub fn with_timeout_config(mut self, config: TimeoutConfig) -> Self {
        self.timeout_config = config;
        self
    }

    pub async fn process_media(
        &self,
        job: MediaProcessingJob,
    ) -> Result<ProcessingResult, GrpcAdapterError> {
        let mut attempt = 0;
        let mut delay = std::time::Duration::from_millis(self.retry_config.initial_delay_ms);

        loop {
            attempt += 1;
            
            match self.process_media_internal(&job).await {
                Ok(result) => return Ok(result),
                Err(e) if attempt >= self.retry_config.max_attempts => {
                    log::error!("Max retry attempts reached for media processing: {:?}", e);
                    return Err(e);
                }
                Err(e) => {
                    log::warn!("Media processing attempt {} failed: {:?}. Retrying...", attempt, e);
                    
                    // Send retry event
                    self.event_tx.send(MediaProcessingEvent::StatusChanged {
                        media_id: job.media_id,
                        status: ProcessingStatus::Processing,
                        progress: Some(0),
                    }).ok();

                    tokio::time::sleep(delay).await;
                    delay = std::cmp::min(
                        std::time::Duration::from_millis(
                            (delay.as_millis() as f64 * self.retry_config.backoff_multiplier) as u64
                        ),
                        std::time::Duration::from_millis(self.retry_config.max_delay_ms),
                    );
                }
            }
        }
    }

    async fn process_media_internal(
        &self,
        job: &MediaProcessingJob,
    ) -> Result<ProcessingResult, GrpcAdapterError> {
        // Create gRPC request
        let request = crate::grpc::ProcessMediaRequest {
            media_id: job.media_id.to_string(),
            source_url: job.source_url.clone(),
            media_type: match job.media_type {
                MediaType::Image => "image".to_string(),
                MediaType::Video => "video".to_string(),
                MediaType::Audio => "audio".to_string(),
                MediaType::Document => "document".to_string(),
                MediaType::Unknown => "unknown".to_string(),
            },
            target_formats: job.target_formats.clone(),
            quality: job.quality.clone(),
        };

        // Set timeout
        let timeout = tokio::time::timeout(
            self.timeout_config.request_timeout,
            self.client.process_media(request),
        );

        match timeout.await {
            Ok(Ok(response)) => {
                let processing_result = ProcessingResult {
                    media_id: job.media_id,
                    processed_url: response.processed_url,
                    thumbnail_url: response.thumbnail_url,
                    formats: response.formats,
                    metadata: response.metadata,
                    processing_time_ms: response.processing_time_ms,
                };

                // Send completion event
                self.event_tx.send(MediaProcessingEvent::ProcessingCompleted {
                    media_id: job.media_id,
                    result_url: processing_result.processed_url.clone(),
                }).ok();

                Ok(processing_result)
            }
            Ok(Err(e)) => {
                let error_msg = format!("gRPC processing error: {:?}", e);
                self.event_tx.send(MediaProcessingEvent::ProcessingFailed {
                    media_id: job.media_id,
                    error: error_msg.clone(),
                }).ok();
                Err(GrpcAdapterError::Request(error_msg))
            }
            Err(_) => {
                let error_msg = "Request timeout".to_string();
                self.event_tx.send(MediaProcessingEvent::ProcessingFailed {
                    media_id: job.media_id,
                    error: error_msg.clone(),
                }).ok();
                Err(GrpcAdapterError::Timeout(error_msg))
            }
        }
    }

    pub async fn process_media_streaming(
        &self,
        job: MediaProcessingJob,
    ) -> Result<(), GrpcAdapterError> {
        let request = crate::grpc::ProcessMediaRequest {
            media_id: job.media_id.to_string(),
            source_url: job.source_url.clone(),
            media_type: match job.media_type {
                MediaType::Image => "image".to_string(),
                MediaType::Video => "video".to_string(),
                MediaType::Audio => "audio".to_string(),
                MediaType::Document => "document".to_string(),
                MediaType::Unknown => "unknown".to_string(),
            },
            target_formats: job.target_formats.clone(),
            quality: job.quality.clone(),
        };

        let mut stream = match self.client.process_media_streaming(request).await {
            Ok(stream) => stream,
            Err(e) => {
                return Err(GrpcAdapterError::Request(format!("Failed to start streaming: {:?}", e)));
            }
        };

        let timeout = tokio::time::timeout(
            self.timeout_config.streaming_timeout,
            self.handle_streaming_updates(&mut stream, job.media_id)
        );

        match timeout.await {
            Ok(result) => result,
            Err(_) => {
                let error_msg = "Streaming timeout".to_string();
                self.event_tx.send(MediaProcessingEvent::ProcessingFailed {
                    media_id: job.media_id,
                    error: error_msg.clone(),
                }).ok();
                Err(GrpcAdapterError::Timeout(error_msg))
            }
        }
    }

    async fn handle_streaming_updates(
        &self,
        stream: &mut (impl futures::Stream<Item = Result<crate::grpc::MediaProcessingResponse, tonic::Status>> + Unpin),
        media_id: Uuid,
    ) -> Result<(), GrpcAdapterError> {
        while let Some(response) = stream.next().await {
            match response {
                Ok(update) => {
                    let status = match update.status.as_str() {
                        "processing" => ProcessingStatus::Processing,
                        "completed" => ProcessingStatus::Completed,
                        "failed" => ProcessingStatus::Failed,
                        _ => ProcessingStatus::Pending,
                    };

                    self.event_tx.send(MediaProcessingEvent::StatusChanged {
                        media_id,
                        status,
                        progress: Some(update.progress as u32),
                    }).ok();

                    if update.status == "completed" {
                        self.event_tx.send(MediaProcessingEvent::ProcessingCompleted {
                            media_id,
                            result_url: update.processed_url.clone(),
                        }).ok();
                        break;
                    } else if update.status == "failed" {
                        self.event_tx.send(MediaProcessingEvent::ProcessingFailed {
                            media_id,
                            error: update.error.unwrap_or_else(|| "Unknown error".to_string()),
                        }).ok();
                        return Err(GrpcAdapterError::Request("Processing failed".to_string()));
                    }
                }
                Err(e) => {
                    let error_msg = format!("Streaming error: {:?}", e);
                    self.event_tx.send(MediaProcessingEvent::ProcessingFailed {
                        media_id,
                        error: error_msg.clone(),
                    }).ok();
                    return Err(GrpcAdapterError::Request(error_msg));
                }
            }
        }

        Ok(())
    }

    pub async fn get_processing_status(
        &self,
        media_id: Uuid,
    ) -> Result<ProcessingStatus, GrpcAdapterError> {
        let request = crate::grpc::GetProcessingStatusRequest {
            media_id: media_id.to_string(),
        };

        let response = tokio::time::timeout(
            self.timeout_config.request_timeout,
            self.client.get_processing_status(request),
        ).await
            .map_err(|_| GrpcAdapterError::Timeout("Status check timeout".to_string()))?
            .map_err(|e| GrpcAdapterError::Request(format!("Status check error: {:?}", e)))?;

        let status = match response.status.as_str() {
            "pending" => ProcessingStatus::Pending,
            "processing" => ProcessingStatus::Processing,
            "completed" => ProcessingStatus::Completed,
            "failed" => ProcessingStatus::Failed,
            _ => ProcessingStatus::Pending,
        };

        Ok(status)
    }
}