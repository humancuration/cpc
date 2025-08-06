//! Error reporting utilities
//!
//! This module provides utilities for reporting errors to monitoring services
//! and for collecting error statistics.

use crate::utils::error_handling::WebError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::JsValue;
use web_sys::console;

/// Error report structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorReport {
    /// Unique identifier for the error report
    pub id: String,
    
    /// The error that occurred
    pub error: WebError,
    
    /// Timestamp when the error occurred
    pub timestamp: u64,
    
    /// Component or module where the error occurred
    pub source: String,
    
    /// Additional context information
    pub context: HashMap<String, String>,
    
    /// User agent information
    pub user_agent: Option<String>,
    
    /// URL where the error occurred
    pub url: Option<String>,
}

/// Error reporting configuration
#[derive(Debug, Clone)]
pub struct ErrorReportingConfig {
    /// Whether error reporting is enabled
    pub enabled: bool,
    
    /// Endpoint to send error reports to
    pub endpoint: Option<String>,
    
    /// Maximum number of errors to report per session
    pub max_reports_per_session: usize,
    
    /// Whether to include user agent information
    pub include_user_agent: bool,
    
    /// Whether to include URL information
    pub include_url: bool,
}

impl Default for ErrorReportingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            endpoint: None,
            max_reports_per_session: 10,
            include_user_agent: true,
            include_url: true,
        }
    }
}

/// Error reporter service
pub struct ErrorReporter {
    /// Configuration for error reporting
    config: ErrorReportingConfig,
    
    /// Number of errors reported in the current session
    reported_count: usize,
    
    /// Collected error statistics
    stats: ErrorStatistics,
}

/// Error statistics
#[derive(Debug, Clone, Default)]
pub struct ErrorStatistics {
    /// Count of each error type
    pub error_counts: HashMap<String, usize>,
    
    /// Most common error sources
    pub common_sources: HashMap<String, usize>,
}

impl ErrorReporter {
    /// Create a new error reporter with the given configuration
    pub fn new(config: ErrorReportingConfig) -> Self {
        Self {
            config,
            reported_count: 0,
            stats: ErrorStatistics::default(),
        }
    }
    
    /// Report an error
    pub async fn report_error(&mut self, error: WebError, source: &str, context: HashMap<String, String>) {
        if !self.config.enabled {
            return;
        }
        
        if self.reported_count >= self.config.max_reports_per_session {
            console::warn_1(&"Maximum error reports per session reached".into());
            return;
        }
        
        // Update statistics
        let error_type = format!("{:?}", error);
        *self.stats.error_counts.entry(error_type).or_insert(0) += 1;
        *self.stats.common_sources.entry(source.to_string()).or_insert(0) += 1;
        
        // Create error report
        let report = ErrorReport {
            id: uuid::Uuid::new_v4().to_string(),
            error,
            timestamp: self.current_timestamp(),
            source: source.to_string(),
            context,
            user_agent: if self.config.include_user_agent {
                self.get_user_agent()
            } else {
                None
            },
            url: if self.config.include_url {
                self.get_current_url()
            } else {
                None
            },
        };
        
        // Send report if endpoint is configured
        if let Some(endpoint) = &self.config.endpoint {
            if let Err(e) = self.send_report(&report, endpoint).await {
                console::error_1(&format!("Failed to send error report: {:?}", e).into());
            }
        }
        
        // Log the error locally
        console::error_1(&format!("Error reported: {:?}", report).into());
        
        self.reported_count += 1;
    }
    
    /// Get error statistics
    pub fn get_statistics(&self) -> &ErrorStatistics {
        &self.stats
    }
    
    /// Reset error statistics
    pub fn reset_statistics(&mut self) {
        self.stats = ErrorStatistics::default();
    }
    
    /// Get the number of errors reported in the current session
    pub fn reported_count(&self) -> usize {
        self.reported_count
    }
    
    /// Send an error report to the configured endpoint
    async fn send_report(&self, report: &ErrorReport, endpoint: &str) -> Result<(), JsValue> {
        let client = gloo_net::http::Request::post(endpoint);
        
        let body = serde_json::to_string(report)
            .map_err(|e| JsValue::from_str(&format!("Failed to serialize error report: {:?}", e)))?;
        
        let _response = client
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .map_err(|e| JsValue::from_str(&format!("Failed to send error report: {:?}", e)))?;
        
        Ok(())
    }
    
    /// Get the current timestamp in milliseconds
    fn current_timestamp(&self) -> u64 {
        // In a real implementation, we would use:
        // web_sys::window().unwrap().performance().unwrap().now() as u64
        // For now, we'll use a mock timestamp
        0
    }
    
    /// Get the current user agent
    fn get_user_agent(&self) -> Option<String> {
        web_sys::window()
            .and_then(|w| w.navigator().user_agent().ok())
    }
    
    /// Get the current URL
    fn get_current_url(&self) -> Option<String> {
        web_sys::window()
            .and_then(|w| w.location().href().ok())
    }
}

impl Default for ErrorReporter {
    fn default() -> Self {
        Self::new(ErrorReportingConfig::default())
    }
}

/// Hook for reporting errors in components
#[derive(Debug, Clone)]
pub struct UseErrorReporting {
    /// The error reporter instance
    reporter: std::rc::Rc<std::cell::RefCell<ErrorReporter>>,
}

impl UseErrorReporting {
    /// Create a new error reporting hook
    pub fn new(reporter: std::rc::Rc<std::cell::RefCell<ErrorReporter>>) -> Self {
        Self { reporter }
    }
    
    /// Report an error with context
    pub async fn report_error_with_context(
        &self,
        error: WebError,
        source: &str,
        context: HashMap<String, String>,
    ) {
        self.reporter.borrow_mut().report_error(error, source, context).await;
    }
    
    /// Report an error without context
    pub async fn report_error(&self, error: WebError, source: &str) {
        self.report_error_with_context(error, source, HashMap::new()).await;
    }
    
    /// Get error statistics
    pub fn get_statistics(&self) -> ErrorStatistics {
        self.reporter.borrow().get_statistics().clone()
    }
}

/// Create an error reporting hook
pub fn use_error_reporting() -> UseErrorReporting {
    // In a real implementation, this would use Yew's hook system
    // For now, we'll create a simple instance
    let reporter = std::rc::Rc::new(std::cell::RefCell::new(ErrorReporter::default()));
    UseErrorReporting::new(reporter)
}

/// Trait for services that can report errors
pub trait ErrorReportingService {
    /// Report an error
    async fn report_error(&self, error: WebError, source: &str, context: HashMap<String, String>);
    
    /// Get error statistics
    fn get_error_statistics(&self) -> ErrorStatistics;
}