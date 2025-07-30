//! Monitoring service for the API & Integration Hub module

use async_trait::async_trait;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::{
    domain::api_endpoint::HttpMethod,
    application::api_management::ApiManagementError,
};
use thiserror::Error;
use tracing::{info, warn, error, debug};

/// Error types for monitoring operations
#[derive(Error, Debug)]
pub enum MonitoringError {
    #[error("Storage error: {0}")]
    StorageError(String),
    
    #[error("Invalid data: {0}")]
    InvalidData(String),
    
    #[error("Query error: {0}")]
    QueryError(String),
}

/// API call log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiCallLog {
    pub id: Uuid,
    pub api_endpoint_id: Uuid,
    pub user_id: Option<Uuid>,
    pub http_method: HttpMethod,
    pub request_path: String,
    pub request_headers: serde_json::Value,
    pub request_body: Option<serde_json::Value>,
    pub response_status: u16,
    pub response_body: Option<serde_json::Value>,
    pub response_time_ms: u64,
    pub timestamp: DateTime<Utc>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

/// API usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiUsageStats {
    pub total_calls: u64,
    pub successful_calls: u64,
    pub failed_calls: u64,
    pub average_response_time_ms: f64,
    pub calls_by_status: std::collections::HashMap<u16, u64>,
    pub calls_by_method: std::collections::HashMap<String, u64>,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
}

/// Monitoring service
pub struct MonitoringService<R: MonitoringRepository> {
    repository: R,
}

impl<R: MonitoringRepository> MonitoringService<R> {
    /// Create a new monitoring service
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
    
    /// Log an API call
    pub async fn log_api_call(&self, log_entry: ApiCallLog) -> Result<(), MonitoringError> {
        debug!("Logging API call: {}", log_entry.id);
        
        self.repository.save_api_call_log(&log_entry)
            .await
            .map_err(|e| {
                error!("Failed to save API call log: {}", e);
                MonitoringError::StorageError(e.to_string())
            })?;
        
        info!("Successfully logged API call: {}", log_entry.id);
        Ok(())
    }
    
