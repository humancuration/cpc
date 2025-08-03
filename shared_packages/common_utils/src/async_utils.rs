//! Async utilities for the CPC platform
//!
//! This module provides common async utilities and patterns for handling
//! asynchronous operations, timeouts, retries, and concurrency.

use std::future::Future;
use std::time::Duration;
use tokio::time::timeout;
use crate::error::{CommonError, Result};

/// Retry a function with exponential backoff
pub async fn retry<F, Fut, T, E>(
    mut f: F,
    max_attempts: usize,
    initial_delay: Duration,
) -> Result<T>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = std::result::Result<T, E>>,
    E: std::error::Error + Send + Sync + 'static,
{
    let mut delay = initial_delay;
    let mut last_error = None;
    
    for attempt in 1..=max_attempts {
        match f().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                last_error = Some(e);
                if attempt < max_attempts {
                    tokio::time::sleep(delay).await;
                    delay *= 2; // Exponential backoff
                }
            }
        }
    }
    
    match last_error {
        Some(e) => Err(CommonError::Generic(format!("Operation failed after {} attempts: {}", max_attempts, e))),
        None => Err(CommonError::Generic("Operation failed with no error".to_string())),
    }
}

/// Execute a future with a timeout
pub async fn with_timeout<F, T>(
    future: F,
    timeout_duration: Duration,
) -> Result<T>
where
    F: Future<Output = Result<T>>,
{
    match timeout(timeout_duration, future).await {
        Ok(result) => result,
        Err(_) => Err(CommonError::Timeout),
    }
}

/// Execute a function with a timeout, returning a default value on timeout
pub async fn with_timeout_or_default<F, T>(
    future: F,
    timeout_duration: Duration,
    default: T,
) -> T
where
    F: Future<Output = T>,
    T: Clone,
{
    match timeout(timeout_duration, future).await {
        Ok(result) => result,
        Err(_) => default,
    }
}

/// Run multiple futures concurrently and collect their results
pub async fn join_all<T>(
    futures: Vec<impl Future<Output = Result<T>>>,
) -> Result<Vec<T>> {
    let results = futures::future::join_all(futures).await;
    let mut success_results = Vec::new();
    
    for result in results {
        match result {
            Ok(value) => success_results.push(value),
            Err(e) => return Err(e),
        }
    }
    
    Ok(success_results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_retry_success() {
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();
        
        let result = retry(
            || {
                let counter = counter_clone.clone();
                async move {
                    let count = counter.fetch_add(1, Ordering::SeqCst);
                    if count < 2 {
                        Err::<(), CommonError>(CommonError::Generic("Temporary error".to_string()))
                    } else {
                        Ok(())
                    }
                }
            },
            5,
            Duration::from_millis(10),
        ).await;
        
        assert!(result.is_ok());
        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }
    
    #[tokio::test]
    async fn test_retry_failure() {
        let result = retry(
            || async { Err::<(), CommonError>(CommonError::Generic("Permanent error".to_string())) },
            3,
            Duration::from_millis(10),
        ).await;
        
        assert!(result.is_err());
    }
    
    #[tokio::test]
    async fn test_with_timeout_success() {
        let future = async {
            tokio::time::sleep(Duration::from_millis(10)).await;
            Ok("success")
        };
        
        let result = with_timeout(future, Duration::from_millis(100)).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "success");
    }
    
    #[tokio::test]
    async fn test_with_timeout_failure() {
        let future = async {
            tokio::time::sleep(Duration::from_millis(100)).await;
            Ok("success")
        };
        
        let result = with_timeout(future, Duration::from_millis(10)).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), CommonError::Timeout));
    }
}