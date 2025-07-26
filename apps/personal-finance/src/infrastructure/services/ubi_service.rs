//! UBI service implementation for Universal Basic Income calculations

use std::sync::Arc;
use uuid::Uuid;
use rust_decimal::Decimal;
use reqwest::Client;

use crate::domain::{models::FinanceError, budget_service::UbiService};

/// Configuration for UBI service
#[derive(Debug, Clone)]
pub struct UbiServiceConfig {
    pub base_url: String,
    pub api_key: String,
}

/// HTTP implementation of UbiService
pub struct HttpUbiService {
    client: Client,
    config: UbiServiceConfig,
}

impl HttpUbiService {
    pub fn new(config: UbiServiceConfig) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }
}

#[async_trait::async_trait]
impl UbiService for HttpUbiService {
    async fn get_monthly_ubi_income(&self, user_id: Uuid) -> Result<Decimal, FinanceError> {
        let response = self.client
            .get(&format!("{}/api/v1/ubi/monthly", self.config.base_url))
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .query(&[("user_id", user_id.to_string())])
            .send()
            .await
            .map_err(|e| FinanceError::ExternalServiceError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(FinanceError::ExternalServiceError(
                format!("UBI service error: {}", response.status())
            ));
        }

        let amount: f64 = response
            .json::<serde_json::Value>()
            .await
            .map_err(|e| FinanceError::ExternalServiceError(e.to_string()))?
            .get("monthly_amount")
            .and_then(|v| v.as_f64())
            .ok_or_else(|| FinanceError::ExternalServiceError("Invalid response format".to_string()))?;

        Ok(Decimal::from_f64_retain(amount).unwrap_or_else(|| Decimal::new(1000, 0)))
    }

    async fn get_ubi_balance(&self, user_id: Uuid) -> Result<Decimal, FinanceError> {
        let response = self.client
            .get(&format!("{}/api/v1/ubi/balance", self.config.base_url))
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .query(&[("user_id", user_id.to_string())])
            .send()
            .await
            .map_err(|e| FinanceError::ExternalServiceError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(FinanceError::ExternalServiceError(
                format!("UBI service error: {}", response.status())
            ));
        }

        let balance: f64 = response
            .json::<serde_json::Value>()
            .await
            .map_err(|e| FinanceError::ExternalServiceError(e.to_string()))?
            .get("balance")
            .and_then(|v| v.as_f64())
            .ok_or_else(|| FinanceError::ExternalServiceError("Invalid response format".to_string()))?;

        Ok(Decimal::from_f64_retain(balance).unwrap_or_else(|| Decimal::new(0, 0)))
    }
}

/// Mock UBI service for development/testing
pub struct MockUbiService;

#[async_trait::async_trait]
impl UbiService for MockUbiService {
    async fn get_monthly_ubi_income(&self, _user_id: Uuid) -> Result<Decimal, FinanceError> {
        // Mock UBI amount - $1000 per month
        Ok(Decimal::new(1000, 0))
    }

    async fn get_ubi_balance(&self, _user_id: Uuid) -> Result<Decimal, FinanceError> {
        // Mock balance - $5000
        Ok(Decimal::new(5000, 0))
    }
}