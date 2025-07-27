//! Treasury service implementation for financial data and insights

use std::sync::Arc;
use uuid::Uuid;
use rust_decimal::Decimal;
use reqwest::Client;
use chrono::{NaiveDate, Utc};

use crate::domain::{models::FinanceError, expense_service::TreasuryService};

/// Configuration for Treasury service
#[derive(Debug, Clone)]
pub struct TreasuryServiceConfig {
    pub base_url: String,
    pub api_key: String,
}

/// HTTP implementation of TreasuryService
pub struct HttpTreasuryService {
    client: Client,
    config: TreasuryServiceConfig,
}

impl HttpTreasuryService {
    pub fn new(config: TreasuryServiceConfig) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }
}

#[async_trait::async_trait]
impl TreasuryService for HttpTreasuryService {
    async fn get_average_spending_by_category(
        &self,
        user_id: Uuid,
        category: String,
        months: u32,
    ) -> Result<Decimal, FinanceError> {
        let response = self.client
            .get(&format!("{}/api/v1/treasury/average-spending", self.config.base_url))
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .query(&[
                ("user_id", user_id.to_string()),
                ("category", category),
                ("months", months.to_string())
            ])
            .send()
            .await
            .map_err(|e| FinanceError::ExternalServiceError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(FinanceError::ExternalServiceError(
                format!("Treasury service error: {}", response.status())
            ));
        }

        let amount: f64 = response
            .json::<serde_json::Value>()
            .await
            .map_err(|e| FinanceError::ExternalServiceError(e.to_string()))?
            .get("average_amount")
            .and_then(|v| v.as_f64())
            .ok_or_else(|| FinanceError::ExternalServiceError("Invalid response format".to_string()))?;

        Ok(Decimal::from_f64_retain(amount).unwrap_or_else(|| Decimal::new(0, 0)))
    }

    async fn get_monthly_trend(
        &self,
        user_id: Uuid,
        metric: String,
        months: u32,
    ) -> Result<Vec<(NaiveDate, Decimal)>, FinanceError> {
        let response = self.client
            .get(&format!("{}/api/v1/treasury/monthly-trend", self.config.base_url))
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .query(&[
                ("user_id", user_id.to_string()),
                ("metric", metric),
                ("months", months.to_string())
            ])
            .send()
            .await
            .map_err(|e| FinanceError::ExternalServiceError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(FinanceError::ExternalServiceError(
                format!("Treasury service error: {}", response.status())
            ));
        }

        let trend_data: Vec<(String, f64)> = response
            .json::<serde_json::Value>()
            .await
            .map_err(|e| FinanceError::ExternalServiceError(e.to_string()))?
            .get("trend")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .ok_or_else(|| FinanceError::ExternalServiceError("Invalid response format".to_string()))?;

        trend_data
            .into_iter()
            .map(|(date_str, amount)| {
                let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
                    .map_err(|e| FinanceError::ExternalServiceError(e.to_string()))?;
                let decimal_amount = Decimal::from_f64_retain(amount)
                    .ok_or_else(|| FinanceError::ExternalServiceError("Invalid amount".to_string()))?;
                Ok((date, decimal_amount))
            })
            .collect()
    }

    async fn get_financial_insights(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<String>, FinanceError> {
        let response = self.client
            .get(&format!("{}/api/v1/treasury/insights", self.config.base_url))
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .query(&[("user_id", user_id.to_string())])
            .send()
            .await
            .map_err(|e| FinanceError::ExternalServiceError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(FinanceError::ExternalServiceError(
                format!("Treasury service error: {}", response.status())
            ));
        }

        let insights: Vec<String> = response
            .json::<serde_json::Value>()
            .await
            .map_err(|e| FinanceError::ExternalServiceError(e.to_string()))?
            .get("insights")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .ok_or_else(|| FinanceError::ExternalServiceError("Invalid response format".to_string()))?;

        Ok(insights)
    }
}

/// Mock Treasury service for development/testing
pub struct MockTreasuryService;

#[async_trait::async_trait]
impl TreasuryService for MockTreasuryService {
    async fn get_average_spending_by_category(
        &self,
        _user_id: Uuid,
        category: String,
        _months: u32,
    ) -> Result<Decimal, FinanceError> {
        // Mock average spending based on category
        let amount = match category.as_str() {
            "groceries" => Decimal::new(45000, 2), // $450.00
            "utilities" => Decimal::new(15000, 2), // $150.00
            "entertainment" => Decimal::new(20000, 2), // $200.00
            _ => Decimal::new(30000, 2), // $300.00 default
        };
        Ok(amount)
    }

    async fn get_monthly_trend(
        &self,
        _user_id: Uuid,
        _metric: String,
        months: u32,
    ) -> Result<Vec<(NaiveDate, Decimal)>, FinanceError> {
        // Mock trend data - decreasing trend
        let mut trends = Vec::new();
        let today = Utc::now().date_naive();
        
        for i in 0..months {
            let date = today - chrono::Duration::days((i * 30) as i64);
            let amount = Decimal::new((3000 - (i as i64 * 100)) * 100, 2);
            trends.push((date, amount));
        }
        
        Ok(trends)
    }

    async fn get_financial_insights(
        &self,
        _user_id: Uuid,
    ) -> Result<Vec<String>, FinanceError> {
        // Mock insights
        Ok(vec![
            "Your spending in groceries has decreased by 15% compared to last month".to_string(),
            "Consider increasing your monthly savings by $50 to meet your goals faster".to_string(),
            "You've been consistent with utility payments this quarter".to_string(),
        ])
    }
}