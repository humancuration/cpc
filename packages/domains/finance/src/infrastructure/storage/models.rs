//! Database models for financial entities

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::domain::primitives::{FinancialCategory, Money, TimePeriod, BillingCycle, PaymentMethod, AssetClass, RiskLevel, PaymentSchedule, GoalVisualStyle};

/// Database model for Budget
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetModel {
    pub id: Uuid,
    pub name: String,
    pub category: FinancialCategory,
    pub amount: Money,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<crate::domain::budget::Budget> for BudgetModel {
    fn from(budget: crate::domain::budget::Budget) -> Self {
        Self {
            id: budget.id,
            name: budget.name,
            category: budget.category,
            amount: budget.amount,
            period_start: budget.period.start,
            period_end: budget.period.end,
            created_at: budget.created_at,
            updated_at: budget.updated_at,
        }
    }
}

impl From<BudgetModel> for crate::domain::budget::Budget {
    fn from(model: BudgetModel) -> Self {
        Self {
            id: model.id,
            name: model.name,
            category: model.category,
            amount: model.amount,
            period: TimePeriod::new(model.period_start, model.period_end),
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

/// Database model for Expense
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpenseModel {
    pub id: Uuid,
    pub amount: Money,
    pub category: FinancialCategory,
    pub date: DateTime<Utc>,
    pub description: String,
    pub receipt_id: Option<Uuid>,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<crate::domain::expense::Expense> for ExpenseModel {
    fn from(expense: crate::domain::expense::Expense) -> Self {
        Self {
            id: expense.id,
            amount: expense.amount,
            category: expense.category,
            date: expense.date,
            description: expense.description,
            receipt_id: expense.receipt_id,
            tags: expense.tags,
            created_at: expense.created_at,
            updated_at: expense.updated_at,
        }
    }
}

impl From<ExpenseModel> for crate::domain::expense::Expense {
    fn from(model: ExpenseModel) -> Self {
        Self {
            id: model.id,
            amount: model.amount,
            category: model.category,
            date: model.date,
            description: model.description,
            receipt_id: model.receipt_id,
            tags: model.tags,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

/// Database model for Subscription
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionModel {
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

impl From<crate::domain::subscription::Subscription> for SubscriptionModel {
    fn from(subscription: crate::domain::subscription::Subscription) -> Self {
        Self {
            id: subscription.id,
            name: subscription.name,
            amount: subscription.amount,
            billing_cycle: subscription.billing_cycle,
            next_payment_date: subscription.next_payment_date,
            payment_method: subscription.payment_method,
            category: subscription.category,
            auto_renew: subscription.auto_renew,
            notification_days: subscription.notification_days,
            created_at: subscription.created_at,
            updated_at: subscription.updated_at,
        }
    }
}

impl From<SubscriptionModel> for crate::domain::subscription::Subscription {
    fn from(model: SubscriptionModel) -> Self {
        Self {
            id: model.id,
            name: model.name,
            amount: model.amount,
            billing_cycle: model.billing_cycle,
            next_payment_date: model.next_payment_date,
            payment_method: model.payment_method,
            category: model.category,
            auto_renew: model.auto_renew,
            notification_days: model.notification_days,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

/// Database model for SavingsGoal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavingsGoalModel {
    pub id: Uuid,
    pub name: String,
    pub target_amount: Money,
    pub current_amount: Money,
    pub target_date: DateTime<Utc>,
    pub progress: f64,
    pub visual_style: GoalVisualStyle,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<crate::domain::savings_goal::SavingsGoal> for SavingsGoalModel {
    fn from(savings_goal: crate::domain::savings_goal::SavingsGoal) -> Self {
        Self {
            id: savings_goal.id,
            name: savings_goal.name,
            target_amount: savings_goal.target_amount,
            current_amount: savings_goal.current_amount,
            target_date: savings_goal.target_date,
            progress: savings_goal.progress,
            visual_style: savings_goal.visual_style,
            created_at: savings_goal.created_at,
            updated_at: savings_goal.updated_at,
        }
    }
}

impl From<SavingsGoalModel> for crate::domain::savings_goal::SavingsGoal {
    fn from(model: SavingsGoalModel) -> Self {
        Self {
            id: model.id,
            name: model.name,
            target_amount: model.target_amount,
            current_amount: model.current_amount,
            target_date: model.target_date,
            progress: model.progress,
            visual_style: model.visual_style,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

/// Database model for Investment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestmentModel {
    pub id: Uuid,
    pub symbol: String,
    pub name: String,
    pub quantity: f64,
    pub purchase_price: Money,
    pub current_value: Money,
    pub asset_class: AssetClass,
    pub risk_level: RiskLevel,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<crate::domain::investment::Investment> for InvestmentModel {
    fn from(investment: crate::domain::investment::Investment) -> Self {
        Self {
            id: investment.id,
            symbol: investment.symbol,
            name: investment.name,
            quantity: investment.quantity,
            purchase_price: investment.purchase_price,
            current_value: investment.current_value,
            asset_class: investment.asset_class,
            risk_level: investment.risk_level,
            created_at: investment.created_at,
            updated_at: investment.updated_at,
        }
    }
}

impl From<InvestmentModel> for crate::domain::investment::Investment {
    fn from(model: InvestmentModel) -> Self {
        Self {
            id: model.id,
            symbol: model.symbol,
            name: model.name,
            quantity: model.quantity,
            purchase_price: model.purchase_price,
            current_value: model.current_value,
            asset_class: model.asset_class,
            risk_level: model.risk_level,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

/// Database model for Debt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebtModel {
    pub id: Uuid,
    pub creditor: String,
    pub balance: Money,
    pub interest_rate: f64,
    pub minimum_payment: Money,
    pub payment_schedule: PaymentSchedule,
    pub snowball_priority: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<crate::domain::debt::Debt> for DebtModel {
    fn from(debt: crate::domain::debt::Debt) -> Self {
        Self {
            id: debt.id,
            creditor: debt.creditor,
            balance: debt.balance,
            interest_rate: debt.interest_rate,
            minimum_payment: debt.minimum_payment,
            payment_schedule: debt.payment_schedule,
            snowball_priority: debt.snowball_priority,
            created_at: debt.created_at,
            updated_at: debt.updated_at,
        }
    }
}

impl From<DebtModel> for crate::domain::debt::Debt {
    fn from(model: DebtModel) -> Self {
        Self {
            id: model.id,
            creditor: model.creditor,
            balance: model.balance,
            interest_rate: model.interest_rate,
            minimum_payment: model.minimum_payment,
            payment_schedule: model.payment_schedule,
            snowball_priority: model.snowball_priority,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}