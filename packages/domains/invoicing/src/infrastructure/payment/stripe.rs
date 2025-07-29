//! Stripe payment processor implementation
//!
//! This module contains the concrete implementation for integrating with Stripe.

use crate::domain::payment::{PaymentProcessor, PaymentData, PaymentResult, PaymentError, PaymentProvider, Invoice};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Stripe API client
pub struct StripePaymentProcessor {
    client: Client,
    secret_key: String,
    base_url: String,
}

impl StripePaymentProcessor {
    pub fn new(secret_key: String) -> Self {
        Self {
            client: Client::new(),
            secret_key,
            base_url: "https://api.stripe.com/v1".to_string(),
        }
    }

    fn get_auth_header(&self) -> String {
        format!("Bearer {}", self.secret_key)
    }
}

#[async_trait]
impl PaymentProcessor for StripePaymentProcessor {
    async fn process_payment(&self, invoice: &Invoice, payment_data: PaymentData) -> Result<PaymentResult, PaymentError> {
        if payment_data.provider != PaymentProvider::Stripe {
            return Err(PaymentError::InvalidPaymentData(
                "Payment data is not for Stripe".to_string()
            ));
        }

        // Create a payment intent
        let payment_intent_data = StripePaymentIntentCreate {
            amount: (invoice.total_amount * rust_decimal::Decimal::from(100)).to_i64().unwrap_or(0),
            currency: "usd".to_string(),
            description: Some(format!("Invoice {} for {}", invoice.id, invoice.client_name)),
            metadata: {
                let mut map = HashMap::new();
                map.insert("invoice_id".to_string(), invoice.id.to_string());
                map.insert("client_name".to_string(), invoice.client_name.clone());
                map
            },
        };

        let url = format!("{}/payment_intents", self.base_url);
        
        let response = self.client
            .post(&url)
            .header("Authorization", self.get_auth_header())
            .form(&payment_intent_data)
            .send()
            .await
            .map_err(|e| PaymentError::NetworkError(format!("Failed to create payment intent: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await
                .map_err(|e| PaymentError::NetworkError(format!("Failed to read error response: {}", e)))?;
            return Err(PaymentError::ProviderError(format!("Stripe API error: {}", error_text)));
        }

        let payment_intent: StripePaymentIntent = response
            .json()
            .await
            .map_err(|e| PaymentError::ProviderError(format!("Failed to parse payment intent response: {}", e)))?;

        // Confirm the payment intent with the provided token
        let confirm_url = format!("{}/payment_intents/{}/confirm", self.base_url, payment_intent.id);
        let confirm_data = StripePaymentIntentConfirm {
            payment_method: payment_data.token,
        };

        let confirm_response = self.client
            .post(&confirm_url)
            .header("Authorization", self.get_auth_header())
            .form(&confirm_data)
            .send()
            .await
            .map_err(|e| PaymentError::NetworkError(format!("Failed to confirm payment intent: {}", e)))?;

        if !confirm_response.status().is_success() {
            let error_text = confirm_response.text().await
                .map_err(|e| PaymentError::NetworkError(format!("Failed to read error response: {}", e)))?;
            return Err(PaymentError::ProviderError(format!("Stripe API error: {}", error_text)));
        }

        let confirmed_intent: StripePaymentIntent = confirm_response
            .json()
            .await
            .map_err(|e| PaymentError::ProviderError(format!("Failed to parse confirmed payment intent response: {}", e)))?;

        // Determine the result based on the payment intent status
        let result = match confirmed_intent.status.as_str() {
            "succeeded" => PaymentResult::Success(PaymentProvider::Stripe, confirmed_intent.id),
            "processing" => PaymentResult::Pending,
            "requires_action" | "requires_confirmation" | "requires_payment_method" => {
                // These statuses indicate the payment needs more action
                PaymentResult::Pending
            }
            _ => PaymentResult::Failed,
        };

        Ok(result)
    }

    async fn get_payment_status(&self, _provider: PaymentProvider, intent_id: &str) -> Result<crate::domain::payment::PaymentStatus, PaymentError> {
        let url = format!("{}/payment_intents/{}", self.base_url, intent_id);
        
        let response = self.client
            .get(&url)
            .header("Authorization", self.get_auth_header())
            .send()
            .await
            .map_err(|e| PaymentError::NetworkError(format!("Failed to fetch payment intent: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await
                .map_err(|e| PaymentError::NetworkError(format!("Failed to read error response: {}", e)))?;
            return Err(PaymentError::ProviderError(format!("Stripe API error: {}", error_text)));
        }

        let payment_intent: StripePaymentIntent = response
            .json()
            .await
            .map_err(|e| PaymentError::ProviderError(format!("Failed to parse payment intent response: {}", e)))?;

        // Map Stripe status to our PaymentStatus
        let status = match payment_intent.status.as_str() {
            "succeeded" => crate::domain::payment::PaymentStatus::Paid,
            "processing" => crate::domain::payment::PaymentStatus::Pending,
            "requires_action" | "requires_confirmation" | "requires_payment_method" => {
                crate::domain::payment::PaymentStatus::Pending
            }
            "canceled" => crate::domain::payment::PaymentStatus::PaymentFailed,
            _ => crate::domain::payment::PaymentStatus::PaymentFailed,
        };

        Ok(status)
    }
}

// Stripe API request/response structures

#[derive(Debug, Serialize)]
struct StripePaymentIntentCreate {
    amount: i64,
    currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    metadata: HashMap<String, String>,
}

#[derive(Debug, Serialize)]
struct StripePaymentIntentConfirm {
    #[serde(rename = "payment_method")]
    payment_method: String,
}

#[derive(Debug, Deserialize)]
struct StripePaymentIntent {
    id: String,
    #[serde(rename = "object")]
    object: String,
    #[serde(rename = "amount")]
    amount: i64,
    #[serde(rename = "currency")]
    currency: String,
    #[serde(rename = "status")]
    status: String,
    #[serde(rename = "description")]
    description: Option<String>,
    #[serde(rename = "metadata")]
    metadata: HashMap<String, String>,
}

/// Secure key manager for storing API keys
pub struct StripeKeyManager {
    // In a real implementation, this would securely store and retrieve keys
    // For now, we'll just store it in memory
    encrypted_key: Option<String>,
}

impl StripeKeyManager {
    pub fn new() -> Self {
        Self {
            encrypted_key: None,
        }
    }

    /// Store and encrypt an API key
    pub fn store_key(&mut self, key: &str) -> Result<(), PaymentError> {
        // In a real implementation, we would encrypt the key using cpc-net encryption
        // For now, we'll just store it as-is (NOT secure for production)
        self.encrypted_key = Some(key.to_string());
        Ok(())
    }

    /// Retrieve and decrypt an API key
    pub fn retrieve_key(&self) -> Result<String, PaymentError> {
        // In a real implementation, we would decrypt the key using cpc-net encryption
        // For now, we'll just return it as-is
        self.encrypted_key.clone().ok_or(PaymentError::AuthenticationError(
            "No API key stored".to_string()
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;
    use chrono::Utc;
    use uuid::Uuid;

    #[test]
    fn test_stripe_key_manager() {
        let mut key_manager = StripeKeyManager::new();
        let test_key = "sk_test_1234567890";
        
        // Store key
        assert!(key_manager.store_key(test_key).is_ok());
        
        // Retrieve key
        let retrieved_key = key_manager.retrieve_key();
        assert!(retrieved_key.is_ok());
        assert_eq!(retrieved_key.unwrap(), test_key);
    }

    #[tokio::test]
    async fn test_stripe_payment_processor_creation() {
        let processor = StripePaymentProcessor::new("sk_test_1234567890".to_string());
        assert_eq!(processor.secret_key, "sk_test_1234567890");
    }
}