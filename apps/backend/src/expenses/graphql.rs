use async_graphql::{Context, Object, Result, ID, InputObject};
use cpc_core::expenses::model::{Expense as CoreExpense, ExpenseStatus, Receipt as CoreReceipt, ExpenseCategory};
use cpc_core::expenses::service::{CreateExpenseInput as CoreCreateExpenseInput, ExpenseService};
use std::sync::Arc;
use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};

// Cloned a part of the core DTO here to use with #[InputObject]
#[derive(InputObject)]
pub struct CreateExpenseInput {
    pub project_id: Option<Uuid>,
    pub client_id: Option<Uuid>,
    pub amount: Decimal,
    pub currency: String,
    pub description: String,
    pub category: String, // Simplified to string for GraphQL
    pub transaction_date: DateTime<Utc>,
}


#[derive(Default)]
pub struct ExpensesQueryRoot;

#[Object]
impl ExpensesQueryRoot {
    async fn expense(&self, ctx: &Context<'_>, id: ID) -> Result<Option<Expense>> {
        let service = ctx.data_unchecked::<Arc<dyn ExpenseService>>();
        let expense_id = Uuid::parse_str(id.as_str())?;
        let core_expense = service.get_expense_by_id(expense_id).await?;
        Ok(core_expense.map(Expense::from))
    }

    async fn expenses(&self, ctx: &Context<'_>) -> Result<Vec<Expense>> {
        // Assuming we get user_id from auth context
        let user_id = Uuid::new_v4(); // placeholder
        let service = ctx.data_unchecked::<Arc<dyn ExpenseService>>();
        let core_expenses = service.get_expenses_for_user(user_id).await?;
        Ok(core_expenses.into_iter().map(Expense::from).collect())
    }
}

#[derive(Default)]
pub struct ExpensesMutationRoot;

#[Object]
impl ExpensesMutationRoot {
    async fn create_expense(&self, ctx: &Context<'_>, input: CreateExpenseInput) -> Result<Expense> {
        let service = ctx.data_unchecked::<Arc<dyn ExpenseService>>();
        let user_id = Uuid::new_v4(); // placeholder, should come from auth
        
        // Convert GraphQL input to core input
        let category = match input.category.as_str() {
            "Travel" => ExpenseCategory::Travel,
            "Meals" => ExpenseCategory::Meals,
            "Software" => ExpenseCategory::Software,
            "Hardware" => ExpenseCategory::Hardware,
            "OfficeSupplies" => ExpenseCategory::OfficeSupplies,
            other => ExpenseCategory::Other(other.to_string()),
        };

        let core_input = CoreCreateExpenseInput {
            project_id: input.project_id,
            client_id: input.client_id,
            amount: input.amount,
            currency: input.currency,
            description: input.description,
            category,
            transaction_date: input.transaction_date,
        };
        let core_expense = service.create_expense(user_id, core_input).await?;
        Ok(Expense::from(core_expense))
    }

    async fn update_expense_status(&self, ctx: &Context<'_>, id: ID, status: String) -> Result<Expense> {
        let service = ctx.data_unchecked::<Arc<dyn ExpenseService>>();
        let expense_id = Uuid::parse_str(id.as_str())?;
        let expense_status: ExpenseStatus = serde_json::from_str(&format!("\"{}\"", status))?;
        let core_expense = service.update_expense_status(expense_id, expense_status).await?;
        Ok(Expense::from(core_expense))
    }
}

// GraphQL-specific representation of an Expense
pub struct Expense {
    core: CoreExpense,
}

#[Object]
impl Expense {
    async fn id(&self) -> ID {
        self.core.id.into()
    }
    async fn amount(&self) -> f64 {
        self.core.amount.to_string().parse().unwrap_or(0.0)
    }
    async fn currency(&self) -> &str {
        &self.core.currency
    }
    async fn description(&self) -> &str {
        &self.core.description
    }
    async fn category(&self) -> String {
        format!("{:?}", self.core.category)
    }
    async fn status(&self) -> String {
        format!("{:?}", self.core.status)
    }
    async fn transaction_date(&self) -> String {
        self.core.transaction_date.to_rfc3339()
    }
    async fn receipts(&self) -> Vec<Receipt> {
        self.core.receipts.iter().map(|r| Receipt::from(r.clone())).collect()
    }
}

impl From<CoreExpense> for Expense {
    fn from(core: CoreExpense) -> Self {
        Self { core }
    }
}


// GraphQL-specific representation of a Receipt
#[derive(Clone)]
pub struct Receipt {
    core: CoreReceipt,
}

#[Object]
impl Receipt {
    async fn id(&self) -> ID {
        self.core.id.into()
    }
    async fn file_name(&self) -> &str {
        &self.core.file_name
    }
    async fn url(&self) -> String {
        // This would generate a signed URL or a direct link to the file.
        format!("/files/{}", self.core.file_path)
    }
}

impl From<CoreReceipt> for Receipt {
    fn from(core: CoreReceipt) -> Self {
        Self { core }
    }
}
use async_graphql::Subscription;
use tokio_stream::Stream;

#[derive(Default)]
pub struct ExpensesSubscriptionRoot;

#[Subscription]
impl ExpensesSubscriptionRoot {
    async fn expense_updated(&self, _id: ID) -> impl Stream<Item = Expense> {
        futures_util::stream::empty()
    }
}