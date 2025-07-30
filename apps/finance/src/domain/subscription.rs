use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::domain::primitives::{FinancialCategory, Money};

/// Billing cycle for subscriptions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BillingCycle {
    Monthly,
    Quarterly,
    Annual,
}

/// Payment method for subscriptions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PaymentMethod {
    CreditCard,
    DebitCard,
    BankTransfer,
    PayPal,
    Crypto,
}

/// Subscription entity representing a recurring payment
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Subscription {
    pub id: Uuid,
    pub name: String,
    pub amount: Money,
    pub billing_cycle: BillingCycle,
    pub next_payment_date: DateTime<Utc>,
    pub payment_method: PaymentMethod,
    pub category: FinancialCategory,
    pub auto_renew: bool,
    pub notification_days: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Subscription {
    pub fn new(
        name: String,
        amount: Money,
        billing_cycle: BillingCycle,
        next_payment_date: DateTime<Utc>,
        payment_method: PaymentMethod,
        category: FinancialCategory,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            amount,
            billing_cycle,
            next_payment_date,
            payment_method,
            category,
            auto_renew: true,
            notification_days: 3,
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn update_amount(&mut self, new_amount: Money) {
        self.amount = new_amount;
        self.updated_at = Utc::now();
    }
    
    pub fn update_next_payment_date(&mut self, new_date: DateTime<Utc>) {
        self.next_payment_date = new_date;
        self.updated_at = Utc::now();
    }
    
    pub fn toggle_auto_renew(&mut self) {
        self.auto_renew = !self.auto_renew;
        self.updated_at = Utc::now();
    }
    
    pub fn set_notification_days(&mut self, days: u32) {
        self.notification_days = days;
        self.updated_at = Utc::now();
    }
    
    /// Calculate the next payment date based on the billing cycle
    pub fn calculate_next_payment_date(&self) -> DateTime<Utc> {
        match self.billing_cycle {
            BillingCycle::Monthly => self.next_payment_date + chrono::Duration::days(30),
            BillingCycle::Quarterly => self.next_payment_date + chrono::Duration::days(90),
            BillingCycle::Annual => self.next_payment_date + chrono::Duration::days(365),
        }
    }
}