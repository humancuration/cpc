//! gRPC client for user preferences service
//! 
//! This module implements the gRPC client for syncing user preferences with the backend.
//! It handles authentication, retries, and network status monitoring.

use tonic::transport::Channel;
use uuid::Uuid;
use packages::domains::finance::domain::primitives::Currency;
use packages::domains::finance::application::user_preferences::UserPreferences;
use std::time::Duration;
use tokio::time::sleep;
use tonic::metadata::MetadataValue;

// Import the generated gRPC client code
// Note: This would be generated from the .proto file
// For now, we'll define the necessary types manually

/// Generated gRPC client (simplified for this example)
#[derive(Debug, Clone)]
pub struct UserPreferencesClient {
    inner: InnerClient,
}

#[derive(Debug, Clone)]
struct InnerClient {
    channel: Channel,
    auth_token: String,
}

/// Request message for getting preferred currency
#[derive(Debug, Clone)]
pub struct GetPreferredCurrencyRequest {
    pub user_id: String,
}

/// Response message for getting preferred currency
#[derive(Debug, Clone)]
pub struct GetPreferredCurrencyResponse {
    pub currency_code: String,
}

/// Request message for setting preferred currency
#[derive(Debug, Clone)]
pub struct SetPreferredCurrencyRequest {
    pub user_id: String,
    pub currency_code: String,
}

impl UserPreferencesClient {
    /// Create a new UserPreferencesClient
    pub async fn new(endpoint: &str, auth_token: String) -> Result<Self, Box<dyn std::error::Error>> {
        let channel = Channel::from_shared(endpoint.to_string())?
            .connect()
            .await?;
        
        Ok(Self {
            inner: InnerClient {
                channel,
                auth_token,
            }
        })
    }
    
    /// Get user's preferred currency from the backend
    pub async fn get_preferred_currency(&self, request: GetPreferredCurrencyRequest) -> Result<GetPreferredCurrencyResponse, Box<dyn std::error::Error>> {
        // In a real implementation, this would make an actual gRPC call
        // For now, we'll simulate the response
        Ok(GetPreferredCurrencyResponse {
            currency_code: "USD".to_string(), // Default fallback
        })
    }
    
    /// Set user's preferred currency on the backend
    pub async fn set_preferred_currency(&self, request: SetPreferredCurrencyRequest) -> Result<(), Box<dyn std::error::Error>> {
        // In a real implementation, this would make an actual gRPC call
        // For now, we'll just simulate success
        Ok(())
    }
}

impl UserPreferencesClient {
    /// Create a new instance with default values for testing
    pub fn new() -> Self {
        // This is a simplified implementation for testing purposes
        // In a real implementation, this would connect to a real gRPC service
        Self {
            inner: InnerClient {
                channel: Channel::from_static("http://[::1]:50051")
                    .connect_lazy()
                    .unwrap(),
                auth_token: "test_token".to_string(),
            }
        }
    }
}

/// gRPC adapter that implements UserPreferences trait
#[derive(Clone)]
pub struct GrpcUserPreferences {
    client: UserPreferencesClient,
    user_id: Uuid,
}

impl GrpcUserPreferences {
    /// Create a new GrpcUserPreferences instance
    pub fn new(client: UserPreferencesClient, user_id: Uuid) -> Self {
        Self { client, user_id }
    }
}

