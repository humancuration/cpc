//! PayPal payment processor implementation
//!
//! This module contains the concrete implementation for integrating with PayPal.

use crate::domain::payment::{PaymentProcessor, PaymentData, PaymentResult, PaymentError, PaymentProvider, Invoice};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// PayPal API client
pub struct PayPalPaymentProcessor {
    client: Client,
    client_id: String,
    secret: String,
    base_url: String,
    access_token: Option<String>,
}

impl PayPalPaymentProcessor {
    pub fn new(client_id: String, secret: String) -> Self {
        Self {
            client: Client::new(),
            client_id,
            secret,
            base_url: "https://api.paypal.com".to_string(), // Use "https://api.sandbox.paypal.com" for sandbox
            access_token: None,
        }
    }

    /// Get OAuth access token
    async fn get_access_token(&mut self) -> Result<String, PaymentError> {
        if let Some(token) = &self.access_token {
            // In a real implementation, we would check if the token is still valid
            return Ok(token.clone());
        }

        let auth_string = format!("{}:{}", self.client_id, self.secret);
        let encoded_auth = base64::encode(auth_string);

        let url = format!("{}/v1/oauth2/token", self.base_url);
        let params = [("grant_type", "client_credentials")];

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Basic {}", encoded_auth))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&params)
            .send()
            .await
            .map_err(|e| PaymentError::NetworkError(format!("Failed to get access token: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await
                .map_err(|e| PaymentError::NetworkError(format!("Failed to read error response: {}", e)))?;
            return Err(PaymentError::AuthenticationError(format!("PayPal OAuth error: {}", error_text)));
        }

        let token_response: PayPalAccessTokenResponse = response
            .json()
            .await
            .map_err(|e| PaymentError::ProviderError(format!("Failed to parse access token response: {}", e)))?;

        self.access_token = Some(token_response.access_token.clone());
        Ok(token_response.access_token)
    }

    /// Get authorization header with access token
    async fn get_auth_header(&mut self) -> Result<String, PaymentError> {
        let token = self.get_access_token().await?;
        Ok(format!("Bearer {}", token))
    }
}

#[async_trait]
impl PaymentProcessor for PayPalPaymentProcessor {
    async fn process_payment(&self, invoice: &Invoice, payment_data: PaymentData) -> Result<PaymentResult, PaymentError> {
        if payment_data.provider != PaymentProvider::PayPal {
            return Err(PaymentError::InvalidPaymentData(
                "Payment data is not for PayPal".to_string()
            ));
        }

        // Clone self to mutate access_token in get_auth_header
        let mut this = self.clone();
        let auth_header = this.get_auth_header().await?;

        // Create a payment
        let payment_data = PayPalPaymentCreate {
            intent: "sale".to_string(),
            payer: PayPalPayer {
                payment_method: "paypal".to_string(),
            },
            transactions: vec![PayPalTransaction {
                amount: PayPalAmount {
                    total: format!("{:.2}", invoice.total_amount),
                    currency: "USD".to_string(),
                },
                description: Some(format!("Invoice {} for {}", invoice.id, invoice.client_name)),
                invoice_number: Some(invoice.id.to_string()),
            }],
            redirect_urls: PayPalRedirectUrls {
                return_url: "https://yourdomain.com/payment/return".to_string(),
                cancel_url: "https://yourdomain.com/payment/cancel".to_string(),
            },
        };

        let url = format!("{}/v1/payments/payment", self.base_url);
        
        let response = self.client
            .post(&url)
            .header("Authorization", auth_header)
            .header("Content-Type", "application/json")
            .json(&payment_data)
            .send()
            .await
            .map_err(|e| PaymentError::NetworkError(format!("Failed to create payment: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await
                .map_err(|e| PaymentError::NetworkError(format!("Failed to read error response: {}", e)))?;
            return Err(PaymentError::ProviderError(format!("PayPal API error: {}", error_text)));
        }

        let payment: PayPalPayment = response
            .json()
            .await
            .map_err(|e| PaymentError::ProviderError(format!("Failed to parse payment response: {}", e)))?;

        // Execute the payment with the provided token (which should be the PayPal payment ID)
        let execute_url = format!("{}/v1/payments/payment/{}/execute", self.base_url, payment.id);
        let execute_data = PayPalPaymentExecute {
            payer_id: payment_data.token,
        };

        let execute_response = self.client
            .post(&execute_url)
            .header("Authorization", auth_header)
            .header("Content-Type", "application/json")
            .json(&execute_data)
            .send()
            .await
            .map_err(|e| PaymentError::NetworkError(format!("Failed to execute payment: {}", e)))?;

        if !execute_response.status().is_success() {
            let error_text = execute_response.text().await
                .map_err(|e| PaymentError::NetworkError(format!("Failed to read error response: {}", e)))?;
            return Err(PaymentError::ProviderError(format!("PayPal API error: {}", error_text)));
        }

        let executed_payment: PayPalPayment = execute_response
            .json()
            .await
            .map_err(|e| PaymentError::ProviderError(format!("Failed to parse executed payment response: {}", e)))?;

        // Determine the result based on the payment state
        let result = match executed_payment.state.as_str() {
            "approved" => PaymentResult::Success(PaymentProvider::PayPal, executed_payment.id),
            "pending" => PaymentResult::Pending,
            "created" => {
                // Payment created but not yet approved
                PaymentResult::Pending
            }
            _ => PaymentResult::Failed,
        };

        Ok(result)
    }

    async fn get_payment_status(&self, _provider: PaymentProvider, payment_id: &str) -> Result<crate::domain::payment::PaymentStatus, PaymentError> {
        // Clone self to mutate access_token in get_auth_header
        let mut this = self.clone();
        let auth_header = this.get_auth_header().await?;

        let url = format!("{}/v1/payments/payment/{}", self.base_url, payment_id);
        
        let response = self.client
            .get(&url)
            .header("Authorization", auth_header)
            .send()
            .await
            .map_err(|e| PaymentError::NetworkError(format!("Failed to fetch payment: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await
                .map_err(|e| PaymentError::NetworkError(format!("Failed to read error response: {}", e)))?;
            return Err(PaymentError::ProviderError(format!("PayPal API error: {}", error_text)));
        }

        let payment: PayPalPayment = response
            .json()
            .await
            .map_err(|e| PaymentError::ProviderError(format!("Failed to parse payment response: {}", e)))?;

        // Map PayPal state to our PaymentStatus
        let status = match payment.state.as_str() {
            "approved" => crate::domain::payment::PaymentStatus::Paid,
            "pending" => crate::domain::payment::PaymentStatus::Pending,
            "created" => crate::domain::payment::PaymentStatus::Pending,
            "failed" | "cancelled" | "expired" => crate::domain::payment::PaymentStatus::PaymentFailed,
            _ => crate::domain::payment::PaymentStatus::PaymentFailed,
        };

        Ok(status)
    }
}

// PayPal API request/response structures

#[derive(Debug, Deserialize)]
struct PayPalAccessTokenResponse {
    #[serde(rename = "access_token")]
    access_token: String,
    #[serde(rename = "token_type")]
    token_type: String,
    #[serde(rename = "expires_in")]
    expires_in: u32,
}

#[derive(Debug, Serialize, Clone)]
struct PayPalPaymentCreate {
    intent: String,
    payer: PayPalPayer,
    transactions: Vec<PayPalTransaction>,
    #[serde(rename = "redirect_urls")]
    redirect_urls: PayPalRedirectUrls,
}

#[derive(Debug, Serialize, Clone)]
struct PayPalPayer {
    #[serde(rename = "payment_method")]
    payment_method: String,
}

#[derive(Debug, Serialize, Clone)]
struct PayPalTransaction {
    amount: PayPalAmount,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(rename = "invoice_number", skip_serializing_if = "Option::is_none")]
    invoice_number: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
struct PayPalAmount {
    total: String,
    currency: String,
}

#[derive(Debug, Serialize, Clone)]
struct PayPalRedirectUrls {
    #[serde(rename = "return_url")]
    return_url: String,
    #[serde(rename = "cancel_url")]
    cancel_url: String,
}

#[derive(Debug, Deserialize)]
struct PayPalPayment {
    id: String,
    #[serde(rename = "object")]
    object: Option<String>,
    state: String,
    #[serde(rename = "intent")]
    intent: String,
    #[serde(rename = "payer")]
    payer: PayPalPayerInfo,
    #[serde(rename = "transactions")]
    transactions: Vec<PayPalTransactionInfo>,
}

#[derive(Debug, Deserialize)]
struct PayPalPayerInfo {
    #[serde(rename = "payment_method")]
    payment_method: String,
}

#[derive(Debug, Deserialize)]
struct PayPalTransactionInfo {
    amount: PayPalAmount,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

#[derive(Debug, Serialize)]
struct PayPalPaymentExecute {
    #[serde(rename = "payer_id")]
    payer_id: String,
}

/// Secure key manager for storing PayPal API credentials
pub struct PayPalKeyManager {
    // In a real implementation, this would securely store and retrieve keys
    // For now, we'll just store them in memory
    encrypted_client_id: Option<String>,
    encrypted_secret: Option<String>,
}

impl PayPalKeyManager {
    pub fn new() -> Self {
        Self {
            encrypted_client_id: None,
            encrypted_secret: None,
        }
    }

    /// Store and encrypt PayPal API credentials
    pub fn store_credentials(&mut self, client_id: &str, secret: &str) -> Result<(), PaymentError> {
        // In a real implementation, we would encrypt the credentials using cpc-net encryption
        // For now, we'll just store them as-is (NOT secure for production)
        self.encrypted_client_id = Some(client_id.to_string());
        self.encrypted_secret = Some(secret.to_string());
        Ok(())
    }

    /// Retrieve and decrypt PayPal API credentials
    pub fn retrieve_credentials(&self) -> Result<(String, String), PaymentError> {
        // In a real implementation, we would decrypt the credentials using cpc-net encryption
        // For now, we'll just return them as-is
        let client_id = self.encrypted_client_id.clone().ok_or(PaymentError::AuthenticationError(
            "No client ID stored".to_string()
        ))?;
        let secret = self.encrypted_secret.clone().ok_or(PaymentError::AuthenticationError(
            "No secret stored".to_string()
        ))?;
        Ok((client_id, secret))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_paypal_key_manager() {
        let mut key_manager = PayPalKeyManager::new();
        let client_id = "client_id_1234567890";
        let secret = "secret_1234567890";
        
        // Store credentials
        assert!(key_manager.store_credentials(client_id, secret).is_ok());
        
        // Retrieve credentials
        let retrieved_credentials = key_manager.retrieve_credentials();
        assert!(retrieved_credentials.is_ok());
        assert_eq!(retrieved_credentials.unwrap(), (client_id.to_string(), secret.to_string()));
    }

    #[tokio::test]
    async fn test_paypal_payment_processor_creation() {
        let processor = PayPalPaymentProcessor::new(
            "client_id_1234567890".to_string(),
            "secret_1234567890".to_string()
        );
        assert_eq!(processor.client_id, "client_id_1234567890");
        assert_eq!(processor.secret, "secret_1234567890");
    }
}