    /// Get API usage statistics for a specific endpoint
    pub async fn get_endpoint_usage_stats(
        &self,
        endpoint_id: Uuid,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<ApiUsageStats, MonitoringError> {
        info!("Getting usage stats for endpoint: {} from {} to {}", 
              endpoint_id, start_time, end_time);
        
        let logs = self.repository.get_api_call_logs_by_endpoint(endpoint_id, start_time, end_time)
            .await
            .map_err(|e| {
                error!("Failed to get API call logs for endpoint {}: {}", endpoint_id, e);
                MonitoringError::QueryError(e.to_string())
            })?;
        
        let stats = self.calculate_usage_stats(&logs, start_time, end_time);
        Ok(stats)
    }
    
    /// Get API usage statistics for a specific user
    pub async fn get_user_usage_stats(
        &self,
        user_id: Uuid,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<ApiUsageStats, MonitoringError> {
        info!("Getting usage stats for user: {} from {} to {}", 
              user_id, start_time, end_time);
        
        let logs = self.repository.get_api_call_logs_by_user(user_id, start_time, end_time)
            .await
            .map_err(|e| {
                error!("Failed to get API call logs for user {}: {}", user_id, e);
                MonitoringError::QueryError(e.to_string())
            })?;
        
        let stats = self.calculate_usage_stats(&logs, start_time, end_time);
        Ok(stats)
    }
    
    /// Get error rate statistics
    pub async fn get_error_rate_stats(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<std::collections::HashMap<Uuid, f64>, MonitoringError> {
        info!("Getting error rate stats from {} to {}", start_time, end_time);
        
        let logs = self.repository.get_api_call_logs_by_time_range(start_time, end_time)
            .await
            .map_err(|e| {
                error!("Failed to get API call logs: {}", e);
                MonitoringError::QueryError(e.to_string())
            })?;
        
        let mut error_rates = std::collections::HashMap::new();
        
        // Group logs by endpoint
        let mut logs_by_endpoint: std::collections::HashMap<Uuid, Vec<ApiCallLog>> = std::collections::HashMap::new();
        for log in logs {
            logs_by_endpoint.entry(log.api_endpoint_id).or_insert_with(Vec::new).push(log);
        }
        
        // Calculate error rate for each endpoint
        for (endpoint_id, endpoint_logs) in logs_by_endpoint {
            let total_calls = endpoint_logs.len() as f64;
            let error_calls = endpoint_logs.iter().filter(|log| log.response_status >= 400).count() as f64;
            
            let error_rate = if total_calls > 0.0 {
                error_calls / total_calls
            } else {
                0.0
            };
            
            error_rates.insert(endpoint_id, error_rate);
        }
        
        Ok(error_rates)
    }
    
    /// Get top API endpoints by call volume
    pub async fn get_top_endpoints_by_volume(
        &self,
        limit: usize,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<Vec<(Uuid, u64)>, MonitoringError> {
        info!("Getting top {} endpoints by volume from {} to {}", 
              limit, start_time, end_time);
        
        let logs = self.repository.get_api_call_logs_by_time_range(start_time, end_time)
            .await
            .map_err(|e| {
                error!("Failed to get API call logs: {}", e);
                MonitoringError::QueryError(e.to_string())
            })?;
        
        // Count calls by endpoint
        let mut call_counts: std::collections::HashMap<Uuid, u64> = std::collections::HashMap::new();
        for log in logs {
            *call_counts.entry(log.api_endpoint_id).or_insert(0) += 1;
        }
        
        // Sort by count and take top N
        let mut sorted_counts: Vec<(Uuid, u64)> = call_counts.into_iter().collect();
        sorted_counts.sort_by(|a, b| b.1.cmp(&a.1));
        sorted_counts.truncate(limit);
        
        Ok(sorted_counts)
    }
    
    /// Calculate usage statistics from log entries
    fn calculate_usage_stats(
        &self,
        logs: &[ApiCallLog],
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
    ) -> ApiUsageStats {
        let total_calls = logs.len() as u64;
        let mut successful_calls = 0;
        let mut failed_calls = 0;
        let mut total_response_time = 0u64;
        let mut calls_by_status = std::collections::HashMap::new();
        let mut calls_by_method = std::collections::HashMap::new();
        
        for log in logs {
            if log.response_status < 400 {
                successful_calls += 1;
            } else {
                failed_calls += 1;
            }
            
            total_response_time += log.response_time_ms;
            
            *calls_by_status.entry(log.response_status).or_insert(0) += 1;
            
            let method_str = match log.http_method {
                HttpMethod::GET => "GET".to_string(),
                HttpMethod::POST => "POST".to_string(),
                HttpMethod::PUT => "PUT".to_string(),
                HttpMethod::DELETE => "DELETE".to_string(),
                HttpMethod::PATCH => "PATCH".to_string(),
            };
            *calls_by_method.entry(method_str).or_insert(0) += 1;
        }
        
        let average_response_time_ms = if total_calls > 0 {
            total_response_time as f64 / total_calls as f64
        } else {
            0.0
        };
        
        ApiUsageStats {
            total_calls,
            successful_calls,
            failed_calls,
            average_response_time_ms,
            calls_by_status,
            calls_by_method,
            period_start,
            period_end,
        }
    }
    
    /// Get recent API call logs
    pub async fn get_recent_api_calls(
        &self,
        limit: usize,
    ) -> Result<Vec<ApiCallLog>, MonitoringError> {
        info!("Getting {} recent API call logs", limit);
        
        let logs = self.repository.get_recent_api_call_logs(limit)
            .await
            .map_err(|e| {
                error!("Failed to get recent API call logs: {}", e);
                MonitoringError::QueryError(e.to_string())
            })?;
        
        Ok(logs)
    }
    
    /// Get API call logs by status code range
    pub async fn get_api_calls_by_status_range(
        &self,
        min_status: u16,
        max_status: u16,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<Vec<ApiCallLog>, MonitoringError> {
        info!("Getting API call logs with status codes {}-{} from {} to {}", 
              min_status, max_status, start_time, end_time);
        
        let logs = self.repository.get_api_call_logs_by_status_range(min_status, max_status, start_time, end_time)
            .await
            .map_err(|e| {
                error!("Failed to get API call logs by status range: {}", e);
                MonitoringError::QueryError(e.to_string())
            })?;
        
        Ok(logs)
    }
}

/// Repository trait for monitoring data storage
#[async_trait]
pub trait MonitoringRepository: Send + Sync {
    /// Save an API call log entry
    async fn save_api_call_log(&self, log: &ApiCallLog) -> Result<(), MonitoringError>;
    
    /// Get API call logs by endpoint ID and time range
    async fn get_api_call_logs_by_endpoint(
        &self,
        endpoint_id: Uuid,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<Vec<ApiCallLog>, MonitoringError>;
    
    /// Get API call logs by user ID and time range
    async fn get_api_call_logs_by_user(
        &self,
        user_id: Uuid,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<Vec<ApiCallLog>, MonitoringError>;
    
    /// Get API call logs by time range
    async fn get_api_call_logs_by_time_range(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<Vec<ApiCallLog>, MonitoringError>;
    
    /// Get recent API call logs
    async fn get_recent_api_call_logs(&self, limit: usize) -> Result<Vec<ApiCallLog>, MonitoringError>;
    
    /// Get API call logs by status code range
    async fn get_api_call_logs_by_status_range(
        &self,
        min_status: u16,
        max_status: u16,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<Vec<ApiCallLog>, MonitoringError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::api_endpoint::HttpMethod;
    use serde_json::json;
    use chrono::Duration;
    
    // Mock monitoring repository for testing
    struct MockMonitoringRepository;
    
    #[async_trait]
    impl MonitoringRepository for MockMonitoringRepository {
        async fn save_api_call_log(&self, _log: &ApiCallLog) -> Result<(), MonitoringError> {
            Ok(())
        }
        
        async fn get_api_call_logs_by_endpoint(
            &self,
            _endpoint_id: Uuid,
            _start_time: DateTime<Utc>,
            _end_time: DateTime<Utc>,
        ) -> Result<Vec<ApiCallLog>, MonitoringError> {
            let user_id = Uuid::new_v4();
            let endpoint_id = Uuid::new_v4();
            
            let log = ApiCallLog {
                id: Uuid::new_v4(),
                api_endpoint_id: endpoint_id,
                user_id: Some(user_id),
                http_method: HttpMethod::GET,
                request_path: "/test".to_string(),
                request_headers: json!({}),
                request_body: None,
                response_status: 200,
                response_body: Some(json!({"result": "success"})),
                response_time_ms: 50,
                timestamp: Utc::now(),
                ip_address: Some("127.0.0.1".to_string()),
                user_agent: Some("test-agent".to_string()),
            };
            
            Ok(vec![log])
        }
        
        async fn get_api_call_logs_by_user(
            &self,
            _user_id: Uuid,
            _start_time: DateTime<Utc>,
            _end_time: DateTime<Utc>,
        ) -> Result<Vec<ApiCallLog>, MonitoringError> {
            let user_id = Uuid::new_v4();
            let endpoint_id = Uuid::new_v4();
            
            let log = ApiCallLog {
                id: Uuid::new_v4(),
                api_endpoint_id: endpoint_id,
                user_id: Some(user_id),
                http_method: HttpMethod::GET,
                request_path: "/test".to_string(),
                request_headers: json!({}),
                request_body: None,
                response_status: 200,
                response_body: Some(json!({"result": "success"})),
                response_time_ms: 50,
                timestamp: Utc::now(),
                ip_address: Some("127.0.0.1".to_string()),
                user_agent: Some("test-agent".to_string()),
            };
            
            Ok(vec![log])
        }
        
        async fn get_api_call_logs_by_time_range(
            &self,
            _start_time: DateTime<Utc>,
            _end_time: DateTime<Utc>,
        ) -> Result<Vec<ApiCallLog>, MonitoringError> {
            let user_id = Uuid::new_v4();
            let endpoint_id = Uuid::new_v4();
            
            let log = ApiCallLog {
                id: Uuid::new_v4(),
                api_endpoint_id: endpoint_id,
                user_id: Some(user_id),
                http_method: HttpMethod::GET,
                request_path: "/test".to_string(),
                request_headers: json!({}),
                request_body: None,
                response_status: 200,
                response_body: Some(json!({"result": "success"})),
                response_time_ms: 50,
                timestamp: Utc::now(),
                ip_address: Some("127.0.0.1".to_string()),
                user_agent: Some("test-agent".to_string()),
            };
            
            Ok(vec![log])
        }
        
        async fn get_recent_api_call_logs(&self, _limit: usize) -> Result<Vec<ApiCallLog>, MonitoringError> {
            let user_id = Uuid::new_v4();
            let endpoint_id = Uuid::new_v4();
            
            let log = ApiCallLog {
                id: Uuid::new_v4(),
                api_endpoint_id: endpoint_id,
                user_id: Some(user_id),
                http_method: HttpMethod::GET,
                request_path: "/test".to_string(),
                request_headers: json!({}),
                request_body: None,
                response_status: 200,
                response_body: Some(json!({"result": "success"})),
                response_time_ms: 50,
                timestamp: Utc::now(),
                ip_address: Some("127.0.0.1".to_string()),
                user_agent: Some("test-agent".to_string()),
            };
            
            Ok(vec![log])
        }
        
        async fn get_api_call_logs_by_status_range(
            &self,
            _min_status: u16,
            _max_status: u16,
            _start_time: DateTime<Utc>,
            _end_time: DateTime<Utc>,
        ) -> Result<Vec<ApiCallLog>, MonitoringError> {
            let user_id = Uuid::new_v4();
            let endpoint_id = Uuid::new_v4();
            
            let log = ApiCallLog {
                id: Uuid::new_v4(),
                api_endpoint_id: endpoint_id,
                user_id: Some(user_id),
                http_method: HttpMethod::GET,
                request_path: "/test".to_string(),
                request_headers: json!({}),
                request_body: None,
                response_status: 200,
                response_body: Some(json!({"result": "success"})),
                response_time_ms: 50,
                timestamp: Utc::now(),
                ip_address: Some("127.0.0.1".to_string()),
                user_agent: Some("test-agent".to_string()),
            };
            
            Ok(vec![log])
        }
    }
    
    #[tokio::test]
    async fn test_log_api_call() {
        let repository = MockMonitoringRepository;
        let service = MonitoringService::new(repository);
        
        let user_id = Uuid::new_v4();
        let endpoint_id = Uuid::new_v4();
        
        let log_entry = ApiCallLog {
            id: Uuid::new_v4(),
            api_endpoint_id: endpoint_id,
            user_id: Some(user_id),
            http_method: HttpMethod::GET,
            request_path: "/test".to_string(),
            request_headers: json!({}),
            request_body: None,
            response_status: 200,
            response_body: Some(json!({"result": "success"})),
            response_time_ms: 50,
            timestamp: Utc::now(),
            ip_address: Some("127.0.0.1".to_string()),
            user_agent: Some("test-agent".to_string()),
        };
        
        let result = service.log_api_call(log_entry).await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_get_endpoint_usage_stats() {
        let repository = MockMonitoringRepository;
        let service = MonitoringService::new(repository);
        
        let endpoint_id = Uuid::new_v4();
        let start_time = Utc::now() - Duration::hours(1);
        let end_time = Utc::now();
        
        let stats = service.get_endpoint_usage_stats(endpoint_id, start_time, end_time).await.unwrap();
        
        assert_eq!(stats.total_calls, 1);
        assert_eq!(stats.successful_calls, 1);
        assert_eq!(stats.failed_calls, 0);
        assert_eq!(stats.average_response_time_ms, 50.0);
    }
    
    #[tokio::test]
    async fn test_get_user_usage_stats() {
        let repository = MockMonitoringRepository;
        let service = MonitoringService::new(repository);
        
        let user_id = Uuid::new_v4();
        let start_time = Utc::now() - Duration::hours(1);
        let end_time = Utc::now();
        
        let stats = service.get_user_usage_stats(user_id, start_time, end_time).await.unwrap();
        
        assert_eq!(stats.total_calls, 1);
        assert_eq!(stats.successful_calls, 1);
        assert_eq!(stats.failed_calls, 0);
        assert_eq!(stats.average_response_time_ms, 50.0);
    }
    
    #[tokio::test]
    async fn test_calculate_usage_stats() {
        let repository = MockMonitoringRepository;
        let service = MonitoringService::new(repository);
        
        let user_id = Uuid::new_v4();
        let endpoint_id = Uuid::new_v4();
        
        let logs = vec![
            ApiCallLog {
                id: Uuid::new_v4(),
                api_endpoint_id: endpoint_id,
                user_id: Some(user_id),
                http_method: HttpMethod::GET,
                request_path: "/test".to_string(),
                request_headers: json!({}),
                request_body: None,
                response_status: 200,
                response_body: Some(json!({"result": "success"})),
                response_time_ms: 50,
                timestamp: Utc::now(),
                ip_address: Some("127.0.0.1".to_string()),
                user_agent: Some("test-agent".to_string()),
            },
            ApiCallLog {
                id: Uuid::new_v4(),
                api_endpoint_id: endpoint_id,
                user_id: Some(user_id),
                http_method: HttpMethod::POST,
                request_path: "/test".to_string(),
                request_headers: json!({}),
                request_body: None,
                response_status: 400,
                response_body: Some(json!({"error": "bad request"})),
                response_time_ms: 100,
                timestamp: Utc::now(),
                ip_address: Some("127.0.0.1".to_string()),
                user_agent: Some("test-agent".to_string()),
            }
        ];
        
        let start_time = Utc::now() - Duration::hours(1);
        let end_time = Utc::now();
        
        let stats = service.calculate_usage_stats(&logs, start_time, end_time);
        
        assert_eq!(stats.total_calls, 2);
        assert_eq!(stats.successful_calls, 1);
        assert_eq!(stats.failed_calls, 1);
        assert_eq!(stats.average_response_time_ms, 75.0);
        assert_eq!(stats.calls_by_status.get(&200), Some(&1));
        assert_eq!(stats.calls_by_status.get(&400), Some(&1));
        assert_eq!(stats.calls_by_method.get("GET"), Some(&1));
        assert_eq!(stats.calls_by_method.get("POST"), Some(&1));
    }
}