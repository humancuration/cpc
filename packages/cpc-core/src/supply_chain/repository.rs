use async_trait::async_trait;
use uuid::Uuid;
use crate::supply_chain::models::{CreateProductionStageData, UpdateProductionStageData, ProductSummary, ProductionStage, SupplyChain, UpdateSupplyChainData};

// Using a more specific error type is better than anyhow::Error for libraries
#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("Database query failed: {0}")]
    QueryError(#[from] sqlx::Error),
    #[error("Data not found")]
    NotFound,
    #[error("An unexpected error occurred: {0}")]
    Other(#[from] anyhow::Error),
}

#[async_trait]
pub trait SupplyChainRepository: Send + Sync {
    /// Fetches the entire supply chain for a given product.
    /// If a timestamp is provided, it should fetch the state of the supply chain at that time.
    /// (For now, we can ignore the timestamp and return the full history).
    async fn get_full_supply_chain(
        &self,
        product_id: Uuid,
        timestamp: Option<i64>
    ) -> Result<SupplyChain, RepositoryError>;

    /// Fetches a summary of all products that have a supply chain.
    async fn list_products_with_supply_chains(&self) -> Result<Vec<ProductSummary>, RepositoryError>;

    /// Updates the stages and connections for a given supply chain.
    /// This should be an atomic operation.
    async fn update_supply_chain(
        &self,
        data: &UpdateSupplyChainData,
    ) -> Result<SupplyChain, RepositoryError>;

    /// Creates a new production stage for a given product.
    async fn create_production_stage(
        &self,
        product_id: Uuid,
        stage_data: &CreateProductionStageData,
    ) -> Result<ProductionStage, RepositoryError>;

    /// Updates an existing production stage
    async fn update_production_stage(
        &self,
        stage_id: Uuid,
        stage_data: &UpdateProductionStageData,
    ) -> Result<ProductionStage, RepositoryError>;

    /// Fetches all production stages for a specific product.
    async fn list_stages_for_product(&self, product_id: Uuid) -> Result<Vec<ProductionStage>, RepositoryError>;
}