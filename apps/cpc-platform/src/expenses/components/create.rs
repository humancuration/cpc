use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::api::{
    client::perform_graphql_query,
    graphql::expenses::{CreateExpense, create_expense},
};
use crate::expenses::ExpenseRoute;
use super::expense_form::{ExpenseForm, ExpenseState};

#[function_component(CreateExpense)]
pub fn create_expense() -> Html {
    let loading = use_state(|| false);
    let error = use_state(|| None::<String>);
    let navigator = use_navigator().unwrap();

    let on_submit = {
        let loading = loading.clone();
        let error = error.clone();
        let navigator = navigator.clone();

        Callback::from(move |state: ExpenseState| {
            loading.set(true);
            error.set(None);

            let final_category = if state.category == "Other" {
                state.other_category.clone()
            } else {
                state.category.clone()
            };

            // Basic Validation
            if state.description.is_empty() || state.amount.is_empty() || final_category.is_empty() || state.transaction_date.is_empty() {
                error.set(Some("Please fill out all required fields.".to_string()));
                loading.set(false);
                return;
            }

            let amount = match state.amount.parse::<f64>() {
                Ok(a) if a > 0.0 => a,
                _ => {
                    error.set(Some("Amount must be a positive number.".to_string()));
                    loading.set(false);
                    return;
                }
            };
            
            let currency = if state.currency.is_empty() { "USD".to_string() } else { state.currency.clone() };

            spawn_local(async move {
                let vars = create_expense::Variables {
                    input: create_expense::CreateExpenseInput {
                        project_id: None, // Simplified for now
                        amount,
                        currency,
                        description: state.description,
                        category: final_category,
                        transaction_date: state.transaction_date, // Assumes YYYY-MM-DD
                    },
                };

                match perform_graphql_query::<CreateExpense>(vars).await {
                    Ok(data) => {
                        if let Some(expense) = data.create_expense {
                             navigator.push(&ExpenseRoute::Detail { id: expense.id });
                        } else {
                            error.set(Some("Failed to create expense, no ID returned.".to_string()));
                        }
                    }
                    Err(e) => {
                        error.set(Some(e.to_string()));
                    }
                }
                loading.set(false);
            });
        })
    };
    
    html! {
        <div class="container mx-auto px-4 py-8 max-w-2xl">
            <h1 class="text-3xl font-bold mb-6">{ "Create New Expense" }</h1>
            <ExpenseForm
                initial_state={ExpenseState::default()}
                on_submit={on_submit}
                loading={*loading}
                error={(*error).clone()}
                submit_label={"Create Expense".to_string()}
            />
            <div class="text-center mt-4">
                <Link<ExpenseRoute> to={ExpenseRoute::List} classes="inline-block align-baseline font-bold text-sm text-blue-500 hover:text-blue-800">
                    { "Cancel" }
                </Link<ExpenseRoute>>
            </div>
        </div>