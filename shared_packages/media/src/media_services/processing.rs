//! Media processing service that integrates with the task manager
//! 
//! This module provides functionality for creating media processing tasks,
//! scheduling recurring jobs, and tracking progress through the task manager.

use uuid::Uuid;
use chrono::{DateTime, Utc};
use task_manager::task_core::{
    entities::Task,
    value_objects::{TaskStatus, ProgressPercentage, DependencyGraph, RecurrenceRule, RecurrencePattern},
    services::{RecurrenceGenerator, ProgressCalculator},
    errors::DomainError,
};

/// Error types for media processing
#[derive(Debug)]
pub enum ProcessingError {
    TaskError(DomainError),
    ProcessingFailed(String),
}

impl From<DomainError> for ProcessingError {
    fn from(error: DomainError) -> Self {
        ProcessingError::TaskError(error)
    }
}

/// Media processing service
pub struct MediaProcessingService;

impl MediaProcessingService {
    /// Create a processing task for a media asset
    /// 
    /// # Arguments
    /// * `asset_id` - UUID of the media asset to process
    /// * `title` - Title for the processing task
    /// * `description` - Description of the processing to perform
    /// 
    /// # Returns
    /// Result with the created Task on success, ProcessingError on failure
    pub async fn create_processing_task(
        asset_id: Uuid,
        title: String,
        description: String,
    ) -> Result<Task, ProcessingError> {
        let task = Task {
            id: Uuid::new_v4(),
            title,
            description,
            due_date: None,
            priority: task_manager::task_core::value_objects::TaskPriority::Medium,
            status: TaskStatus::NotStarted,
            progress: ProgressPercentage(0),
            assignee: None,
            dependencies: DependencyGraph::new(),
            recurrence: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        // In a real implementation, we would save the task to a repository
        // task_repo.save(task).await?;
        
        Ok(task)
    }
    
    /// Create a processing task with dependencies
    /// 
    /// # Arguments
    /// * `asset_id` - UUID of the media asset to process
    /// * `title` - Title for the processing task
    /// * `description` - Description of the processing to perform
    /// * `dependencies` - List of task IDs that this task depends on
    /// 
    /// # Returns
    /// Result with the created Task on success, ProcessingError on failure
    pub async fn create_processing_task_with_dependencies(
        asset_id: Uuid,
        title: String,
        description: String,
        dependencies: Vec<Uuid>,
    ) -> Result<Task, ProcessingError> {
        let mut task = Task {
            id: Uuid::new_v4(),
            title,
            description,
            due_date: None,
            priority: task_manager::task_core::value_objects::TaskPriority::Medium,
            status: TaskStatus::NotStarted,
            progress: ProgressPercentage(0),
            assignee: None,
            dependencies: DependencyGraph::new(),
            recurrence: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        // Add dependencies
        for dependency_id in dependencies {
            task.dependencies.add_dependency(dependency_id)?;
        }
        
        // In a real implementation, we would save the task to a repository
        // task_repo.save(task).await?;
        
        Ok(task)
    }
    
    /// Schedule a recurring media processing job
    /// 
    /// # Arguments
    /// * `title` - Title for the recurring task
    /// * `description` - Description of the processing to perform
    /// * `pattern` - Recurrence pattern (Daily, Weekly, Monthly)
    /// * `interval` - Interval for recurrence (e.g., every 1 day, every 2 weeks)
    /// 
    /// # Returns
    /// Result with the created Task on success, ProcessingError on failure
    pub fn schedule_recurring_job(
        title: String,
        description: String,
        pattern: RecurrencePattern,
        interval: u32,
    ) -> Result<Task, ProcessingError> {
        let recurrence_rule = RecurrenceRule {
            pattern,
            interval,
        };
        
        let task = Task {
            id: Uuid::new_v4(),
            title,
            description,
            due_date: None,
            priority: task_manager::task_core::value_objects::TaskPriority::Medium,
            status: TaskStatus::NotStarted,
            progress: ProgressPercentage(0),
            assignee: None,
            dependencies: DependencyGraph::new(),
            recurrence: Some(recurrence_rule),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        // In a real implementation, we would save the task to a repository
        // task_repo.save(task).await?;
        
        Ok(task)
    }
    
    /// Update the progress of a media processing task
    /// 
    /// # Arguments
    /// * `task` - Mutable reference to the task to update
    /// * `progress` - Progress percentage (0-100)
    /// 
    /// # Returns
    /// Result with unit on success, ProcessingError on failure
    pub fn update_task_progress(task: &mut Task, progress: u8) -> Result<(), ProcessingError> {
        ProgressCalculator::update_progress(task, progress)?;
        Ok(())
    }
    
    /// Generate the next task in a recurring series
    /// 
    /// # Arguments
    /// * `task` - The recurring task to generate the next instance for
    /// 
    /// # Returns
    /// Option with the next Task if the input task is recurring, None otherwise
    pub fn generate_next_task(task: &Task) -> Option<Task> {
        RecurrenceGenerator::generate_next_task(task)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use task_manager::task_core::value_objects::{RecurrencePattern, TaskPriority};
    
    #[tokio::test]
    async fn test_create_processing_task() {
        let asset_id = Uuid::new_v4();
        let title = "Process video".to_string();
        let description = "Transcode video to WebM format".to_string();
        
        let result = MediaProcessingService::create_processing_task(
            asset_id,
            title.clone(),
            description.clone(),
        ).await;
        
        assert!(result.is_ok());
        let task = result.unwrap();
        assert_eq!(task.title, title);
        assert_eq!(task.description, description);
        assert_eq!(task.status, TaskStatus::NotStarted);
    }
    
    #[tokio::test]
    async fn test_create_processing_task_with_dependencies() {
        let asset_id = Uuid::new_v4();
        let title = "Process video".to_string();
        let description = "Transcode video to WebM format".to_string();
        let dependency_id = Uuid::new_v4();
        let dependencies = vec![dependency_id];
        
        let result = MediaProcessingService::create_processing_task_with_dependencies(
            asset_id,
            title.clone(),
            description.clone(),
            dependencies.clone(),
        ).await;
        
        assert!(result.is_ok());
        let task = result.unwrap();
        assert_eq!(task.title, title);
        assert_eq!(task.description, description);
        assert_eq!(task.status, TaskStatus::NotStarted);
        assert!(task.dependencies.dependencies.contains(&dependency_id));
    }
    
    #[test]
    fn test_schedule_recurring_job() {
        let title = "Daily backup".to_string();
        let description = "Backup media assets daily".to_string();
        
        let result = MediaProcessingService::schedule_recurring_job(
            title.clone(),
            description.clone(),
            RecurrencePattern::Daily,
            1,
        );
        
        assert!(result.is_ok());
        let task = result.unwrap();
        assert_eq!(task.title, title);
        assert_eq!(task.description, description);
        assert_eq!(task.status, TaskStatus::NotStarted);
        assert!(task.recurrence.is_some());
        
        let recurrence = task.recurrence.unwrap();
        assert_eq!(recurrence.pattern, RecurrencePattern::Daily);
        assert_eq!(recurrence.interval, 1);
    }
    
    #[test]
    fn test_update_task_progress() {
        let mut task = Task {
            id: Uuid::new_v4(),
            title: "Test Task".to_string(),
            description: "Test Description".to_string(),
            due_date: None,
            priority: TaskPriority::Medium,
            status: TaskStatus::NotStarted,
            progress: ProgressPercentage(0),
            assignee: None,
            dependencies: DependencyGraph::new(),
            recurrence: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        let result = MediaProcessingService::update_task_progress(&mut task, 50);
        assert!(result.is_ok());
        assert_eq!(task.progress.0, 50);
        assert_eq!(task.status, TaskStatus::InProgress);
    }
    
    #[test]
    fn test_generate_next_task() {
        let task = Task {
            id: Uuid::new_v4(),
            title: "Recurring Task".to_string(),
            description: "Test Description".to_string(),
            due_date: None,
            priority: TaskPriority::Medium,
            status: TaskStatus::NotStarted,
            progress: ProgressPercentage(0),
            assignee: None,
            dependencies: DependencyGraph::new(),
            recurrence: Some(RecurrenceRule {
                pattern: RecurrencePattern::Daily,
                interval: 1,
            }),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        let next_task = MediaProcessingService::generate_next_task(&task);
        assert!(next_task.is_some());
        
        let next_task = next_task.unwrap();
        assert_eq!(next_task.title, task.title);
        assert_eq!(next_task.priority, task.priority);
    }
}