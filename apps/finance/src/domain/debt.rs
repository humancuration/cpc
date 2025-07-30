use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::domain::primitives::Money;

/// Payment schedule for debt
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PaymentSchedule {
    Monthly,
    BiWeekly,
    Weekly,
}

/// Debt entity representing a financial obligation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Debt {
    pub id: Uuid,
    pub creditor: String,
    pub balance: Money,
    pub interest_rate: f64, // APR
    pub minimum_payment: Money,
    pub payment_schedule: PaymentSchedule,
    pub snowball_priority: u32, // For debt snowball method
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Debt {
    pub fn new(
        creditor: String,
        balance: Money,
        interest_rate: f64,
        minimum_payment: Money,
        payment_schedule: PaymentSchedule,
        snowball_priority: u32,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            creditor,
            balance,
            interest_rate,
            minimum_payment,
            payment_schedule,
            snowball_priority,
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn update_balance(&mut self, new_balance: Money) {
        self.balance = new_balance;
        self.updated_at = Utc::now();
    }
    
    pub fn update_interest_rate(&mut self, new_rate: f64) {
        self.interest_rate = new_rate;
        self.updated_at = Utc::now();
    }
    
    pub fn update_minimum_payment(&mut self, new_payment: Money) {
        self.minimum_payment = new_payment;
        self.updated_at = Utc::now();
    }
    
    pub fn update_payment_schedule(&mut self, new_schedule: PaymentSchedule) {
        self.payment_schedule = new_schedule;
        self.updated_at = Utc::now();
    }
    
    pub fn update_snowball_priority(&mut self, new_priority: u32) {
        self.snowball_priority = new_priority;
        self.updated_at = Utc::now();
    }
    
    pub fn update_creditor(&mut self, new_creditor: String) {
        self.creditor = new_creditor;
        self.updated_at = Utc::now();
    }
    
    /// Calculate monthly interest
    pub fn monthly_interest(&self) -> Money {
        let monthly_rate = self.interest_rate / 12.0 / 100.0;
        Money::new(
            self.balance.amount * monthly_rate,
            &self.balance.currency,
        )
    }
    
    /// Calculate the number of payments needed to pay off the debt
    pub fn payments_to_payoff(&self, monthly_payment: f64) -> u32 {
        if monthly_payment <= self.monthly_interest().amount {
            // If payment is less than interest, debt will never be paid off
            return u32::MAX;
        }
        
        let balance = self.balance.amount;
        let monthly_rate = self.interest_rate / 12.0 / 100.0;
        
        // Formula: n = -log(1 - (r * B) / P) / log(1 + r)
        // where r = monthly interest rate, B = balance, P = monthly payment
        let numerator = 1.0 - (monthly_rate * balance) / monthly_payment;
        let denominator = 1.0 + monthly_rate;
        
        if numerator <= 0.0 {
            return u32::MAX;
        }
        
        (-numerator.ln() / denominator.ln()).ceil() as u32
    }
    
    /// Calculate total interest paid over the life of the loan
    pub fn total_interest_paid(&self, monthly_payment: f64) -> Money {
        let payments = self.payments_to_payoff(monthly_payment) as f64;
        let total_paid = monthly_payment * payments;
        let interest = total_paid - self.balance.amount;
        
        Money::new(
            interest.max(0.0),
            &self.balance.currency,
        )
    }
}