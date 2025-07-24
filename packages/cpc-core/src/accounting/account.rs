//! Account management for accounting system

use super::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Chart of accounts entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub account_type: AccountType,
    pub parent_id: Option<Uuid>,
    pub balance: Money,
    pub is_active: bool,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Account {
    pub fn new(
        code: String,
        name: String,
        account_type: AccountType,
        parent_id: Option<Uuid>,
        currency: &str,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            code,
            name,
            account_type,
            parent_id,
            balance: Money::new(0.0, currency),
            is_active: true,
            description: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update_balance(&mut self, amount: Money) -> Result<(), AccountingError> {
        if !self.is_active {
            return Err(AccountingError::InactiveAccount(self.id));
        }
        self.balance = self.balance.add(&amount)?;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.updated_at = Utc::now();
    }

    pub fn activate(&mut self) {
        self.is_active = true;
        self.updated_at = Utc::now();
    }
}

/// Account types following standard accounting practices
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AccountType {
    Asset,
    Liability,
    Equity,
    Revenue,
    Expense,
}

impl AccountType {
    pub fn is_debit(&self) -> bool {
        matches!(self, AccountType::Asset | AccountType::Expense)
    }

    pub fn is_credit(&self) -> bool {
        matches!(self, AccountType::Liability | AccountType::Equity | AccountType::Revenue)
    }
}

impl std::fmt::Display for AccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccountType::Asset => write!(f, "Asset"),
            AccountType::Liability => write!(f, "Liability"),
            AccountType::Equity => write!(f, "Equity"),
            AccountType::Revenue => write!(f, "Revenue"),
            AccountType::Expense => write!(f, "Expense"),
        }
    }
}