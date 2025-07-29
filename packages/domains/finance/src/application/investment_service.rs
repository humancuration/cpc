use crate::domain::{Investment, AssetClass, RiskLevel, Money};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Service for managing investments
pub struct InvestmentService {
    investments: HashMap<Uuid, Investment>,
}

impl InvestmentService {
    pub fn new() -> Self {
        Self {
            investments: HashMap::new(),
        }
    }
    
    /// Create a new investment
    pub fn create_investment(
        &mut self,
        symbol: String,
        name: String,
        quantity: f64,
        purchase_price: Money,
        current_value: Money,
        asset_class: AssetClass,
        risk_level: RiskLevel,
    ) -> Result<Investment, String> {
        let investment = Investment::new(
            symbol,
            name,
            quantity,
            purchase_price,
            current_value,
            asset_class,
            risk_level,
        );
        self.investments.insert(investment.id, investment.clone());
        Ok(investment)
    }
    
    /// Get an investment by ID
    pub fn get_investment(&self, id: &Uuid) -> Option<&Investment> {
        self.investments.get(id)
    }
    
    /// Get all investments
    pub fn get_all_investments(&self) -> Vec<&Investment> {
        self.investments.values().collect()
    }
    
    /// Update quantity for an investment
    pub fn update_quantity(
        &mut self,
        id: &Uuid,
        new_quantity: f64,
    ) -> Result<(), String> {
        match self.investments.get_mut(id) {
            Some(investment) => {
                investment.update_quantity(new_quantity);
                Ok(())
            }
            None => Err("Investment not found".to_string()),
        }
    }
    
    /// Update purchase price for an investment
    pub fn update_purchase_price(
        &mut self,
        id: &Uuid,
        new_price: Money,
    ) -> Result<(), String> {
        match self.investments.get_mut(id) {
            Some(investment) => {
                investment.update_purchase_price(new_price);
                Ok(())
            }
            None => Err("Investment not found".to_string()),
        }
    }
    
    /// Update current value for an investment
    pub fn update_current_value(
        &mut self,
        id: &Uuid,
        new_value: Money,
    ) -> Result<(), String> {
        match self.investments.get_mut(id) {
            Some(investment) => {
                investment.update_current_value(new_value);
                Ok(())
            }
            None => Err("Investment not found".to_string()),
        }
    }
    
    /// Update asset class for an investment
    pub fn update_asset_class(
        &mut self,
        id: &Uuid,
        new_class: AssetClass,
    ) -> Result<(), String> {
        match self.investments.get_mut(id) {
            Some(investment) => {
                investment.update_asset_class(new_class);
                Ok(())
            }
            None => Err("Investment not found".to_string()),
        }
    }
    
    /// Update risk level for an investment
    pub fn update_risk_level(
        &mut self,
        id: &Uuid,
        new_risk: RiskLevel,
    ) -> Result<(), String> {
        match self.investments.get_mut(id) {
            Some(investment) => {
                investment.update_risk_level(new_risk);
                Ok(())
            }
            None => Err("Investment not found".to_string()),
        }
    }
    
    /// Delete an investment
    pub fn delete_investment(&mut self, id: &Uuid) -> Result<(), String> {
        match self.investments.remove(id) {
            Some(_) => Ok(()),
            None => Err("Investment not found".to_string()),
        }
    }
    
    /// Get investments by asset class
    pub fn get_investments_by_asset_class(&self, asset_class: &AssetClass) -> Vec<&Investment> {
        self.investments
            .values()
            .filter(|investment| &investment.asset_class == asset_class)
            .collect()
    }
    
    /// Get investments by risk level
    pub fn get_investments_by_risk_level(&self, risk_level: &RiskLevel) -> Vec<&Investment> {
        self.investments
            .values()
            .filter(|investment| &investment.risk_level == risk_level)
            .collect()
    }
    
    /// Get total investment value
    pub fn get_total_investment_value(&self) -> Money {
        // For simplicity, we'll assume all investments are in the same currency
        // In a real implementation, we'd need to handle currency conversion
        let mut total_value = 0.0;
        let mut currency = "USD".to_string();
        
        for investment in self.investments.values() {
            total_value += investment.total_current_value().amount;
            currency = investment.current_value.currency.clone();
        }
        
        Money::new(total_value, &currency)
    }
    
    /// Get total profit/loss
    pub fn get_total_profit_loss(&self) -> Money {
        // For simplicity, we'll assume all investments are in the same currency
        // In a real implementation, we'd need to handle currency conversion
        let mut total_pl = 0.0;
        let mut currency = "USD".to_string();
        
        for investment in self.investments.values() {
            total_pl += investment.profit_loss().amount;
            currency = investment.current_value.currency.clone();
        }
        
        Money::new(total_pl, &currency)
    }
    
    /// Get overall profit/loss percentage
    pub fn get_overall_profit_loss_percentage(&self) -> f64 {
        let mut total_purchase = 0.0;
        let mut total_current = 0.0;
        
        for investment in self.investments.values() {
            total_purchase += investment.total_purchase_cost().amount;
            total_current += investment.total_current_value().amount;
        }
        
        if total_purchase > 0.0 {
            ((total_current - total_purchase) / total_purchase) * 100.0
        } else {
            0.0
        }
    }
}