//! Media collaboration service that integrates with the task manager
//! 
//! This module provides functionality for media sharing, collaboration tracking,
//! and non-monetary rewards for media contributions.

use uuid::Uuid;
use chrono::{DateTime, Utc};
use task_manager::task_core::{
    entities::Task,
    value_objects::{TaskStatus, TaskPriority},
};

/// Error types for media collaboration
#[derive(Debug)]
pub enum CollaborationError {
    TaskError(String),
    SharingFailed(String),
}

/// Media collaboration service
pub struct MediaCollaborationService;

/// Represents a non-monetary reward for media contributions
#[derive(Debug, Clone)]
pub struct RecognitionReward {
    pub recipient_id: Uuid,
    pub reward_type: RecognitionType,
    pub description: String,
    pub awarded_at: DateTime<Utc>,
}

/// Types of non-monetary recognition rewards
#[derive(Debug, Clone)]
pub enum RecognitionType {
    FeaturedCreator,
    CommunitySpotlight,
    AchievementBadge(String),
    SocialRecognition,
}

impl MediaCollaborationService {
    /// Share a media asset with other users
    /// 
    /// # Arguments
    /// * `asset_id` - UUID of the media asset to share
    /// * `sharer_id` - UUID of the user sharing the asset
    /// * `recipients` - List of user IDs to share with
    /// 
    /// # Returns
    /// Result with unit on success, CollaborationError on failure
    pub async fn share_media_asset(
        asset_id: Uuid,
        sharer_id: Uuid,
        recipients: Vec<Uuid>,
    ) -> Result<(), CollaborationError> {
        // In a real implementation, this would:
        // 1. Update sharing permissions in the database
        // 2. Notify recipients of the shared asset
        // 3. Create a sharing task in the task manager
        
        // Create a task to track the sharing activity
        let task = Task {
            id: Uuid::new_v4(),
            title: format!("Share media asset {}", asset_id),
            description: format!("Shared by user {} with {} recipients", sharer_id, recipients.len()),
            due_date: None,
            priority: TaskPriority::Low,
            status: TaskStatus::Completed, // Sharing is immediate
            progress: task_manager::task_core::value_objects::ProgressPercentage(100),
            assignee: Some(sharer_id),
            dependencies: task_manager::task_core::value_objects::DependencyGraph::new(),
            recurrence: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        // In a real implementation, we would save the task to a repository
        // task_repo.save(task).await?;
        
        Ok(())
    }
    
    /// Award non-monetary recognition for media contributions
    /// 
    /// # Arguments
    /// * `asset_id` - UUID of the media asset that triggered the reward
    /// * `creator_id` - UUID of the user who created the asset
    /// * `reward_type` - Type of recognition to award
    /// 
    /// # Returns
    /// Result with RecognitionReward on success, CollaborationError on failure
    pub fn award_recognition(
        asset_id: Uuid,
        creator_id: Uuid,
        reward_type: RecognitionType,
    ) -> Result<RecognitionReward, CollaborationError> {
        let reward = RecognitionReward {
            recipient_id: creator_id,
            reward_type,
            description: format!("Recognition for media asset {}", asset_id),
            awarded_at: Utc::now(),
        };
        
        // In a real implementation, this would:
        // 1. Save the reward to a database
        // 2. Notify the recipient
        // 3. Update their profile with the recognition
        
        Ok(reward)
    }
    
    /// Create a collaboration task for media editing
    /// 
    /// # Arguments
    /// * `asset_id` - UUID of the media asset to collaborate on
    /// * `title` - Title for the collaboration task
    /// * `description` - Description of the collaboration work
    /// * `collaborators` - List of user IDs to collaborate with
    /// 
    /// # Returns
    /// Result with the created Task on success, CollaborationError on failure
    pub async fn create_collaboration_task(
        asset_id: Uuid,
        title: String,
        description: String,
        collaborators: Vec<Uuid>,
    ) -> Result<Task, CollaborationError> {
        let task = Task {
            id: Uuid::new_v4(),
            title,
            description,
            due_date: None,
            priority: TaskPriority::Medium,
            status: TaskStatus::NotStarted,
            progress: task_manager::task_core::value_objects::ProgressPercentage(0),
            assignee: collaborators.first().copied(), // Assign to first collaborator
            dependencies: task_manager::task_core::value_objects::DependencyGraph::new(),
            recurrence: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        // In a real implementation, we would save the task to a repository
        // task_repo.save(task).await?;
        
        Ok(task)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use task_manager::task_core::value_objects::ProgressPercentage;
    
    #[tokio::test]
    async fn test_share_media_asset() {
        let asset_id = Uuid::new_v4();
        let sharer_id = Uuid::new_v4();
        let recipients = vec![Uuid::new_v4(), Uuid::new_v4()];
        
        let result = MediaCollaborationService::share_media_asset(
            asset_id,
            sharer_id,
            recipients.clone(),
        ).await;
        
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_award_recognition() {
        let asset_id = Uuid::new_v4();
        let creator_id = Uuid::new_v4();
        let reward_type = RecognitionType::CommunitySpotlight;
        
        let result = MediaCollaborationService::award_recognition(
            asset_id,
            creator_id,
            reward_type.clone(),
        );
        
        assert!(result.is_ok());
        let reward = result.unwrap();
        assert_eq!(reward.recipient_id, creator_id);
        assert_eq!(reward.reward_type, reward_type);
    }
    
    #[tokio::test]
    async fn test_create_collaboration_task() {
        let asset_id = Uuid::new_v4();
        let title = "Collaborative video editing".to_string();
        let description = "Edit video with team members".to_string();
        let collaborators = vec![Uuid::new_v4(), Uuid::new_v4()];
        
        let result = MediaCollaborationService::create_collaboration_task(
            asset_id,
            title.clone(),
            description.clone(),
            collaborators.clone(),
        ).await;
        
        assert!(result.is_ok());
        let task = result.unwrap();
        assert_eq!(task.title, title);
        assert_eq!(task.description, description);
        assert_eq!(task.status, TaskStatus::NotStarted);
        assert_eq!(task.progress, ProgressPercentage(0));
    }
}