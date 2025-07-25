use graphql_client::GraphQLQuery;
use serde::{Deserialize, Serialize};
use crate::types::expense::Expense;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/expenses.graphql",
    response_derives = "Debug, Clone, Serialize, Deserialize"
)]
pub struct CreateExpense;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpenseFilters {
    pub project_id: Option<String>,
    pub category: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub min_amount: Option<f64>,
    pub max_amount: Option<f64>,
}

pub struct ExpenseService {
    client: reqwest::Client,
    endpoint: String,
}

impl ExpenseService {
    pub fn new(endpoint: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            endpoint,
        }
    }

    pub async fn create_expense_mutation(&self, expense: Expense) -> Result<CreateExpense::CreateExpenseCreateExpense, Box<dyn std::error::Error>> {
        let input = CreateExpense::CreateExpenseInput {
            project_id: expense.project_id,
            category: expense.category,
            description: expense.description,
            amount: expense.amount,
            currency: expense.currency,
            transaction_date: expense.transaction_date.to_rfc3339(),
        };

        let variables = CreateExpense::Variables {
            input,
            // Note: user_id should be obtained from auth context in a real app
            user_id: uuid::Uuid::new_v4().to_string(),
        };

        let response = self.client
            .post(&self.endpoint)
            .json(&CreateExpense::build_query(variables))
            .send()
            .await?
            .json::<graphql_client::Response<CreateExpense::ResponseData>>()
            .await?;

        response.data
            .and_then(|d| d.create_expense)
            .ok_or_else(|| "Failed to create expense".into())
    }
}