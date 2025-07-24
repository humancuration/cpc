//! General ledger implementation

use super::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// General ledger containing all accounts and transactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ledger {
    pub id: Uuid,
    pub name: String,
    pub currency: String,
    pub accounts: HashMap<Uuid, Account>,
    pub transactions: Vec<Transaction>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Ledger {
    pub fn new(name: String, currency: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            currency: currency.clone(),
            accounts: HashMap::new(),
            transactions: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn add_account(&mut self, account: Account) -> Result<(), AccountingError> {
        if account.balance.currency != self.currency {
            return Err(AccountingError::CurrencyMismatch {
                expected: self.currency.clone(),
                actual: account.balance.currency.clone(),
            });
        }

        self.accounts.insert(account.id, account);
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn get_account(&self, account_id: Uuid) -> Option<&Account> {
        self.accounts.get(&account_id)
    }

    pub fn get_account_mut(&mut self, account_id: Uuid) -> Option<&mut Account> {
        self.accounts.get_mut(&account_id)
    }

    pub fn record_transaction(&mut self, mut transaction: Transaction) -> Result<(), AccountingError> {
        // Validate transaction
        transaction.validate()?;

        // Check currency consistency
        for entry in &transaction.journal_entries {
            let account = self.get_account(entry.account_id)
                .ok_or(AccountingError::AccountNotFound(entry.account_id))?;
            
            if account.balance.currency != self.currency {
                return Err(AccountingError::CurrencyMismatch {
                    expected: self.currency.clone(),
                    actual: account.balance.currency.clone(),
                });
            }
        }

        // Update account balances
        for entry in &transaction.journal_entries {
            if let Some(account) = self.get_account_mut(entry.account_id) {
                let amount = match entry.entry_type {
                    EntryType::Debit => entry.amount.clone(),
                    EntryType::Credit => entry.amount.negate()?,
                };
                account.update_balance(amount)?;
            }
        }

        // Add transaction to ledger
        self.transactions.push(transaction);
        self.updated_at = Utc::now();
        
        Ok(())
    }

    pub fn get_balance(&self, account_id: Uuid) -> Result<Money, AccountingError> {
        self.get_account(account_id)
            .map(|account| account.balance.clone())
            .ok_or(AccountingError::AccountNotFound(account_id))
    }

    pub fn get_trial_balance(&self) -> Result<TrialBalance, AccountingError> {
        let mut total_debits = Money::new(0.0, &self.currency);
        let mut total_credits = Money::new(0.0, &self.currency);

        let mut accounts = Vec::new();

        for account in self.accounts.values() {
            if account.is_active {
                let balance = account.balance.clone();
                
                let (debit_balance, credit_balance) = match account.account_type {
                    AccountType::Asset | AccountType::Expense => {
                        if balance.amount >= 0 {
                            (balance.clone(), Money::new(0.0, &self.currency))
                        } else {
                            (Money::new(0.0, &self.currency), balance.negate()?)
                        }
                    }
                    AccountType::Liability | AccountType::Equity | AccountType::Revenue => {
                        if balance.amount >= 0 {
                            (Money::new(0.0, &self.currency), balance.clone())
                        } else {
                            (balance.negate()?, Money::new(0.0, &self.currency))
                        }
                    }
                };

                total_debits = total_debits.add(&debit_balance)?;
                total_credits = total_credits.add(&credit_balance)?;

                accounts.push(TrialBalanceEntry {
                    account_id: account.id,
                    account_name: account.name.clone(),
                    account_code: account.code.clone(),
                    account_type: account.account_type,
                    debit_balance,
                    credit_balance,
                });
            }
        }

        Ok(TrialBalance {
            accounts,
            total_debits,
            total_credits,
            is_balanced: total_debits == total_credits,
        })
    }

    pub fn get_income_statement(&self) -> Result<IncomeStatement, AccountingError> {
        let mut revenue = Money::new(0.0, &self.currency);
        let mut expenses = Money::new(0.0, &self.currency);

        let mut revenue_accounts = Vec::new();
        let mut expense_accounts = Vec::new();

        for account in self.accounts.values() {
            if account.is_active {
                match account.account_type {
                    AccountType::Revenue => {
                        revenue = revenue.add(&account.balance)?;
                        revenue_accounts.push(IncomeStatementAccount {
                            account_id: account.id,
                            account_name: account.name.clone(),
                            amount: account.balance.clone(),
                        });
                    }
                    AccountType::Expense => {
                        expenses = expenses.add(&account.balance)?;
                        expense_accounts.push(IncomeStatementAccount {
                            account_id: account.id,
                            account_name: account.name.clone(),
                            amount: account.balance.clone(),
                        });
                    }
                    _ => {}
                }
            }
        }

        let net_income = revenue.subtract(&expenses)?;

        Ok(IncomeStatement {
            revenue,
            expenses,
            net_income,
            revenue_accounts,
            expense_accounts,
        })
    }

    pub fn get_balance_sheet(&self) -> Result<BalanceSheet, AccountingError> {
        let mut assets = Money::new(0.0, &self.currency);
        let mut liabilities = Money::new(0.0, &self.currency);
        let mut equity = Money::new(0.0, &self.currency);

        let mut asset_accounts = Vec::new();
        let mut liability_accounts = Vec::new();
        let mut equity_accounts = Vec::new();

        for account in self.accounts.values() {
            if account.is_active {
                match account.account_type {
                    AccountType::Asset => {
                        assets = assets.add(&account.balance)?;
                        asset_accounts.push(BalanceSheetAccount {
                            account_id: account.id,
                            account_name: account.name.clone(),
                            amount: account.balance.clone(),
                        });
                    }
                    AccountType::Liability => {
                        liabilities = liabilities.add(&account.balance)?;
                        liability_accounts.push(BalanceSheetAccount {
                            account_id: account.id,
                            account_name: account.name.clone(),
                            amount: account.balance.clone(),
                        });
                    }
                    AccountType::Equity => {
                        equity = equity.add(&account.balance)?;
                        equity_accounts.push(BalanceSheetAccount {
                            account_id: account.id,
                            account_name: account.name.clone(),
                            amount: account.balance.clone(),
                        });
                    }
                    _ => {}
                }
            }
        }

        let total_liabilities_equity = liabilities.add(&equity)?;
        let is_balanced = assets == total_liabilities_equity;

        Ok(BalanceSheet {
            assets,
            liabilities,
            equity,
            total_liabilities_equity,
            is_balanced,
            asset_accounts,
            liability_accounts,
            equity_accounts,
        })
    }
}

/// Trial balance report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrialBalance {
    pub accounts: Vec<TrialBalanceEntry>,
    pub total_debits: Money,
    pub total_credits: Money,
    pub is_balanced: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrialBalanceEntry {
    pub account_id: Uuid,
    pub account_name: String,
    pub account_code: String,
    pub account_type: AccountType,
    pub debit_balance: Money,
    pub credit_balance: Money,
}

/// Income statement report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncomeStatement {
    pub revenue: Money,
    pub expenses: Money,
    pub net_income: Money,
    pub revenue_accounts: Vec<IncomeStatementAccount>,
    pub expense_accounts: Vec<IncomeStatementAccount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncomeStatementAccount {
    pub account_id: Uuid,
    pub account_name: String,
    pub amount: Money,
}

/// Balance sheet report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceSheet {
    pub assets: Money,
    pub liabilities: Money,
    pub equity: Money,
    pub total_liabilities_equity: Money,
    pub is_balanced: bool,
    pub asset_accounts: Vec<BalanceSheetAccount>,
    pub liability_accounts: Vec<BalanceSheetAccount>,
    pub equity_accounts: Vec<BalanceSheetAccount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceSheetAccount {
    pub account_id: Uuid,
    pub account_name: String,
    pub amount: Money,
}