use async_trait::async_trait;
use crate::domain::community::Community;
use crate::infrastructure::repositories::community_repo::CommunityRepository;
use uuid::Uuid;
use std::sync::Arc;
use crate::application::error::ApplicationError;

#[derive(Debug, Clone)]
pub struct CreateCommunityInput {
    pub name: String,
    pub description: String,
    pub rules: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct UpdateCommunityInput {
    pub name: Option<String>,
    pub description: Option<String>,
    pub rules: Option<Vec<String>>,
}

#[async_trait]
pub trait CommunityService: Send + Sync {
    async fn create_community(&self, input: CreateCommunityInput) -> Result<Community, ApplicationError>;
    async fn update_community(&self, id: Uuid, input: UpdateCommunityInput) -> Result<Community, ApplicationError>;
    async fn delete_community(&self, id: Uuid) -> Result<bool, ApplicationError>;
    async fn get_community(&self, id: Uuid) -> Result<Option<Community>, ApplicationError>;
    async fn search_communities(&self, query: String) -> Result<Vec<Community>, ApplicationError>;
}

pub struct CommunityServiceImpl {
    community_repo: Arc<dyn CommunityRepository>,
}

impl CommunityServiceImpl {
    pub fn new(community_repo: Arc<dyn CommunityRepository>) -> Self {
        Self { community_repo }
    }
}

#[async_trait]
impl CommunityService for CommunityServiceImpl {
    async fn create_community(&self, input: CreateCommunityInput) -> Result<Community, ApplicationError> {
        // Validate input
        if input.name.is_empty() {
            return Err(ApplicationError::InvalidInput("Community name cannot be empty".to_string()));
        }
        
        // Check if community with this name already exists
        if let Some(_) = self.community_repo.find_by_name(&input.name).await? {
            return Err(ApplicationError::InvalidInput("Community with this name already exists".to_string()));
        }
        
        // Create community
        let community = Community::new(input.name, input.description, input.rules);
        self.community_repo.create(&community).await?;
        
        Ok(community)
    }
    
    async fn update_community(&self, id: Uuid, input: UpdateCommunityInput) -> Result<Community, ApplicationError> {
        // Find existing community
        let mut community = self.community_repo.find_by_id(id).await?
            .ok_or(ApplicationError::NotFound)?;
        
        // Update fields if provided
        if let Some(name) = input.name {
            if !name.is_empty() {
                community.name = name;
            }
        }
        
        if let Some(description) = input.description {
            community.description = description;
        }
        
        if let Some(rules) = input.rules {
            // Limit rules to 10 as per requirements
            if rules.len() > 10 {
                return Err(ApplicationError::InvalidInput("Cannot have more than 10 rules".to_string()));
            }
            community.rules = rules;
        }
        
        // Save updated community
        self.community_repo.update(&community).await?;
        
        Ok(community)
    }
    
    async fn delete_community(&self, id: Uuid) -> Result<bool, ApplicationError> {
        // Check if community exists
        if self.community_repo.find_by_id(id).await?.is_none() {
            return Err(ApplicationError::NotFound);
        }
        
        // Delete community
        self.community_repo.delete(id).await?;
        
        Ok(true)
    }
    
    async fn get_community(&self, id: Uuid) -> Result<Option<Community>, ApplicationError> {
        self.community_repo.find_by_id(id).await.map_err(ApplicationError::from)
    }
    
    async fn search_communities(&self, query: String) -> Result<Vec<Community>, ApplicationError> {
        // For now, we'll implement a simple search that returns all communities
        // In a real implementation, this would search by name or description
        // This would require a new repository method
        todo!("Implement search_communities")
    }
}