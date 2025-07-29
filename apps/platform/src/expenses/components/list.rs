use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::api::{
    client::perform_graphql_query,
    graphql::expenses::{GetExpenses, get_expenses},
};
use crate::expenses::ExpenseRoute;

type Expense = get_expenses::GetExpensesExpenses;

#[function_component(ExpenseList)]
pub fn expense_list() -> Html {
    let expenses = use_state(|| vec![]);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    {
        let expenses = expenses.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                let vars = get_expenses::Variables {};
                match perform_graphql_query::<GetExpenses>(vars).await {
                    Ok(data) => {
                        expenses.set(data.expenses);
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

    let navigator = use_navigator().unwrap();
    let on_create = Callback::from(move |_| navigator.push(&ExpenseRoute::Create));

    html! {
        <div class="container mx-auto px-4 py-8">
            <div class="flex justify-between items-center mb-6">
                <h1 class="text-3xl font-bold">{ "Expenses" }</h1>
                <button onclick={on_create}
                    class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
                    { "New Expense" }
                </button>
            </div>

            if *loading {
                <p>{ "Loading..." }</p>
            } else if let Some(err) = &*error {
                <p class="text-red-500">{ format!("Error: {}", err) }</p>
            } else {
                <div class="shadow overflow-hidden border-b border-gray-200 sm:rounded-lg">
                    <table class="min-w-full divide-y divide-gray-200">
                        <thead class="bg-gray-50">
                            <tr>
                                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">{"Description"}</th>
                                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">{"Category"}</th>
                                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">{"Amount"}</th>
                                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">{"Date"}</th>
                                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">{"Status"}</th>
                                <th class="relative px-6 py-3"><span class="sr-only">{"Actions"}</span></th>
                            </tr>
                        </thead>
                        <tbody class="bg-white divide-y divide-gray-200">
                            { for expenses.iter().map(|expense| html! {
                                <tr key={expense.id.clone()}>
                                    <td class="px-6 py-4 whitespace-nowrap">{ &expense.description }</td>
                                    <td class="px-6 py-4 whitespace-nowrap">{ &expense.category }</td>
                                    <td class="px-6 py-4 whitespace-nowrap">{ format!("{} {}", expense.amount, expense.currency) }</td>
                                    <td class="px-6 py-4 whitespace-nowrap">{ &expense.transaction_date }</td>
                                    <td class="px-6 py-4 whitespace-nowrap">{ &expense.status }</td>
                                    <td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                                        <Link<ExpenseRoute> to={ExpenseRoute::Detail { id: expense.id.clone() }} classes="text-indigo-600 hover:text-indigo-900 mr-4">
                                            { "View" }
                                        </Link<ExpenseRoute>>
                                        <Link<ExpenseRoute> to={ExpenseRoute::Edit { id: expense.id.clone() }} classes="text-indigo-600 hover:text-indigo-900">
                                            { "Edit" }
                                        </Link<ExpenseRoute>>
                                    </td>
                                </tr>
                            })}
                        </tbody>
                    </table>
                </div>
            }
        </div>
    }
}