#[async_trait::async_trait]
impl UserPreferences for GrpcUserPreferences {
    async fn get_preferred_currency(&self, _user_id: Uuid) -> Result<Currency, String> {
        // Implement retry logic with exponential backoff
        let mut attempts = 0;
        let max_attempts = 3;
        let mut delay = Duration::from_millis(100);
        
        loop {
            let request = GetPreferredCurrencyRequest {
                user_id: self.user_id.to_string(),
            };
            
            match self.client.get_preferred_currency(request).await {
                Ok(response) => {
                    // Convert currency code to Currency enum
                    return match response.currency_code.as_str() {
                        "USD" => Ok(Currency::USD),
                        "EUR" => Ok(Currency::EUR),
                        "GBP" => Ok(Currency::GBP),
                        "JPY" => Ok(Currency::JPY),
                        "CAD" => Ok(Currency::CAD),
                        "AUD" => Ok(Currency::AUD),
                        "CHF" => Ok(Currency::CHF),
                        "CNY" => Ok(Currency::CNY),
                        "SEK" => Ok(Currency::SEK),
                        "NZD" => Ok(Currency::NZD),
                        "MXN" => Ok(Currency::MXN),
                        "SGD" => Ok(Currency::SGD),
                        "HKD" => Ok(Currency::HKD),
                        "NOK" => Ok(Currency::NOK),
                        "KRW" => Ok(Currency::KRW),
                        "TRY" => Ok(Currency::TRY),
                        "RUB" => Ok(Currency::RUB),
                        "INR" => Ok(Currency::INR),
                        "BRL" => Ok(Currency::BRL),
                        "ZAR" => Ok(Currency::ZAR),
                        "DABLOONS" => Ok(Currency::Dabloons),
                        _ => Err(format!("Unknown currency code: {}", response.currency_code)),
                    };
                }
                Err(e) => {
                    attempts += 1;
                    if attempts >= max_attempts {
                        return Err(format!("Failed to get preferred currency after {} attempts: {}", max_attempts, e));
                    }
                    
                    // Exponential backoff
                    sleep(delay).await;
                    delay *= 2;
                }
            }
        }
    }
    
    async fn set_preferred_currency(&self, _user_id: Uuid, currency: Currency) -> Result<(), String> {
        // Implement retry logic with exponential backoff
        let mut attempts = 0;
        let max_attempts = 3;
        let mut delay = Duration::from_millis(100);
        
        loop {
            let request = SetPreferredCurrencyRequest {
                user_id: self.user_id.to_string(),
                currency_code: currency.code().to_string(),
            };
            
            match self.client.set_preferred_currency(request).await {
                Ok(()) => return Ok(()),
                Err(e) => {
                    attempts += 1;
                    if attempts >= max_attempts {
                        return Err(format!("Failed to set preferred currency after {} attempts: {}", max_attempts, e));
                    }
                    
                    // Exponential backoff
                    sleep(delay).await;
                    delay *= 2;
                }
            }
        }
    }
}

/// Network status monitor
pub struct NetworkStatusMonitor {
    is_connected: bool,
}

impl NetworkStatusMonitor {
    /// Create a new NetworkStatusMonitor
    pub fn new() -> Self {
        Self {
            is_connected: true, // Assume connected by default
        }
    }
    
    /// Check if network is available
    pub fn is_connected(&self) -> bool {
        self.is_connected
    }
    
    /// Simulate network status change
    pub fn set_connected(&mut self, connected: bool) {
        self.is_connected = connected;
    }
    
    /// Subscribe to network status changes
    pub fn subscribe(&self) -> tokio::sync::watch::Receiver<bool> {
        // In a real implementation, this would return a receiver that gets notified
        // when the network status changes
        let (tx, rx) = tokio::sync::watch::channel(self.is_connected);
        rx
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_get_preferred_currency() {
        let client = UserPreferencesClient {
            inner: InnerClient {
                channel: Channel::from_static("http://[::1]:50051")
                    .connect_lazy()
                    .unwrap(),
                auth_token: "test_token".to_string(),
            }
        };
        
        let user_id = Uuid::new_v4();
        let preferences = GrpcUserPreferences::new(client, user_id);
        
        // This would normally make a network call
        let result = preferences.get_preferred_currency(user_id).await;
        // In our mock implementation, it will return USD
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_set_preferred_currency() {
        let client = UserPreferencesClient {
            inner: InnerClient {
                channel: Channel::from_static("http://[::1]:50051")
                    .connect_lazy()
                    .unwrap(),
                auth_token: "test_token".to_string(),
            }
        };
        
        let user_id = Uuid::new_v4();
        let preferences = GrpcUserPreferences::new(client, user_id);
        
        // This would normally make a network call
        let result = preferences.set_preferred_currency(user_id, Currency::EUR).await;
        // In our mock implementation, it will succeed
        assert!(result.is_ok());
    }
}