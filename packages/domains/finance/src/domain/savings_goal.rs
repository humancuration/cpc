//! Savings goals domain models

use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use super::primitives::{Money, Currency, FinancialError};

/// Dual currency target structure for savings goals
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DualCurrencyTarget {
    pub primary: Money,    // Traditional currency
    pub dabloons: Money,   // Dabloons component
}

impl DualCurrencyTarget {
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
    
    pub fn add(&self, amount: &Money) -> Result<Self, FinancialError> {
        match amount.currency {
            Currency::Dabloons => Ok(Self {
                primary: self.primary.clone(),
                dabloons: self.dabloons.add(amount)?,
            }),
            _ => Ok(Self {
                primary: self.primary.add(amount)?,
                dabloons: self.dabloons.clone(),
            }),
        }
    }
}

/// Savings currency type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SavingsCurrencyType {
    TraditionalOnly,
    DabloonsOnly,
    Mixed,
}

/// Savings goal model for tracking savings objectives
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SavingsGoal {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub target: DualCurrencyTarget,
    pub current: DualCurrencyTarget,
    pub currency_type: SavingsCurrencyType,
    pub target_date: DateTime<Utc>,
    pub auto_deduct: bool,
    pub deduction_percentage: Decimal,
    pub description: Option<String>,
    pub category: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

impl SavingsGoal {
    pub fn new(
        user_id: Uuid,
        name: String,
        target_amount: Money,
        target_date: DateTime<Utc>,
    ) -> Self {
        let currency_type = if target_amount.currency == Currency::Dabloons {
            SavingsCurrencyType::DabloonsOnly
        } else {
            SavingsCurrencyType::TraditionalOnly
        };
        
        let target = DualCurrencyTarget {
            primary: if target_amount.currency == Currency::Dabloons {
                Money::zero(Currency::USD) // Default to USD for primary
            } else {
                target_amount.clone()
            },
            dabloons: if target_amount.currency == Currency::Dabloons {
                target_amount.clone()
            } else {
                Money::zero(Currency::Dabloons)
            },
        };
        
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            name,
            target,
            current: DualCurrencyTarget::zero(target_amount.currency),
            currency_type,
            target_date,
            auto_deduct: false,
            deduction_percentage: Decimal::ZERO,
            description: None,
            category: None,
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn new_mixed(
        user_id: Uuid,
        name: String,
        primary_target: Money,
        dabloons_target: Money,
        target_date: DateTime<Utc>,
    ) -> Result<Self, FinancialError> {
        if primary_target.currency == Currency::Dabloons {
            return Err(FinancialError::InvalidCurrency);
        }
        if dabloons_target.currency != Currency::Dabloons {
            return Err(FinancialError::InvalidCurrency);
        }
        
        let target = DualCurrencyTarget::new(primary_target, dabloons_target)?;
        
        let now = Utc::now();
        Ok(Self {
            id: Uuid::new_v4(),
            user_id,
            name,
            target,
            current: DualCurrencyTarget::zero(primary_target.currency),
            currency_type: SavingsCurrencyType::Mixed,
            target_date,
            auto_deduct: false,
            deduction_percentage: Decimal::ZERO,
            description: None,
            category: None,
            created_at: now,
            updated_at: now,
        })
    }

    pub fn progress_percentage(&self) -> Decimal {
        // For mixed goals, we need a conversion rate
        // This is a simplified implementation - in a real system, this would be configurable
        let dabloons_conversion_rate = Decimal::new(1, 2); // 1 Dabloon = 0.01 USD (example rate)
        
        let total_target = self.target.primary.amount +
            self.target.dabloons.amount * dabloons_conversion_rate;
        let total_current = self.current.primary.amount +
            self.current.dabloons.amount * dabloons_conversion_rate;
        
        if total_target.is_zero() {
            Decimal::ZERO
        } else {
            (total_current / total_target) * Decimal::from(100)
        }
    }

    pub fn remaining_amount(&self) -> DualCurrencyTarget {
        DualCurrencyTarget {
            primary: self.target.primary.subtract(&self.current.primary).unwrap_or_else(|_| {
                Money::zero(self.target.primary.currency.clone())
            }),
            dabloons: self.target.dabloons.subtract(&self.current.dabloons).unwrap_or_else(|_| {
                Money::zero(Currency::Dabloons)
            }),
        }
    }

    pub fn is_complete(&self) -> bool {
        match self.currency_type {
            SavingsCurrencyType::TraditionalOnly => {
                self.current.primary.amount >= self.target.primary.amount
            },
            SavingsCurrencyType::DabloonsOnly => {
                self.current.dabloons.amount >= self.target.dabloons.amount
            },
            SavingsCurrencyType::Mixed => {
                // For mixed goals, check if both currencies meet their targets
                (self.current.primary.amount >= self.target.primary.amount) &&
                (self.current.dabloons.amount >= self.target.dabloons.amount)
            },
        }
    }

    pub fn days_until_target(&self) -> i64 {
        (self.target_date - Utc::now()).num_days()
    }

    pub fn monthly_savings_needed(&self) -> DualCurrencyTarget {
        let remaining = self.remaining_amount();
        let days = self.days_until_target();
        
        if days <= 0 {
            return remaining;
        }
        
        let months = Decimal::from(days) / Decimal::from(30);
        if months.is_zero() {
            return remaining;
        }
        
        let primary_amount = if months.is_zero() {
            remaining.primary.amount
        } else {
            remaining.primary.amount / months
        };
        
        let dabloons_amount = if months.is_zero() {
            remaining.dabloons.amount
        } else {
            remaining.dabloons.amount / months
        };
        
        DualCurrencyTarget {
            primary: Money::new(primary_amount, remaining.primary.currency),
            dabloons: Money::new(dabloons_amount, Currency::Dabloons),
        }
    }
    
    pub fn add_contribution(&mut self, amount: Money) -> Result<(), FinancialError> {
        self.current = self.current.add(&amount)?;
        self.updated_at = Utc::now();
        Ok(())
    }
}

/// Savings progress tracking structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavingsProgress {
    pub goal_id: Uuid,
    pub current: DualCurrencyTarget,
    pub target: DualCurrencyTarget,
    pub percentage: Decimal,
    pub days_remaining: i64,
    pub monthly_savings_needed: DualCurrencyTarget,
}

impl SavingsProgress {
    pub fn from_goal(goal: &SavingsGoal) -> Self {
        Self {
            goal_id: goal.id,
            current: goal.current.clone(),
            target: goal.target.clone(),
            percentage: goal.progress_percentage(),
            days_remaining: goal.days_until_target(),
            monthly_savings_needed: goal.monthly_savings_needed(),
        }
    }
}
}