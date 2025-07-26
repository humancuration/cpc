use cpc_core::supply_chain::{
    models::{CreateProductionStageData, ProductSummary, ProductionStage, SupplyChain, UpdateSupplyChainData},
    repository::{RepositoryError, SupplyChainRepository},
};
use std::sync::Arc;
use uuid::Uuid;
use tracing::instrument;

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Not found")]
    NotFound,
    #[error("An unexpected error occurred: {0}")]
    Other(#[from] anyhow::Error),
}

#[derive(Clone)]
pub struct SupplyChainService {
    repository: Arc<dyn SupplyChainRepository>,
}

impl SupplyChainService {
    pub fn new(repository: Arc<dyn SupplyChainRepository>) -> Self {
        Self { repository }
    }

    #[instrument(skip(self))]
    pub async fn get_full_supply_chain(
        &self,
        product_id: Uuid,
        timestamp: Option<i64>,
    ) -> Result<SupplyChain, ServiceError> {
        if product_id.is_nil() {
            return Err(ServiceError::InvalidInput("Product ID cannot be nil".to_string()));
        }

        match self.repository.get_full_supply_chain(product_id, timestamp).await {
            Ok(supply_chain) => Ok(supply_chain),
            Err(RepositoryError::NotFound) => Err(ServiceError::NotFound),
            Err(e) => Err(ServiceError::Repository(e)),
        }
    }

    #[instrument(skip(self, data))]
    pub async fn update_supply_chain(
        &self,
        data: &UpdateSupplyChainData,
    ) -> Result<SupplyChain, ServiceError> {
        if data.product_id.is_nil() {
            return Err(ServiceError::InvalidInput("Product ID cannot be nil".to_string()));
        }
        // Potentially add more validation here for stages and connections
        // e.g., ensure no duplicate stage IDs, all connection IDs map to a provided stage, etc.

        self.repository
            .update_supply_chain(data)
            .await
            .map_err(ServiceError::from)
    }

    #[instrument(skip(self))]
    pub async fn list_products_with_supply_chains(
        &self,
    ) -> Result<Vec<ProductSummary>, ServiceError> {
        self.repository
            .list_products_with_supply_chains()
            .await
            .map_err(ServiceError::from)
    }

    #[instrument(skip(self, stage_data))]
    pub async fn create_production_stage(
        &self,
        product_id: Uuid,
        stage_data: &CreateProductionStageData,
    ) -> Result<ProductionStage, ServiceError> {
        if product_id.is_nil() {
            return Err(ServiceError::InvalidInput("Product ID cannot be nil".to_string()));
        }
        if stage_data.name.is_empty() {
             return Err(ServiceError::InvalidInput("Stage name cannot be empty".to_string()));
        }
        // Add more validation as needed

        self.repository
            .create_production_stage(product_id, stage_data)
            .await
            .map_err(ServiceError::from)
    }

    #[instrument(skip(self))]
    pub async fn list_stages_for_product(
        &self,
        product_id: Uuid,
    ) -> Result<Vec<ProductionStage>, ServiceError> {
        if product_id.is_nil() {
            return Err(ServiceError::InvalidInput(
                "Product ID cannot be nil".to_string(),
            ));
        }
        self.repository
            .list_stages_for_product(product_id)
            .await
            .map_err(ServiceError::from)
    }
}