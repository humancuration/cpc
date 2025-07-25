use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::api::{
    client::perform_graphql_query,
    graphql::expenses::{GetExpense, get_expense, UpdateExpenseMutation, update_expense_mutation},
};
use crate::expenses::ExpenseRoute;
use super::expense_form::{ExpenseForm, ExpenseState};

#[derive(Properties, PartialEq, Clone)]
pub struct EditExpenseProps {
    pub id: String,
}

#[function_component(EditExpense)]
pub fn edit_expense(props: &EditExpenseProps) -> Html {
    let initial_state = use_state(ExpenseState::default);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let navigator = use_navigator().unwrap();

    // Fetch existing expense data
    {
        let initial_state = initial_state.clone();
        let loading = loading.clone();
        let error = error.clone();
        let id = props.id.clone();

        use_effect_with(id.clone(), move |_| {
            spawn_local(async move {
                let vars = get_expense::Variables { id };
                match perform_graphql_query::<GetExpense>(vars).await {
                    Ok(data) => {
                        if let Some(expense) = data.expense {
                            initial_state.set(ExpenseState {
                                amount: expense.amount.to_string(),
                                currency: expense.currency,
                                description: expense.description,
                                category: expense.category,
                                other_category: "".to_string(), // Assume not "Other" initially
                                transaction_date: expense.transaction_date,
                            });
                        } else {
                            error.set(Some("Expense not found.".to_string()));
                        }
                    }
                    Err(e) => {
                        error.set(Some(e.to_string()));
                    }
                }
                loading.set(false);
            });
            || ()
        });
    }

    let on_submit = {
        let loading = loading.clone();
        let error = error.clone();
        let navigator = navigator.clone();
        let id = props.id.clone();

        Callback::from(move |state: ExpenseState| {
            loading.set(true);
            error.set(None);

            let final_category = if state.category == "Other" {
                state.other_category.clone()
            } else {
                state.category.clone()
            };

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
                let vars = update_expense_mutation::Variables {
                    input: update_expense_mutation::UpdateExpenseInput {
                        id: id.clone(),
                        title: None, // Assuming title is not part of the form for now
                        amount: Some(amount),
                        currency: Some(currency),
                        category: Some(final_category),
                        transaction_date: Some(state.transaction_date),
                        description: Some(state.description),
                        status: None, // Status is not updated via this form
                        project_id: None,
                    },
                };

                match perform_graphql_query::<UpdateExpenseMutation>(vars).await {
                    Ok(data) => {
                        if data.update_expense.is_some() {
                            navigator.push(&ExpenseRoute::Detail { id });
                        } else {
                            error.set(Some("Failed to update expense.".to_string()));
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

    if *loading && (*initial_state).description.is_empty() {
        return html! { <p>{ "Loading..." }</p> };
    }
    
    html! {
        <div class="container mx-auto px-4 py-8 max-w-2xl">
            <h1 class="text-3xl font-bold mb-6">{ "Edit Expense" }</h1>
            <ExpenseForm 
                initial_state={(*initial_state).clone()} 
                on_submit={on_submit} 
                loading={*loading} 
                error={(*error).clone()}
                submit_label={"Save Changes".to_string()}
            />
            <div class="text-center mt-4">
                <Link<ExpenseRoute> to={ExpenseRoute::Detail { id: props.id.clone() }} classes="inline-block align-baseline font-bold text-sm text-blue-500 hover:text-blue-800">
                    { "Cancel" }
                </Link<ExpenseRoute>>
            </div>
        </div>
    }
}