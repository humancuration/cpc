use crate::domain::{Subscription, BillingCycle, PaymentMethod, FinancialCategory, Money};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Service for managing subscriptions
pub struct SubscriptionService {
    subscriptions: HashMap<Uuid, Subscription>,
}

impl SubscriptionService {
    pub fn new() -> Self {
        Self {
            subscriptions: HashMap::new(),
        }
    }
    
    /// Create a new subscription
    pub fn create_subscription(
        &mut self,
        name: String,
        amount: Money,
        billing_cycle: BillingCycle,
        next_payment_date: DateTime<Utc>,
        payment_method: PaymentMethod,
        category: FinancialCategory,
    ) -> Result<Subscription, String> {
        let subscription = Subscription::new(
            name,
            amount,
            billing_cycle,
            next_payment_date,
            payment_method,
            category,
        );
        self.subscriptions.insert(subscription.id, subscription.clone());
        Ok(subscription)
    }
    
    /// Get a subscription by ID
    pub fn get_subscription(&self, id: &Uuid) -> Option<&Subscription> {
        self.subscriptions.get(id)
    }
    
    /// Get all subscriptions
    pub fn get_all_subscriptions(&self) -> Vec<&Subscription> {
        self.subscriptions.values().collect()
    }
    
    /// Update subscription amount
    pub fn update_subscription_amount(
        &mut self,
        id: &Uuid,
        new_amount: Money,
    ) -> Result<(), String> {
        match self.subscriptions.get_mut(id) {
            Some(subscription) => {
                subscription.update_amount(new_amount);
                Ok(())
            }
            None => Err("Subscription not found".to_string()),
        }
    }
    
    /// Update next payment date
    pub fn update_next_payment_date(
        &mut self,
        id: &Uuid,
        new_date: DateTime<Utc>,
    ) -> Result<(), String> {
        match self.subscriptions.get_mut(id) {
            Some(subscription) => {
                subscription.update_next_payment_date(new_date);
                Ok(())
            }
            None => Err("Subscription not found".to_string()),
        }
    }
    
    /// Toggle auto-renew
    pub fn toggle_auto_renew(&mut self, id: &Uuid) -> Result<(), String> {
        match self.subscriptions.get_mut(id) {
            Some(subscription) => {
                subscription.toggle_auto_renew();
                Ok(())
            }
            None => Err("Subscription not found".to_string()),
        }
    }
    
    /// Set notification days
    pub fn set_notification_days(
        &mut self,
        id: &Uuid,
        days: u32,
    ) -> Result<(), String> {
        match self.subscriptions.get_mut(id) {
            Some(subscription) => {
                subscription.set_notification_days(days);
                Ok(())
            }
            None => Err("Subscription not found".to_string()),
        }
    }
    
    /// Delete a subscription
    pub fn delete_subscription(&mut self, id: &Uuid) -> Result<(), String> {
        match self.subscriptions.remove(id) {
            Some(_) => Ok(()),
            None => Err("Subscription not found".to_string()),
        }
    }
    
    /// Get subscriptions by category
    pub fn get_subscriptions_by_category(&self, category: &FinancialCategory) -> Vec<&Subscription> {
        self.subscriptions
            .values()
            .filter(|subscription| &subscription.category == category)
            .collect()
    }
    
    /// Get subscriptions expiring within a number of days
    pub fn get_subscriptions_expiring_soon(&self, days: i64) -> Vec<&Subscription> {
        let cutoff_date = Utc::now() + chrono::Duration::days(days);
        self.subscriptions
            .values()
            .filter(|subscription| subscription.next_payment_date <= cutoff_date)
            .collect()
    }
    
    /// Get total monthly subscription cost
    pub fn get_total_monthly_cost(&self) -> Money {
        // For simplicity, we'll assume all subscriptions are in the same currency
        // In a real implementation, we'd need to handle currency conversion
        let mut total_amount = 0.0;
        let mut currency = "USD".to_string();
        
        for subscription in self.subscriptions.values() {
            let monthly_amount = match subscription.billing_cycle {
                BillingCycle::Monthly => subscription.amount.amount,
                BillingCycle::Quarterly => subscription.amount.amount / 3.0,
                BillingCycle::Annual => subscription.amount.amount / 12.0,
            };
            total_amount += monthly_amount;
            currency = subscription.amount.currency.clone();
        }
        
        Money::new(total_amount, &currency)
    }
}