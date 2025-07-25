use anyhow::Context;
use async_trait::async_trait;
use cpc_core::supply_chain::{
    models::{
        CooperativeImpactSummary, ProductSummary, ProductionStage, StageConnection, SupplyChain,
        UpdateSupplyChainData,
    },
    repository::{RepositoryError, SupplyChainRepository},
};
use sqlx::{types::Decimal, PgPool, FromRow};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct SupplyChainRepositoryImpl {
    pool: PgPool,
}

impl SupplyChainRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

// ==============
//  DB Structs
// ==============

#[derive(FromRow, Debug)]
struct ProductionStageDb {
    id: Uuid,
    product_id: Uuid,
    name: String,
    description: Option<String>,
    location: String,
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
}

#[derive(FromRow, Debug)]
struct StageConnectionDb {
    from_stage_id: Uuid,
    to_stage_id: Uuid,
    relationship_type: String,
}

#[derive(FromRow, Debug)]
struct CooperativeImpactSummaryDb {
    product_id: Uuid,
    workers_benefited: i32, // Assuming i32 for u32
    coops_involved: i32, // Assuming i32 for u32
    ethical_sourcing_score: Decimal,
}


// ==============
//  From DB to Domain
// ==============

impl From<ProductionStageDb> for ProductionStage {
    fn from(db: ProductionStageDb) -> Self {
        Self {
            id: db.id,
            name: db.name,
            description: db.description,
            location: db.location,
            start_date: db.start_date,
            end_date: db.end_date,
        }
    }
}

impl From<StageConnectionDb> for StageConnection {
    fn from(db: StageConnectionDb) -> Self {
        Self {
            from_stage_id: db.from_stage_id,
            to_stage_id: db.to_stage_id,
            relationship_type: db.relationship_type,
        }
    }
}

impl From<CooperativeImpactSummaryDb> for CooperativeImpactSummary {
    fn from(db: CooperativeImpactSummaryDb) -> Self {
        Self {
            workers_benefited: db.workers_benefited as u32,
            coops_involved: db.coops_involved as u32,
            ethical_sourcing_score: db.ethical_sourcing_score.into(),
        }
    }
}

// ==============
//  Repository Trait Implementation
// ==============

#[async_trait]
impl SupplyChainRepository for SupplyChainRepositoryImpl {
    async fn get_full_supply_chain(
        &self,
        product_id: Uuid,
        _timestamp: Option<i64>, // For now, we ignore the timestamp
    ) -> Result<SupplyChain, RepositoryError> {
        let mut tx = self.pool.begin().await.context("Failed to begin transaction")?;
        
        let stages_db: Vec<ProductionStageDb> = sqlx::query_as!(
            ProductionStageDb,
            r#"
            SELECT id, product_id, name, description, location, start_date, end_date
            FROM production_stages WHERE product_id = $1
            "#,
            product_id
        )
        .fetch_all(&mut *tx)
        .await
        .context("Failed to fetch production stages")?;

        let connections_db: Vec<StageConnectionDb> = sqlx::query_as!(
            StageConnectionDb,
            r#"
            SELECT sc.from_stage_id, sc.to_stage_id, sc.relationship_type
            FROM stage_connections sc
            JOIN production_stages ps_from ON sc.from_stage_id = ps_from.id
            WHERE ps_from.product_id = $1
            "#,
            product_id
        )
        .fetch_all(&mut *tx)
        .await
        .context("Failed to fetch stage connections")?;

         let impact_db: CooperativeImpactSummaryDb = sqlx::query_as!(
            CooperativeImpactSummaryDb,
            r#"
            SELECT product_id, workers_benefited, coops_involved, ethical_sourcing_score
            FROM cooperative_impact_summaries WHERE product_id = $1
            "#,
            product_id
        )
        .fetch_one(&mut *tx)
        .await
        .context("Failed to fetch cooperative impact summary")?;

        tx.commit().await.context("Failed to commit transaction")?;

        let stages: Vec<ProductionStage> = stages_db.into_iter().map(Into::into).collect();
        let connections: Vec<StageConnection> = connections_db.into_iter().map(Into::into).collect();
        let cooperative_impact: CooperativeImpactSummary = impact_db.into();

        let timeline_range = stages
            .iter()
            .fold(None, |acc, stage| {
                let (min, max) = acc.unwrap_or((stage.start_date, stage.end_date));
                Some((min.min(stage.start_date), max.max(stage.end_date)))
            })
            .unwrap_or((Utc::now(), Utc::now()));


        Ok(SupplyChain {
            product_id,
            stages,
            connections,
            cooperative_impact,
            timeline_range,
        })
    }

    async fn list_products_with_supply_chains(
        &self,
    ) -> Result<Vec<ProductSummary>, RepositoryError> {
        let products = sqlx::query_as!(
            ProductSummary,
            r#"
            SELECT p.id, p.name
            FROM products p
            WHERE EXISTS (SELECT 1 FROM production_stages ps WHERE ps.product_id = p.id)
            ORDER BY p.name
            "#
        )
        .fetch_all(&self.pool)
        .await
        .context("Failed to fetch product summaries")?;

        Ok(products)
    }

    async fn update_supply_chain(
        &self,
        data: &UpdateSupplyChainData,
    ) -> Result<SupplyChain, RepositoryError> {
        let mut tx = self.pool.begin().await.context("Failed to begin transaction")?;

        // 1. Delete existing connections and stages for this product
        // Note: The order matters here. Delete connections first to avoid foreign key violations.
        sqlx::query!(
            r#"
            DELETE FROM stage_connections
            WHERE from_stage_id IN (SELECT id FROM production_stages WHERE product_id = $1)
            "#,
            data.product_id
        )
        .execute(&mut *tx)
        .await
        .context("Failed to delete old stage connections")?;

        sqlx::query!(
            "DELETE FROM production_stages WHERE product_id = $1",
            data.product_id
        )
        .execute(&mut *tx)
        .await
        .context("Failed to delete old production stages")?;

        // 2. Insert new stages
        for stage in &data.stages {
            sqlx::query!(
                r#"
                INSERT INTO production_stages (id, product_id, name, description, location, start_date, end_date)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                "#,
                stage.id,
                data.product_id,
                stage.name,
                stage.description,
                stage.location,
                stage.start_date,
                stage.end_date,
            )
            .execute(&mut *tx)
            .await
            .context("Failed to insert new production stage")?;
        }

        // 3. Insert new connections
        for conn in &data.connections {
            sqlx::query!(
                r#"
                INSERT INTO stage_connections (from_stage_id, to_stage_id, relationship_type)
                VALUES ($1, $2, $3)
                "#,
                conn.from_stage_id,
                conn.to_stage_id,
                conn.relationship_type,
            )
            .execute(&mut *tx)
            .await
            .context("Failed to insert new stage connection")?;
        }

        tx.commit().await.context("Failed to commit transaction")?;

        // 4. Fetch the updated supply chain to return it
        // The timestamp is ignored, as per existing logic.
        self.get_full_supply_chain(data.product_id, None).await
    }
}