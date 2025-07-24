//! Transaction management for accounting system

use super::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A financial transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub date: DateTime<Utc>,
    pub description: String,
    pub journal_entries: Vec<JournalEntry>,
    pub reference: Option<String>,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Transaction {
    pub fn new(
        date: DateTime<Utc>,
        description: String,
        reference: Option<String>,
        tags: Vec<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            date,
            description,
            journal_entries: Vec::new(),
            reference,
            tags,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn add_entry(&mut self, entry: JournalEntry) -> Result<(), AccountingError> {
        self.journal_entries.push(entry);
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn validate(&self) -> Result<(), AccountingError> {
        // Check if transaction is balanced
        let total_debits: Money = self.journal_entries
            .iter()
            .filter(|e| e.entry_type == EntryType::Debit)
            .map(|e| e.amount.clone())
            .try_fold(Money::new(0.0, "USD"), |acc, amount| acc.add(&amount))?;

        let total_credits: Money = self.journal_entries
            .iter()
            .filter(|e| e.entry_type == EntryType::Credit)
            .map(|e| e.amount.clone())
            .try_fold(Money::new(0.0, "USD"), |acc, amount| acc.add(&amount))?;

        if total_debits != total_credits {
            return Err(AccountingError::UnbalancedTransaction {
                debits: total_debits,
                credits: total_credits,
            });
        }

        // Check if we have at least one debit and one credit
        let has_debit = self.journal_entries.iter().any(|e| e.entry_type == EntryType::Debit);
        let has_credit = self.journal_entries.iter().any(|e| e.entry_type == EntryType::Credit);

        if !has_debit || !has_credit {
            return Err(AccountingError::InvalidTransactionStructure);
        }

        Ok(())
    }

    pub fn total_amount(&self) -> Result<Money, AccountingError> {
        self.journal_entries
            .iter()
            .map(|e| &e.amount)
            .try_fold(Money::new(0.0, "USD"), |acc, amount| acc.add(amount))
    }
}

/// Individual journal entry within a transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalEntry {
    pub account_id: Uuid,
    pub entry_type: EntryType,
    pub amount: Money,
    pub description: Option<String>,
}

impl JournalEntry {
    pub fn new(
        account_id: Uuid,
        entry_type: EntryType,
        amount: Money,
        description: Option<String>,
    ) -> Self {
        Self {
            account_id,
            entry_type,
            amount,
            description,
        }
    }
}

/// Entry type for double-entry bookkeeping
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum EntryType {
    Debit,
    Credit,
}

impl std::fmt::Display for EntryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntryType::Debit => write!(f, "Debit"),
            EntryType::Credit => write!(f, "Credit"),
        }
    }
}