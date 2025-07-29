//! Finance events for the event bus

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use cpc_core::event_bus::event::DomainEvent;
use crate::domain::{Budget, Expense, Subscription, SavingsGoal, Investment, Debt};

/// Finance events that can be published to the event bus
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum FinanceEventType {
    BudgetCreated,
    BudgetUpdated,
    BudgetDeleted,
    ExpenseAdded,
    ExpenseUpdated,
    ExpenseDeleted,
    SubscriptionCreated,
    SubscriptionUpdated,
    SubscriptionDeleted,
    SavingsGoalCreated,
    SavingsGoalUpdated,
    SavingsGoalDeleted,
    InvestmentCreated,
    InvestmentUpdated,
    InvestmentDeleted,
    DebtCreated,
    DebtUpdated,
    DebtDeleted,
    // New events for dashboard integration
    DashboardDataRequested,
    DashboardDataUpdated,
    DashboardCellUpdated,
}

/// Finance events that can be published to the event bus
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum FinanceEvent {
    BudgetCreated(Budget),
    BudgetUpdated(Budget),
    BudgetDeleted(Uuid),
    ExpenseAdded(Expense),
    ExpenseUpdated(Expense),
    ExpenseDeleted(Uuid),
    SubscriptionCreated(Subscription),
    SubscriptionUpdated(Subscription),
    SubscriptionDeleted(Uuid),
    SavingsGoalCreated(SavingsGoal),
    SavingsGoalUpdated(SavingsGoal),
    SavingsGoalDeleted(Uuid),
    InvestmentCreated(Investment),
    InvestmentUpdated(Investment),
    InvestmentDeleted(Uuid),
    DebtCreated(Debt),
    DebtUpdated(Debt),
    DebtDeleted(Uuid),
    // New events for dashboard integration
    DashboardDataRequested { 
        user_id: Uuid,
        request_id: Uuid,
        source_sheet: Uuid, // SheetId
    },
    DashboardDataUpdated {
        request_id: Uuid,
        data: serde_json::Value, // DashboardData
    },
    // Reverse flow
    DashboardCellUpdated {
        sheet_id: Uuid, // SheetId
        cell_address: String, // CellAddress as string
        new_value: serde_json::Value, // CellValue
    },
}

impl FinanceEvent {
    pub fn event_type(&self) -> String {
        match self {
            FinanceEvent::BudgetCreated(_) => "finance.budget.created".to_string(),
            FinanceEvent::BudgetUpdated(_) => "finance.budget.updated".to_string(),
            FinanceEvent::BudgetDeleted(_) => "finance.budget.deleted".to_string(),
            FinanceEvent::ExpenseAdded(_) => "finance.expense.added".to_string(),
            FinanceEvent::ExpenseUpdated(_) => "finance.expense.updated".to_string(),
            FinanceEvent::ExpenseDeleted(_) => "finance.expense.deleted".to_string(),
            FinanceEvent::SubscriptionCreated(_) => "finance.subscription.created".to_string(),
            FinanceEvent::SubscriptionUpdated(_) => "finance.subscription.updated".to_string(),
            FinanceEvent::SubscriptionDeleted(_) => "finance.subscription.deleted".to_string(),
            FinanceEvent::SavingsGoalCreated(_) => "finance.savings_goal.created".to_string(),
            FinanceEvent::SavingsGoalUpdated(_) => "finance.savings_goal.updated".to_string(),
            FinanceEvent::SavingsGoalDeleted(_) => "finance.savings_goal.deleted".to_string(),
            FinanceEvent::InvestmentCreated(_) => "finance.investment.created".to_string(),
            FinanceEvent::InvestmentUpdated(_) => "finance.investment.updated".to_string(),
            FinanceEvent::InvestmentDeleted(_) => "finance.investment.deleted".to_string(),
            FinanceEvent::DebtCreated(_) => "finance.debt.created".to_string(),
            FinanceEvent::DebtUpdated(_) => "finance.debt.updated".to_string(),
            FinanceEvent::DebtDeleted(_) => "finance.debt.deleted".to_string(),
            FinanceEvent::DashboardDataRequested { .. } => "finance.dashboard.data.requested".to_string(),
            FinanceEvent::DashboardDataUpdated { .. } => "finance.dashboard.data.updated".to_string(),
            FinanceEvent::DashboardCellUpdated { .. } => "finance.dashboard.cell.updated".to_string(),
        }
    }
    
    pub fn serialize(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        Ok(serde_json::to_vec(self)?)
    }
    
    pub fn deserialize(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(serde_json::from_slice(data)?)
    }
}

/// Publisher for finance events
pub struct FinanceEventPublisher;

impl FinanceEventPublisher {
    pub fn new() -> Self {
        Self
    }
    
    pub fn publish(&self, event: &FinanceEvent) {
        // In a real implementation, this would publish to the event bus
        // For now, we'll just log the event
        tracing::info!("Publishing finance event: {}", event.event_type());
    }
}