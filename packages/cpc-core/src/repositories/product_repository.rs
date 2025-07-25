use anyhow::Result;
use async_trait::async_trait;
use crate::models::product::Product;

#[async_trait]
pub trait ProductRepository: Send + Sync {
    async fn get_by_barcode(&self, barcode: &str) -> Result<Option<Product>>;
}

pub struct PgProductRepository {
    pool: sqlx::PgPool,
}

impl PgProductRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProductRepository for PgProductRepository {
    async fn get_by_barcode(&self, barcode: &str) -> Result<Option<Product>> {
        sqlx::query_as!(
            Product,
            r#"SELECT * FROM products WHERE barcode = $1"#,
            barcode
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(Into::into)
    }
}