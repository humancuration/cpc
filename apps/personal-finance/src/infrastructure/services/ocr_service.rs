//! OCR service implementation for receipt scanning and data extraction

use std::sync::Arc;
use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::NaiveDate;
use reqwest::{Client, multipart};
use serde::{Deserialize, Serialize};

use crate::domain::{models::FinanceError, expense_service::OcrService};

/// OCR service configuration
#[derive(Debug, Clone)]
pub struct OcrServiceConfig {
    pub base_url: String,
    pub api_key: String,
}

/// Extracted receipt data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiptData {
    pub merchant_name: String,
    pub total_amount: Decimal,
    pub date: NaiveDate,
    pub category: Option<String>,
    pub items: Vec<ReceiptItem>,
}

/// Individual receipt item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiptItem {
    pub name: String,
    pub quantity: Decimal,
    pub unit_price: Decimal,
    pub total_price: Decimal,
}

/// HTTP implementation of OcrService
pub struct HttpOcrService {
    client: Client,
    config: OcrServiceConfig,
}

impl HttpOcrService {
    pub fn new(config: OcrServiceConfig) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }
}

#[async_trait::async_trait]
impl OcrService for HttpOcrService {
    async fn process_receipt_image(
        &self,
        user_id: Uuid,
        image_data: Vec<u8>,
        mime_type: String,
    ) -> Result<ReceiptData, FinanceError> {
        let form = multipart::Form::new()
            .part("file", multipart::Part::bytes(image_data)
                .file_name("receipt.jpg")
                .mime_str(&mime_type)
                .map_err(|e| FinanceError::ExternalServiceError(e.to_string()))?);

        let response = self.client
            .post(&format!("{}/api/v1/ocr/process", self.config.base_url))
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .multipart(form)
            .send()
            .await
            .map_err(|e| FinanceError::ExternalServiceError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(FinanceError::ExternalServiceError(
                format!("OCR service error: {}", response.status())
            ));
        }

        let data: ReceiptData = response
            .json()
            .await
            .map_err(|e| FinanceError::ExternalServiceError(e.to_string()))?;

        Ok(data)
    }

    async fn process_receipt_url(
        &self,
        user_id: Uuid,
        image_url: String,
    ) -> Result<ReceiptData, FinanceError> {
        let response = self.client
            .post(&format!("{}/api/v1/ocr/process-url", self.config.base_url))
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .json(&serde_json::json!({
                "user_id": user_id,
                "image_url": image_url
            }))
            .send()
            .await
            .map_err(|e| FinanceError::ExternalServiceError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(FinanceError::ExternalServiceError(
                format!("OCR service error: {}", response.status())
            ));
        }

        let data: ReceiptData = response
            .json()
            .await
            .map_err(|e| FinanceError::ExternalServiceError(e.to_string()))?;

        Ok(data)
    }

    async fn validate_receipt(
        &self,
        user_id: Uuid,
        receipt_data: ReceiptData,
    ) -> Result<bool, FinanceError> {
        let response = self.client
            .post(&format!("{}/api/v1/ocr/validate", self.config.base_url))
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .json(&serde_json::json!({
                "user_id": user_id,
                "receipt_data": receipt_data
            }))
            .send()
            .await
            .map_err(|e| FinanceError::ExternalServiceError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(FinanceError::ExternalServiceError(
                format!("OCR validation service error: {}", response.status())
            ));
        }

        let validation: serde_json::Value = response
            .json()
            .await
            .map_err(|e| FinanceError::ExternalServiceError(e.to_string()))?;

        validation.get("valid")
            .and_then(|v| v.as_bool())
            .ok_or_else(|| FinanceError::ExternalServiceError("Invalid validation response".to_string()))
    }
}

/// Mock OCR service for development/testing
pub struct MockOcrService;

#[async_trait::async_trait]
impl OcrService for MockOcrService {
    async fn process_receipt_image(
        &self,
        _user_id: Uuid,
        _image_data: Vec<u8>,
        _mime_type: String,
    ) -> Result<ReceiptData, FinanceError> {
        // Mock receipt data
        Ok(ReceiptData {
            merchant_name: "Whole Foods Market".to_string(),
            total_amount: Decimal::new(12550, 2), // $125.50
            date: chrono::Utc::now().date_naive(),
            category: Some("groceries".to_string()),
            items: vec![
                ReceiptItem {
                    name: "Organic Milk".to_string(),
                    quantity: Decimal::new(2, 0),
                    unit_price: Decimal::new(450, 2),
                    total_price: Decimal::new(900, 2),
                },
                ReceiptItem {
                    name: "Avocado".to_string(),
                    quantity: Decimal::new(4, 0),
                    unit_price: Decimal::new(175, 2),
                    total_price: Decimal::new(700, 2),
                },
                ReceiptItem {
                    name: "Organic Bread".to_string(),
                    quantity: Decimal::new(1, 0),
                    unit_price: Decimal::new(495, 2),
                    total_price: Decimal::new(495, 2),
                },
            ],
        })
    }

    async fn process_receipt_url(
        &self,
        _user_id: Uuid,
        _image_url: String,
    ) -> Result<ReceiptData, FinanceError> {
        // Same mock data as process_receipt_image for consistency
        self.process_receipt_image(_user_id, vec![], "image/jpeg".to_string()).await
    }

    async fn validate_receipt(
        &self,
        _user_id: Uuid,
        receipt_data: ReceiptData,
    ) -> Result<bool, FinanceError> {
        // Simple validation - reject receipts over $1000
        Ok(receipt_data.total_amount <= Decimal::new(100000, 2))
    }
}