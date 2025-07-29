//! Budget domain models

use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use super::primitives::{Money, Currency, FinancialError};

/// Allocation structure for dual-currency budgets
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BudgetAllocation {
    pub primary: Money,    // Traditional currency (USD)
    pub dabloons: Money,   // Dabloons component
}

impl BudgetAllocation {
    pub fn new(primary: Money, dabloons: Money) -> Result<Self, FinancialError> {
        if primary.currency == Currency::Dabloons {
            return Err(FinancialError::InvalidCurrency);
        }
        if dabloons.currency != Currency::Dabloons {
            return Err(FinancialError::InvalidCurrency);
        }
        
        Ok(Self { primary, dabloons })
    }
    
    pub fn zero(currency: Currency) -> Self {
        Self {
            primary: Money::zero(currency),
            dabloons: Money::zero(Currency::Dabloons),
        }
    }
    
    pub fn is_zero(&self) -> bool {
        self.primary.is_zero() && self.dabloons.is_zero()
    }
    
    pub fn add(&self, other: &Self) -> Result<Self, FinancialError> {
        Ok(Self {
            primary: self.primary.add(&other.primary)?,
            dabloons: self.dabloons.add(&other.dabloons)?,
        })
    }
    
    pub fn subtract(&self, other: &Self) -> Result<Self, FinancialError> {
        Ok(Self {
            primary: self.primary.subtract(&other.primary)?,
            dabloons: self.dabloons.subtract(&other.dabloons)?,
        })
    }
}

/// Budget currency type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BudgetCurrencyType {
    TraditionalOnly,
    DabloonsOnly,
    Mixed,
}

/// Budget model for tracking income allocation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Budget {
    pub id: Uuid,
    pub user_id: Uuid,
    pub category: String,
    pub allocation: BudgetAllocation,
    pub spent: BudgetAllocation,
    pub currency_type: BudgetCurrencyType,
    pub period: BudgetPeriod,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,

impl Budget {
    pub fn new(
        user_id: Uuid,
        category: String,
        allocated_amount: Money,
        period: BudgetPeriod,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Self {
        let currency_type = if allocated_amount.currency == Currency::Dabloons {
            BudgetCurrencyType::DabloonsOnly
        } else {
            BudgetCurrencyType::TraditionalOnly
        };
        
        let allocation = BudgetAllocation {
            primary: if allocated_amount.currency == Currency::Dabloons {
                Money::zero(Currency::USD) // Default to USD for primary
            } else {
                allocated_amount.clone()
            },
            dabloons: if allocated_amount.currency == Currency::Dabloons {
                allocated_amount.clone()
            } else {
                Money::zero(Currency::Dabloons)
            },
        };
        
        Self {
            id: Uuid::new_v4(),
            user_id,
            category,
            allocation,
            spent: BudgetAllocation::zero(allocated_amount.currency),
            currency_type,
            period,
            start_date,
            end_date,
        }
    }
    
    pub fn new_mixed(
        user_id: Uuid,
        category: String,
        primary_amount: Money,
        dabloons_amount: Money,
        period: BudgetPeriod,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Self, FinancialError> {
        if primary_amount.currency == Currency::Dabloons {
            return Err(FinancialError::InvalidCurrency);
        }
        if dabloons_amount.currency != Currency::Dabloons {
            return Err(FinancialError::InvalidCurrency);
        }
        
        let allocation = BudgetAllocation::new(primary_amount, dabloons_amount)?;
        
        Ok(Self {
            id: Uuid::new_v4(),
            user_id,
            category,
            allocation,
            spent: BudgetAllocation::zero(primary_amount.currency),
            currency_type: BudgetCurrencyType::Mixed,
            period,
            start_date,
            end_date,
        })
    }

    pub fn remaining_amount(&self) -> BudgetAllocation {
        self.allocation.subtract(&self.spent).unwrap_or_else(|_| {
            BudgetAllocation::zero(Currency::USD) // Default currency
        })
    }

    pub fn update_spent_with_dabloons(&mut self, amount: Money) -> Result<(), FinancialError> {
        if amount.currency != Currency::Dabloons {
            return Err(FinancialError::InvalidCurrency);
        }
        
        self.spent.dabloons = self.spent.dabloons.add(&amount)?;
        Ok(())
    }

    pub fn utilization_percentage(&self) -> Decimal {
        // For mixed budgets, we need a conversion rate
        // This is a simplified implementation - in a real system, this would be configurable
        let dabloons_conversion_rate = Decimal::new(1, 2); // 1 Dabloon = 0.01 USD (example rate)
        
        let total_allocated = self.allocation.primary.amount +
            self.allocation.dabloons.amount * dabloons_conversion_rate;
        let total_spent = self.spent.primary.amount +
            self.spent.dabloons.amount * dabloons_conversion_rate;
        
        if total_allocated.is_zero() {
            Decimal::ZERO
        } else {
            (total_spent / total_allocated) * Decimal::from(100)
        }
    }

    pub fn is_over_budget(&self) -> bool {
        match self.currency_type {
            BudgetCurrencyType::TraditionalOnly => {
                self.spent.primary.amount > self.allocation.primary.amount
            },
            BudgetCurrencyType::DabloonsOnly => {
                self.spent.dabloons.amount > self.allocation.dabloons.amount
            },
            BudgetCurrencyType::Mixed => {
                // For mixed budgets, check if either currency is over budget
                (self.spent.primary.amount > self.allocation.primary.amount) ||
                (self.spent.dabloons.amount > self.allocation.dabloons.amount)
            },
        }
    }
}
}

/// Budget period enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BudgetPeriod {
    Monthly,
    Weekly,
    BiWeekly,
    Custom,
}