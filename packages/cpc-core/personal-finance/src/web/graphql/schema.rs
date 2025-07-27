//! GraphQL schema definitions for the Personal Finance API

use async_graphql::*;
use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::NaiveDate;

use crate::{
    application::finance_service::FinanceService,
    web::dto::{
        FinancialOverviewDto, CreateExpenseRequest, CreateBudgetRequest, 
        CreateSavingsGoalRequest, ReceiptProcessingRequest, ReceiptProcessingResponse,
        FinancialInsightsDto
    },
};

/// Root GraphQL query type
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Get financial overview for a user
    async fn financial_overview(
        &self,
        ctx: &Context<'_>,
        user_id: Uuid,
    ) -> Result<FinancialOverviewDto> {
        let service = ctx.data_unchecked::<FinanceService>();
        service.get_financial_overview(user_id)
            .await
            .map_err(|e| Error::new(e.to_string()))
    }

    /// Get financial insights for a user
    async fn financial_insights(
        &self,
        ctx: &Context<'_>,
        user_id: Uuid,
    ) -> Result<FinancialInsightsDto> {
        let service = ctx.data_unchecked::<FinanceService>();
        service.get_financial_insights(user_id)
            .await
            .map_err(|e| Error::new(e.to_string()))
    }

    /// Get monthly spending trend
    async fn monthly_trend(
        &self,
        ctx: &Context<'_>,
        user_id: Uuid,
        months: Option<u32>,
    ) -> Result<Vec<MonthlyTrend>> {
        let service = ctx.data_unchecked::<FinanceService>();
        let months = months.unwrap_or(6);
        
        service.get_monthly_trend(user_id, "spending", months)
            .await
            .map_err(|e| Error::new(e.to_string()))
            .map(|data| {
                data.into_iter()
                    .map(|(date, amount)| MonthlyTrend { date, amount })
                    .collect()
            })
    }
}

/// Monthly trend data point
#[derive(SimpleObject)]
pub struct MonthlyTrend {
    pub date: NaiveDate,
    pub amount: Decimal,
}

/// Root GraphQL mutation type
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Create a new expense
    async fn create_expense(
        &self,
        ctx: &Context<'_>,
        input: CreateExpenseInput,
    ) -> Result<Expense> {
        let service = ctx.data_unchecked::<FinanceService>();
        let request = CreateExpenseRequest {
            user_id: input.user_id,
            amount: input.amount,
            category: input.category,
            description: input.description,
            date: input.date,
        };
        
        service.create_expense(request)
            .await
            .map_err(|e| Error::new(e.to_string()))
            .map(|expense| Expense::from(expense))
    }

    /// Create a new budget
    async fn create_budget(
        &self,
        ctx: &Context<'_>,
        input: CreateBudgetInput,
    ) -> Result<Budget> {
        let service = ctx.data_unchecked::<FinanceService>();
        let request = CreateBudgetRequest {
            user_id: input.user_id,
            month: input.month,
            total_income: input.total_income,
            categories: input.categories.into_iter()
                .map(|c| crate::web::dto::BudgetCategoryRequest {
                    name: c.name,
                    allocated_amount: c.allocated_amount,
                })
                .collect(),
        };
        
        service.create_budget(request)
            .await
            .map_err(|e| Error::new(e.to_string()))
            .map(|budget| Budget::from(budget))
    }

    /// Create a new savings goal
    async fn create_savings_goal(
        &self,
        ctx: &Context<'_>,
        input: CreateSavingsGoalInput,
    ) -> Result<SavingsGoal> {
        let service = ctx.data_unchecked::<FinanceService>();
        let request = CreateSavingsGoalRequest {
            user_id: input.user_id,
            name: input.name,
            target_amount: input.target_amount,
            deadline: input.deadline,
            monthly_contribution: input.monthly_contribution,
        };
        
        service.create_savings_goal(request)
            .await
            .map_err(|e| Error::new(e.to_string()))
            .map(|goal| SavingsGoal::from(goal))
    }

    /// Process receipt via image upload
    async fn process_receipt_image(
        &self,
        ctx: &Context<'_>,
        input: ProcessReceiptImageInput,
    ) -> Result<Receipt> {
        let service = ctx.data_unchecked::<FinanceService>();
        let request = ReceiptProcessingRequest {
            user_id: input.user_id,
            image_data: Some(input.image_data),
            image_url: None,
            mime_type: Some(input.mime_type),
        };
        
        service.process_receipt(request)
            .await
            .map_err(|e| Error::new(e.to_string()))
            .map(|receipt| Receipt::from(receipt))
    }

    /// Process receipt via URL
    async fn process_receipt_url(
        &self,
        ctx: &Context<'_>,
        input: ProcessReceiptUrlInput,
    ) -> Result<Receipt> {
        let service = ctx.data_unchecked::<FinanceService>();
        let request = ReceiptProcessingRequest {
            user_id: input.user_id,
            image_data: None,
            image_url: Some(input.url),
            mime_type: None,
        };
        
        service.process_receipt(request)
            .await
            .map_err(|e| Error::new(e.to_string()))
            .map(|receipt| Receipt::from(receipt))
    }

    /// Update monthly savings contribution
    async fn update_savings_contribution(
        &self,
        ctx: &Context<'_>,
        savings_goal_id: Uuid,
        new_contribution: Decimal,
    ) -> Result<SavingsGoal> {
        let service = ctx.data_unchecked::<FinanceService>();
        
        service.update_savings_contribution(savings_goal_id, new_contribution)
            .await
            .map_err(|e| Error::new(e.to_string()))
            .map(|goal| SavingsGoal::from(goal))
    }
}

