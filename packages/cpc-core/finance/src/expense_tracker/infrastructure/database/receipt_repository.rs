//! PostgreSQL implementation of ReceiptRepository

use async_trait::async_trait;
use uuid::Uuid;
use sqlx::PgPool;
use chrono::{DateTime, Utc};
use crate::{
    domain::{
        expense_tracker::{Receipt, ReceiptImageData, ReceiptProcessingStatus},
        FinanceError,
    },
    expense_tracker::application::expense_service::ReceiptRepository,
};

/// Database model for receipts table
#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct ReceiptDbModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub image_data: Vec<u8>,
    pub image_format: String,
    pub extracted_text: String,
    pub merchant_name: Option<String>,
    pub transaction_date: Option<DateTime<Utc>>,
    pub total_amount: Option<sqlx::types::Decimal>,
    pub currency: Option<String>,
    pub dabloons_amount: sqlx::types::Decimal,
    pub processing_status: String,
    pub processing_error: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ReceiptDbModel {
    /// Convert domain Receipt to database model
    pub fn from_domain(receipt: &Receipt) -> Self {
        let (image_data, image_format) = match &receipt.image_data {
            ReceiptImageData::LocalPath(path) => (path.as_bytes().to_vec(), "path".to_string()),
            ReceiptImageData::Base64Data(data) => (data.as_bytes().to_vec(), "base64".to_string()),
            ReceiptImageData::ReferenceId(id) => (id.to_string().as_bytes().to_vec(), "reference".to_string()),
        };
        
        Self {
            id: receipt.id,
            user_id: receipt.user_id,
            image_data,
            image_format,
            extracted_text: receipt.extracted_text.clone(),
            merchant_name: receipt.merchant_name.clone(),
            transaction_date: receipt.transaction_date,
            total_amount: receipt.total_amount.as_ref().map(|m| m.amount),
            currency: receipt.total_amount.as_ref().map(|m| m.currency.code().to_string()),
            dabloons_amount: receipt.total_amount.as_ref()
                .map(|m| if m.currency == crate::domain::primitives::Currency::Dabloons { m.amount } else { sqlx::types::Decimal::ZERO })
                .unwrap_or(sqlx::types::Decimal::ZERO),
            processing_status: format!("{:?}", receipt.processing_status),
            processing_error: match &receipt.processing_status {
                ReceiptProcessingStatus::Failed(error) => Some(error.clone()),
                _ => None,
            },
            created_at: receipt.created_at,
            updated_at: receipt.updated_at,
        }
    }
    
    /// Convert database model to domain Receipt
    pub fn to_domain(&self) -> Receipt {
        let image_data = match self.image_format.as_str() {
            "path" => ReceiptImageData::LocalPath(String::from_utf8_lossy(&self.image_data).to_string()),
            "base64" => ReceiptImageData::Base64Data(String::from_utf8_lossy(&self.image_data).to_string()),
            "reference" => {
                if let Ok(uuid) = std::str::from_utf8(&self.image_data).and_then(|s| s.parse::<Uuid>()) {
                    ReceiptImageData::ReferenceId(uuid)
                } else {
                    ReceiptImageData::Base64Data(String::from_utf8_lossy(&self.image_data).to_string())
                }
            }
            _ => ReceiptImageData::Base64Data(String::from_utf8_lossy(&self.image_data).to_string()),
        };
        
        let total_amount = if let (Some(amount), Some(currency_str)) = (self.total_amount, &self.currency) {
            let currency = match currency_str.as_str() {
                "DABLOONS" => crate::domain::primitives::Currency::Dabloons,
                "USD" => crate::domain::primitives::Currency::USD,
                "EUR" => crate::domain::primitives::Currency::EUR,
                "GBP" => crate::domain::primitives::Currency::GBP,
                "JPY" => crate::domain::primitives::Currency::JPY,
                "CAD" => crate::domain::primitives::Currency::CAD,
                "AUD" => crate::domain::primitives::Currency::AUD,
                "CHF" => crate::domain::primitives::Currency::CHF,
                "CNY" => crate::domain::primitives::Currency::CNY,
                "SEK" => crate::domain::primitives::Currency::SEK,
                "NZD" => crate::domain::primitives::Currency::NZD,
                "MXN" => crate::domain::primitives::Currency::MXN,
                "SGD" => crate::domain::primitives::Currency::SGD,
                "HKD" => crate::domain::primitives::Currency::HKD,
                "NOK" => crate::domain::primitives::Currency::NOK,
                "KRW" => crate::domain::primitives::Currency::KRW,
                "TRY" => crate::domain::primitives::Currency::TRY,
                "RUB" => crate::domain::primitives::Currency::RUB,
                "INR" => crate::domain::primitives::Currency::INR,
                "BRL" => crate::domain::primitives::Currency::BRL,
                "ZAR" => crate::domain::primitives::Currency::ZAR,
                _ => crate::domain::primitives::Currency::USD,
            };
            Some(crate::domain::primitives::Money::new(amount, currency))
        } else if !self.dabloons_amount.is_zero() {
            Some(crate::domain::primitives::Money::new(self.dabloons_amount, crate::domain::primitives::Currency::Dabloons))
        } else {
            None
        };
        
        let processing_status = match self.processing_status.as_str() {
            "Uploaded" => ReceiptProcessingStatus::Uploaded,
            "Processing" => ReceiptProcessingStatus::Processing,
            "Processed" => ReceiptProcessingStatus::Processed,
            "Verified" => ReceiptProcessingStatus::Verified,
            "Failed" => ReceiptProcessingStatus::Failed(self.processing_error.clone().unwrap_or_default()),
            _ => ReceiptProcessingStatus::Uploaded,
        };
        
        Receipt {
            id: self.id,
            user_id: self.user_id,
            image_data,
            extracted_text: self.extracted_text.clone(),
            merchant_name: self.merchant_name.clone(),
            transaction_date: self.transaction_date,
            total_amount,
            processing_status,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

/// PostgreSQL implementation of ReceiptRepository
pub struct PostgresReceiptRepository {
    pool: PgPool,
}

impl PostgresReceiptRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ReceiptRepository for PostgresReceiptRepository {
    async fn save(&self, receipt: &Receipt) -> Result<(), FinanceError> {
        let receipt_db_model = ReceiptDbModel::from_domain(receipt);
        
        sqlx::query!(
            r#"
            INSERT INTO receipts (id, user_id, image_data, image_format, extracted_text, merchant_name, transaction_date, total_amount, currency, dabloons_amount, processing_status, processing_error, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            ON CONFLICT (id) DO UPDATE SET
                image_data = EXCLUDED.image_data,
                image_format = EXCLUDED.image_format,
                extracted_text = EXCLUDED.extracted_text,
                merchant_name = EXCLUDED.merchant_name,
                transaction_date = EXCLUDED.transaction_date,
                total_amount = EXCLUDED.total_amount,
                currency = EXCLUDED.currency,
                dabloons_amount = EXCLUDED.dabloons_amount,
                processing_status = EXCLUDED.processing_status,
                processing_error = EXCLUDED.processing_error,
                updated_at = EXCLUDED.updated_at
            "#,
            receipt_db_model.id,
            receipt_db_model.user_id,
            receipt_db_model.image_data,
            receipt_db_model.image_format,
            receipt_db_model.extracted_text,
            receipt_db_model.merchant_name,
            receipt_db_model.transaction_date,
            receipt_db_model.total_amount,
            receipt_db_model.currency,
            receipt_db_model.dabloons_amount,
            receipt_db_model.processing_status,
            receipt_db_model.processing_error,
            receipt_db_model.created_at,
            receipt_db_model.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| FinanceError::DatabaseError(e.to_string()))?;
        
        Ok(())
    }
    
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Receipt>, FinanceError> {
        let receipt_record = sqlx::query_as!(
            ReceiptDbModel,
            r#"
            SELECT id, user_id, image_data, image_format, extracted_text, merchant_name, transaction_date, total_amount, currency, dabloons_amount, processing_status, processing_error, created_at, updated_at
            FROM receipts
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| FinanceError::DatabaseError(e.to_string()))?;
        
        let receipt = receipt_record.map(|record| record.to_domain());
        
        Ok(receipt)
    }
    
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<Receipt>, FinanceError> {
        let receipt_records = sqlx::query_as!(
            ReceiptDbModel,
            r#"
            SELECT id, user_id, image_data, image_format, extracted_text, merchant_name, transaction_date, total_amount, currency, dabloons_amount, processing_status, processing_error, created_at, updated_at
            FROM receipts
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| FinanceError::DatabaseError(e.to_string()))?;
        
        let receipts: Vec<Receipt> = receipt_records
            .into_iter()
            .map(|record| record.to_domain())
            .collect();
        
        Ok(receipts)
    }
    
    async fn delete(&self, id: Uuid) -> Result<(), FinanceError> {
        sqlx::query!(
            r#"
            DELETE FROM receipts
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| FinanceError::DatabaseError(e.to_string()))?;
        
        Ok(())
    }
    
    async fn update(&self, receipt: &Receipt) -> Result<(), FinanceError> {
        self.save(receipt).await
    }
}