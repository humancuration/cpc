use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::api::{
    client::perform_graphql_query,
    graphql::expenses::{GetExpense, get_expense},
};
use crate::expenses::ExpenseRoute;

type Expense = get_expense::GetExpenseExpense;

#[derive(Properties, PartialEq, Clone)]
pub struct ExpenseDetailProps {
    pub id: String,
}

#[function_component(ExpenseDetail)]
pub fn expense_detail(props: &ExpenseDetailProps) -> Html {
    let expense = use_state(|| None::<Expense>);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    {
        let expense = expense.clone();
        let loading = loading.clone();
        let error = error.clone();
        let id = props.id.clone();

        use_effect_with(id.clone(), move |_| {
            spawn_local(async move {
                let vars = get_expense::Variables { id };
                match perform_graphql_query::<GetExpense>(vars).await {
                    Ok(data) => {
                        expense.set(data.expense);
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

    html! {
        <div class="container mx-auto px-4 py-8 max-w-2xl">
            <Link<ExpenseRoute> to={ExpenseRoute::List} classes="text-blue-500 hover:text-blue-800 mb-4 inline-block">
                { "← Back to Expenses" }
            </Link<ExpenseRoute>>

            if *loading {
                <p>{ "Loading..." }</p>
            } else if let Some(err) = &*error {
                <p class="text-red-500">{ format!("Error: {}", err) }</p>
            } else if let Some(exp) = &*expense {
                <div class="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4">
                    <div class="flex justify-between items-center mb-4">
                        <h1 class="text-3xl font-bold">{ &exp.description }</h1>
                        <Link<ExpenseRoute> to={ExpenseRoute::Edit { id: exp.id.clone() }} classes="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
                            { "Edit" }
                        </Link<ExpenseRoute>>
                    </div>
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                        <div><p class="font-bold">{"Amount:"}</p><p>{ format!("{} {}", &exp.amount, &exp.currency) }</p></div>
                        <div><p class="font-bold">{"Category:"}</p><p>{ &exp.category }</p></div>
                        <div><p class="font-bold">{"Transaction Date:"}</p><p>{ &exp.transaction_date }</p></div>
                        <div><p class="font-bold">{"Status:"}</p><p>{ &exp.status }</p></div>
                    </div>
                    <div class="mt-6">
                        <h2 class="text-xl font-bold mb-2">{"Receipts"}</h2>
                        if exp.receipts.is_empty() {
                            <p>{"No receipts attached."}</p>
                        } else {
                            <ul>
                                { for exp.receipts.iter().map(|r| html!{
                                    <li key={r.id.clone()}>
                                        <a href={r.url.clone()} target="_blank" class="text-blue-500 hover:underline">{ &r.file_name }</a>
                                    </li>
                                })}
                            </ul>
                        }
                    </div>
                </div>
            } else {
                 <p>{ "Expense not found." }</p>
            }
        </div>
    }
}