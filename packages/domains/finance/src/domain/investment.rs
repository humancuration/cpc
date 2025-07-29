use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::domain::primitives::Money;

/// Asset class for investments
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AssetClass {
    Stock,
    Bond,
    ETF,
    Crypto,
    RealEstate,
    Commodities,
}

/// Risk level for investments
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
}

/// Investment entity representing a financial investment
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Investment {
    pub id: Uuid,
    pub symbol: String,
    pub name: String,
    pub quantity: f64,
    pub purchase_price: Money,
    pub current_value: Money,
    pub asset_class: AssetClass, // Stock, Bond, ETF, Crypto
    pub risk_level: RiskLevel,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Investment {
    pub fn new(
        symbol: String,
        name: String,
        quantity: f64,
        purchase_price: Money,
        current_value: Money,
        asset_class: AssetClass,
        risk_level: RiskLevel,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            symbol,
            name,
            quantity,
            purchase_price,
            current_value,
            asset_class,
            risk_level,
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn update_quantity(&mut self, new_quantity: f64) {
        self.quantity = new_quantity;
        self.updated_at = Utc::now();
    }
    
    pub fn update_purchase_price(&mut self, new_price: Money) {
        self.purchase_price = new_price;
        self.updated_at = Utc::now();
    }
    
    pub fn update_current_value(&mut self, new_value: Money) {
        self.current_value = new_value;
        self.updated_at = Utc::now();
    }
    
    pub fn update_asset_class(&mut self, new_class: AssetClass) {
        self.asset_class = new_class;
        self.updated_at = Utc::now();
    }
    
    pub fn update_risk_level(&mut self, new_risk: RiskLevel) {
        self.risk_level = new_risk;
        self.updated_at = Utc::now();
    }
    
    /// Calculate the total purchase cost
    pub fn total_purchase_cost(&self) -> Money {
        Money::new(
            self.purchase_price.amount * self.quantity,
            &self.purchase_price.currency,
        )
    }
    
    /// Calculate the total current value
    pub fn total_current_value(&self) -> Money {
        Money::new(
            self.current_value.amount * self.quantity,
            &self.current_value.currency,
        )
    }
    
    /// Calculate the profit/loss
    pub fn profit_loss(&self) -> Money {
        let current = self.total_current_value().amount;
        let purchase = self.total_purchase_cost().amount;
        Money::new(
            current - purchase,
            &self.current_value.currency,
        )
    }
    
    /// Calculate the profit/loss percentage
    pub fn profit_loss_percentage(&self) -> f64 {
        let current = self.total_current_value().amount;
        let purchase = self.total_purchase_cost().amount;
        if purchase != 0.0 {
            ((current - purchase) / purchase) * 100.0
        } else {
            0.0
        }
    }
}