/// Input types for mutations

#[derive(InputObject)]
pub struct CreateExpenseInput {
    pub user_id: Uuid,
    pub amount: Decimal,
    pub category: String,
    pub description: String,
    pub date: NaiveDate,
}

#[derive(InputObject)]
pub struct CreateBudgetInput {
    pub user_id: Uuid,
    pub month: NaiveDate,
    pub total_income: Decimal,
    pub categories: Vec<BudgetCategoryInput>,
}

#[derive(InputObject)]
pub struct BudgetCategoryInput {
    pub name: String,
    pub allocated_amount: Decimal,
}

#[derive(InputObject)]
pub struct CreateSavingsGoalInput {
    pub user_id: Uuid,
    pub name: String,
    pub target_amount: Decimal,
    pub deadline: NaiveDate,
    pub monthly_contribution: Decimal,
}

#[derive(InputObject)]
pub struct ProcessReceiptImageInput {
    pub user_id: Uuid,
    pub image_data: Vec<u8>,
    pub mime_type: String,
}

#[derive(InputObject)]
pub struct ProcessReceiptUrlInput {
    pub user_id: Uuid,
    pub url: String,
}

/// GraphQL type definitions

#[derive(SimpleObject)]
pub struct Expense {
    pub id: Uuid,
    pub user_id: Uuid,
    pub amount: Decimal,
    pub category: String,
    pub description: String,
    pub date: NaiveDate,
}

impl From<crate::web::dto::ExpenseDto> for Expense {
    fn from(dto: crate::web::dto::ExpenseDto) -> Self {
        Self {
            id: dto.id,
            user_id: dto.user_id,
            amount: dto.amount,
            category: dto.category,
            description: dto.description,
            date: dto.date,
        }
    }
}

#[derive(SimpleObject)]
pub struct Budget {
    pub id: Uuid,
    pub user_id: Uuid,
    pub month: NaiveDate,
    pub total_income: Decimal,
    pub total_allocated: Decimal,
    pub categories: Vec<BudgetCategory>,
}

#[derive(SimpleObject)]
pub struct BudgetCategory {
    pub name: String,
    pub allocated: Decimal,
    pub spent: Decimal,
    pub remaining: Decimal,
}

impl From<crate::domain::models::Budget> for Budget {
    fn from(budget: crate::domain::models::Budget) -> Self {
        Self {
            id: budget.id,
            user_id: budget.user_id,
            month: budget.month,
            total_income: budget.total_income,
            total_allocated: budget.total_allocated,
            categories: budget.categories.into_iter()
                .map(|c| BudgetCategory {
                    name: c.name,
                    allocated: c.allocated_amount,
                    spent: c.spent_amount,
                    remaining: c.remaining_amount(),
                })
                .collect(),
        }
    }
}

#[derive(SimpleObject)]
pub struct SavingsGoal {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub target_amount: Decimal,
    pub current_amount: Decimal,
    pub deadline: NaiveDate,
    pub monthly_contribution: Decimal,
    pub progress_percentage: Decimal,
}

impl From<crate::web::dto::SavingsGoalDto> for SavingsGoal {
    fn from(dto: crate::web::dto::SavingsGoalDto) -> Self {
        let progress = if dto.target_amount > Decimal::ZERO {
            (dto.current_amount / dto.target_amount) * Decimal::from(100)
        } else {
            Decimal::ZERO
        };
        
        Self {
            id: dto.id,
            user_id: dto.user_id,
            name: dto.name,
            target_amount: dto.target_amount,
            current_amount: dto.current_amount,
            deadline: dto.deadline,
            monthly_contribution: dto.monthly_contribution,
            progress_percentage: progress,
        }
    }
}

#[derive(SimpleObject)]
pub struct Receipt {
    pub merchant_name: String,
    pub total_amount: Decimal,
    pub date: NaiveDate,
    pub category: Option<String>,
    pub items: Vec<ReceiptItem>,
}

#[derive(SimpleObject)]
pub struct ReceiptItem {
    pub name: String,
    pub quantity: Decimal,
    pub unit_price: Decimal,
    pub total_price: Decimal,
}

impl From<crate::infrastructure::services::ocr_service::ReceiptData> for Receipt {
    fn from(receipt: crate::infrastructure::services::ocr_service::ReceiptData) -> Self {
        Self {
            merchant_name: receipt.merchant_name,
            total_amount: receipt.total_amount,
            date: receipt.date,
            category: receipt.category,
            items: receipt.items.into_iter()
                .map(|item| ReceiptItem {
                    name: item.name,
                    quantity: item.quantity,
                    unit_price: item.unit_price,
                    total_price: item.total_price,
                })
                .collect(),
        }
    }
}

/// Root GraphQL subscription type
pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    /// Subscribe to real-time expense updates for a user
    async fn expenses_updated(
        &self,
        user_id: Uuid,
    ) -> impl futures::Stream<Item = Expense> {
        // This would typically use a broadcast channel or similar
        // For now, returning an empty stream
        futures::stream::empty()
    }

    /// Subscribe to savings goal progress updates
    async fn savings_goal_updated(
        &self,
        user_id: Uuid,
    ) -> impl futures::Stream<Item = SavingsGoal> {
        // This would typically use a broadcast channel or similar
        // For now, returning an empty stream
        futures::stream::empty()
    }
}