//! Primitive value objects for the SCM module

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, NaiveTime};
use rust_decimal::Decimal;
use uuid::Uuid;
use std::collections::HashMap;

/// Money value object with currency support
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Money {
    pub amount: Decimal,
    pub currency: Currency,
}

impl Money {
    pub fn new(amount: Decimal, currency: Currency) -> Self {
        Self { amount, currency }
    }

    pub fn zero(currency: Currency) -> Self {
        Self {
            amount: Decimal::ZERO,
            currency,
        }
    }

    pub fn add(&self, other: &Money) -> Result<Money, DomainError> {
        if self.currency != other.currency {
            return Err(DomainError::CurrencyMismatch);
        }
        Ok(Money {
            amount: self.amount + other.amount,
            currency: self.currency,
        })
    }

    pub fn multiply(&self, factor: Decimal) -> Money {
        Money {
            amount: self.amount * factor,
            currency: self.currency,
        }
    }
}

/// Supported currencies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Currency {
    USD,
    EUR,
    GBP,
    JPY,
    CAD,
    AUD,
    CHF,
    CNY,
    SEK,
    NZD,
    MXN,
    SGD,
    HKD,
    NOK,
    KRW,
    TRY,
    RUB,
    INR,
    BRL,
    ZAR,
}

/// Geographic location with latitude and longitude
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeoLocation {
    pub latitude: f64,
    pub longitude: f64,
}

impl GeoLocation {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self { latitude, longitude }
    }

    /// Calculate distance between two locations using haversine formula
    pub fn distance_to(&self, other: &GeoLocation) -> f64 {
        let r = 6371.0; // Earth radius in kilometers
        let d_lat = (other.latitude - self.latitude).to_radians();
        let d_lon = (other.longitude - self.longitude).to_radians();
        let lat1 = self.latitude.to_radians();
        let lat2 = other.latitude.to_radians();
        
        let a = (d_lat / 2.0).sin().powi(2) + 
                lat1.cos() * lat2.cos() * (d_lon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
        r * c
    }
}

/// Operating hours for a facility
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OperatingHours {
    pub open_time: NaiveTime,
    pub close_time: NaiveTime,
}

impl OperatingHours {
    pub fn new(open_time: NaiveTime, close_time: NaiveTime) -> Self {
        Self { open_time, close_time }
    }

    /// Check if a given time is within operating hours
    pub fn is_open_at(&self, time: NaiveTime) -> bool {
        time >= self.open_time && time <= self.close_time
    }
}

/// Data sharing levels for consent management
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DataSharingLevel {
    None,
    ViewOnly,
    Editable,
    FullAccess,
}

/// Node identifier in a supply chain network
pub type NodeId = Uuid;

/// Common domain errors for the SCM module
#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("Invalid operation: {message}")]
    InvalidOperation { message: String },
    
    #[error("Validation failed: {message}")]
    ValidationError { message: String },
    
    #[error("Currency mismatch in operation")]
    CurrencyMismatch,
    
    #[error("Insufficient inventory")]
    InsufficientInventory,
    
    #[error("Invalid status transition")]
    InvalidStatusTransition,
    
    #[error("Entity not found")]
    NotFound,
    
    #[error("Network validation error: {message}")]
    NetworkValidationError { message: String },
    
    #[error("Capacity exceeded")]
    CapacityExceeded,
}

/// Result type for domain operations
pub type Result<T> = std::result::Result<T, DomainError>;