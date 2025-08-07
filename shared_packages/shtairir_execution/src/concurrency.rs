//! Concurrency control for Shtairir execution
//!
//! This module manages concurrent execution of independent nodes
//! while maintaining deterministic behavior.

use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

/// Concurrency controller for managing parallel execution
pub struct ConcurrencyController {
    /// Maximum number of concurrent tasks allowed
    max_concurrent_tasks: usize,
    /// Semaphore to limit concurrent tasks
    semaphore: Arc<Semaphore>,
    /// Active tasks tracker
    active_tasks: Arc<Mutex<HashMap<String, usize>>>,
}

impl ConcurrencyController {
    /// Create a new concurrency controller with default settings
    pub fn new() -> Self {
        Self::with_max_concurrent_tasks(8) // Default to 8 concurrent tasks
    }
    
    /// Create a new concurrency controller with specified maximum concurrent tasks
    pub fn with_max_concurrent_tasks(max_concurrent_tasks: usize) -> Self {
        Self {
            max_concurrent_tasks,
            semaphore: Arc::new(Semaphore::new(max_concurrent_tasks)),
            active_tasks: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    /// Get the maximum number of concurrent tasks
    pub fn max_concurrent_tasks(&self) -> usize {
        self.max_concurrent_tasks
    }
    
    /// Set the maximum number of concurrent tasks
    pub fn set_max_concurrent_tasks(&mut self, max_concurrent_tasks: usize) {
        self.max_concurrent_tasks = max_concurrent_tasks;
        // Note: We don't update the semaphore capacity here as it's not trivial
        // In a real implementation, we might want to recreate the semaphore
    }
    
    /// Execute a set of independent tasks concurrently
    pub async fn execute_concurrent<T, F, Fut>(
        &self,
        tasks: Vec<(String, F)>,
    ) -> Result<Vec<(String, T)>, tokio::task::JoinError>
    where
        T: Send + 'static,
        F: FnOnce() -> Fut + Send + 'static,
        Fut: std::future::Future<Output = T> + Send,
    {
        let mut join_set = JoinSet::new();
        let mut task_names = Vec::new();
        
        // Acquire permits for all tasks
        let permits: Vec<_> = futures_util::future::join_all(
            (0..tasks.len()).map(|_| self.semaphore.clone().acquire_owned())
        ).await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| tokio::task::JoinError::cancelled(futures_util::future::pending().boxed()))?;
        
        // Spawn all tasks
        for (permit, (task_name, task_fn)) in permits.into_iter().zip(tasks) {
            task_names.push(task_name.clone());
            join_set.spawn(async move {
                let result = task_fn().await;
                drop(permit); // Release the permit when task completes
                (task_name, result)
            });
        }
        
        // Collect results
        let mut results = Vec::new();
        while let Some(result) = join_set.join_next().await {
            results.push(result?);
        }
        
        Ok(results)
    }
    
    /// Track an active task
    pub fn track_task(&self, task_id: String) {
        let mut active_tasks = self.active_tasks.lock().unwrap();
        let count = active_tasks.entry(task_id).or_insert(0);
        *count += 1;
    }
    
    /// Untrack a completed task
    pub fn untrack_task(&self, task_id: &str) {
        let mut active_tasks = self.active_tasks.lock().unwrap();
        if let Some(count) = active_tasks.get_mut(task_id) {
            *count -= 1;
            if *count == 0 {
                active_tasks.remove(task_id);
            }
        }
    }
    
    /// Get the number of currently active tasks
    pub fn active_task_count(&self) -> usize {
        let active_tasks = self.active_tasks.lock().unwrap();
        active_tasks.len()
    }
}

impl Default for ConcurrencyController {
    fn default() -> Self {
        Self::new()
    }
}