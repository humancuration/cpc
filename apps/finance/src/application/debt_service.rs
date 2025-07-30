use crate::domain::{Debt, PaymentSchedule, Money};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Service for managing debts
pub struct DebtService {
    debts: HashMap<Uuid, Debt>,
}

impl DebtService {
    pub fn new() -> Self {
        Self {
            debts: HashMap::new(),
        }
    }
    
    /// Create a new debt
    pub fn create_debt(
        &mut self,
        creditor: String,
        balance: Money,
        interest_rate: f64,
        minimum_payment: Money,
        payment_schedule: PaymentSchedule,
        snowball_priority: u32,
    ) -> Result<Debt, String> {
        let debt = Debt::new(
            creditor,
            balance,
            interest_rate,
            minimum_payment,
            payment_schedule,
            snowball_priority,
        );
        self.debts.insert(debt.id, debt.clone());
        Ok(debt)
    }
    
    /// Get a debt by ID
    pub fn get_debt(&self, id: &Uuid) -> Option<&Debt> {
        self.debts.get(id)
    }
    
    /// Get all debts
    pub fn get_all_debts(&self) -> Vec<&Debt> {
        self.debts.values().collect()
    }
    
    /// Update balance for a debt
    pub fn update_balance(
        &mut self,
        id: &Uuid,
        new_balance: Money,
    ) -> Result<(), String> {
        match self.debts.get_mut(id) {
            Some(debt) => {
                debt.update_balance(new_balance);
                Ok(())
            }
            None => Err("Debt not found".to_string()),
        }
    }
    
    /// Update interest rate for a debt
    pub fn update_interest_rate(
        &mut self,
        id: &Uuid,
        new_rate: f64,
    ) -> Result<(), String> {
        match self.debts.get_mut(id) {
            Some(debt) => {
                debt.update_interest_rate(new_rate);
                Ok(())
            }
            None => Err("Debt not found".to_string()),
        }
    }
    
    /// Update minimum payment for a debt
    pub fn update_minimum_payment(
        &mut self,
        id: &Uuid,
        new_payment: Money,
    ) -> Result<(), String> {
        match self.debts.get_mut(id) {
            Some(debt) => {
                debt.update_minimum_payment(new_payment);
                Ok(())
            }
            None => Err("Debt not found".to_string()),
        }
    }
    
    /// Update payment schedule for a debt
    pub fn update_payment_schedule(
        &mut self,
        id: &Uuid,
        new_schedule: PaymentSchedule,
    ) -> Result<(), String> {
        match self.debts.get_mut(id) {
            Some(debt) => {
                debt.update_payment_schedule(new_schedule);
                Ok(())
            }
            None => Err("Debt not found".to_string()),
        }
    }
    
    /// Update snowball priority for a debt
    pub fn update_snowball_priority(
        &mut self,
        id: &Uuid,
        new_priority: u32,
    ) -> Result<(), String> {
        match self.debts.get_mut(id) {
            Some(debt) => {
                debt.update_snowball_priority(new_priority);
                Ok(())
            }
            None => Err("Debt not found".to_string()),
        }
    }
    
    /// Update creditor for a debt
    pub fn update_creditor(
        &mut self,
        id: &Uuid,
        new_creditor: String,
    ) -> Result<(), String> {
        match self.debts.get_mut(id) {
            Some(debt) => {
                debt.update_creditor(new_creditor);
                Ok(())
            }
            None => Err("Debt not found".to_string()),
        }
    }
    
    /// Delete a debt
    pub fn delete_debt(&mut self, id: &Uuid) -> Result<(), String> {
        match self.debts.remove(id) {
            Some(_) => Ok(()),
            None => Err("Debt not found".to_string()),
        }
    }
    
    /// Get debts ordered by snowball priority
    pub fn get_debts_by_snowball_priority(&self) -> Vec<&Debt> {
        let mut debts: Vec<&Debt> = self.debts.values().collect();
        debts.sort_by(|a, b| a.snowball_priority.cmp(&b.snowball_priority));
        debts
    }
    
    /// Get total debt balance
    pub fn get_total_debt_balance(&self) -> Money {
        // For simplicity, we'll assume all debts are in the same currency
        // In a real implementation, we'd need to handle currency conversion
        let mut total_balance = 0.0;
        let mut currency = "USD".to_string();
        
        for debt in self.debts.values() {
            total_balance += debt.balance.amount;
            currency = debt.balance.currency.clone();
        }
        
        Money::new(total_balance, &currency)
    }
    
    /// Get total minimum monthly payments
    pub fn get_total_minimum_monthly_payments(&self) -> Money {
        // For simplicity, we'll assume all debts are in the same currency
        // In a real implementation, we'd need to handle currency conversion
        let mut total_payments = 0.0;
        let mut currency = "USD".to_string();
        
        for debt in self.debts.values() {
            // Convert all payment schedules to monthly equivalent
            let monthly_payment = match debt.payment_schedule {
                PaymentSchedule::Monthly => debt.minimum_payment.amount,
                PaymentSchedule::BiWeekly => debt.minimum_payment.amount * 2.0,
                PaymentSchedule::Weekly => debt.minimum_payment.amount * 4.0,
            };
            total_payments += monthly_payment;
            currency = debt.minimum_payment.currency.clone();
        }
        
        Money::new(total_payments, &currency)
    }
    
    /// Get total monthly interest
    pub fn get_total_monthly_interest(&self) -> Money {
        // For simplicity, we'll assume all debts are in the same currency
        // In a real implementation, we'd need to handle currency conversion
        let mut total_interest = 0.0;
        let mut currency = "USD".to_string();
        
        for debt in self.debts.values() {
            total_interest += debt.monthly_interest().amount;
            currency = debt.balance.currency.clone();
        }
        
        Money::new(total_interest, &currency)
    }
}