//! Error handling for the CPC Messenger UI

use std::fmt;

/// Error types for the messenger UI
#[derive(Debug, Clone)]
pub enum MessengerError {
    /// Network error
    NetworkError(String),
    
    /// Authentication error
    AuthenticationError(String),
    
    /// Validation error
    ValidationError(String),
    
    /// Server error
    ServerError(String),
    
    /// WebSocket connection error
    WebSocketError(String),
    
    /// GraphQL error
    GraphQLError(String),
    
    /// Transaction rollback error
    TransactionError(String),
}

impl fmt::Display for MessengerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessengerError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            MessengerError::AuthenticationError(msg) => write!(f, "Authentication error: {}", msg),
            MessengerError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            MessengerError::ServerError(msg) => write!(f, "Server error: {}", msg),
            MessengerError::WebSocketError(msg) => write!(f, "WebSocket error: {}", msg),
            MessengerError::GraphQLError(msg) => write!(f, "GraphQL error: {}", msg),
            MessengerError::TransactionError(msg) => write!(f, "Transaction error: {}", msg),
        }
    }
}

impl std::error::Error for MessengerError {}

/// Error handler for managing errors in the UI
pub struct ErrorHandler {
    /// Last error that occurred
    last_error: Option<MessengerError>,
    
    /// Whether to show error notifications to the user
    show_notifications: bool,
}

impl ErrorHandler {
    /// Create a new error handler
    pub fn new(show_notifications: bool) -> Self {
        Self {
            last_error: None,
            show_notifications,
        }
    }
    
    /// Handle an error
    pub fn handle_error(&mut self, error: MessengerError) {
        // Log the error
        log::error!("Messenger error: {}", error);
        
        // Store the error
        self.last_error = Some(error.clone());
        
        // Show notification if enabled
        if self.show_notifications {
            self.show_error_notification(&error);
        }
    }
    
    /// Get the last error
    pub fn last_error(&self) -> Option<&MessengerError> {
        self.last_error.as_ref()
    }
    
    /// Clear the last error
    pub fn clear_error(&mut self) {
        self.last_error = None;
    }
    
    /// Show an error notification to the user
    fn show_error_notification(&self, error: &MessengerError) {
        // In a real implementation, this would show a toast notification or similar
        // For now, we'll just log it
        log::warn!("Error notification: {}", error);
    }
    
    /// Handle a network error with retry logic
    pub async fn handle_network_error_with_retry<F, Fut>(
        &mut self,
        operation: F,
        max_retries: usize,
    ) -> Result<(), MessengerError>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<(), MessengerError>>,
    {
        let mut retries = 0;
        
        loop {
            match operation().await {
                Ok(()) => return Ok(()),
                Err(MessengerError::NetworkError(_)) if retries < max_retries => {
                    retries += 1;
                    log::warn!("Network error, retrying... ({}/{})", retries, max_retries);
                    // Wait before retrying
                    tokio::time::sleep(tokio::time::Duration::from_millis(100 * retries as u64)).await;
                }
                Err(e) => {
                    self.handle_error(e.clone());
                    return Err(e);
                }
            }
        }
    }
    
    /// Rollback a transaction
    pub fn rollback_transaction(&self, transaction_id: &str) -> Result<(), MessengerError> {
        // In a real implementation, this would rollback the transaction
        // For now, we'll just log it
        log::info!("Rolling back transaction: {}", transaction_id);
        Ok(())
    }
}

impl Default for ErrorHandler {
    fn default() -> Self {
        Self::new(true)
    }
}

/// Connection error recovery strategies
pub enum ConnectionRecoveryStrategy {
    /// Retry immediately
    Immediate,
    
    /// Retry with exponential backoff
    ExponentialBackoff,
    
    /// Retry after a fixed delay
    FixedDelay(std::time::Duration),
}

/// Connection error recovery handler
pub struct ConnectionRecoveryHandler {
    strategy: ConnectionRecoveryStrategy,
    max_attempts: usize,
}

impl ConnectionRecoveryHandler {
    /// Create a new connection recovery handler
    pub fn new(strategy: ConnectionRecoveryStrategy, max_attempts: usize) -> Self {
        Self {
            strategy,
            max_attempts,
        }
    }
    
    /// Attempt to recover from a connection error
    pub async fn recover_from_error<F, Fut, T>(
        &self,
        operation: F,
    ) -> Result<T, MessengerError>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T, MessengerError>>,
    {
        let mut attempts = 0;
        
        loop {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    attempts += 1;
                    
                    if attempts >= self.max_attempts {
                        return Err(e);
                    }
                    
                    match &self.strategy {
                        ConnectionRecoveryStrategy::Immediate => {
                            // Retry immediately
                        }
                        ConnectionRecoveryStrategy::ExponentialBackoff => {
                            let delay = std::time::Duration::from_millis(2_u64.pow(attempts as u32) * 100);
                            tokio::time::sleep(delay).await;
                        }
                        ConnectionRecoveryStrategy::FixedDelay(duration) => {
                            tokio::time::sleep(*duration).await;
                        }
                    }
                }
            }
        }
    }
}