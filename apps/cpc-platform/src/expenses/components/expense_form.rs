use yew::prelude::*;
use web_sys::HtmlInputElement;

#[derive(Clone, Default, PartialEq)]
pub struct ExpenseState {
    pub amount: String,
    pub currency: String,
    pub description: String,
    pub category: String,
    pub other_category: String,
    pub transaction_date: String,
}

const CATEGORIES: [&str; 5] = ["Travel", "Meals", "Software", "Hardware", "OfficeSupplies"];

#[derive(Properties, PartialEq)]
pub struct ExpenseFormProps {
    pub initial_state: ExpenseState,
    pub on_submit: Callback<ExpenseState>,
    pub loading: bool,
    pub error: Option<String>,
    pub submit_label: String,
}

#[function_component(ExpenseForm)]
pub fn expense_form(props: &ExpenseFormProps) -> Html {
    let state = use_state(|| props.initial_state.clone());

    let on_input = {
        let state = state.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut new_state = (*state).clone();
            match input.name().as_str() {
                "description" => new_state.description = input.value(),
                "amount" => new_state.amount = input.value(),
                "currency" => new_state.currency = input.value(),
                "category" => new_state.category = input.value(),
                "other_category" => new_state.other_category = input.value(),
                "transaction_date" => new_state.transaction_date = input.value(),
                _ => {}
            }
            state.set(new_state);
        })
    };

    let on_submit_handler = {
        let state = state.clone();
        let on_submit = props.on_submit.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            on_submit.emit((*state).clone());
        })
    };
    
    html! {
        <form onsubmit={on_submit_handler} class="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4">
            <div class="mb-4">
                <label class="block text-gray-700 text-sm font-bold mb-2" for="description">
                    { "Description" }
                </label>
                <input name="description" value={state.description.clone()} oninput={on_input.clone()}
                    class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                    type="text" placeholder="e.g., Team Lunch" required=true />
            </div>

            <div class="flex flex-wrap -mx-3 mb-4">
                <div class="w-full md:w-1/2 px-3 mb-4 md:mb-0">
                    <label class="block text-gray-700 text-sm font-bold mb-2" for="amount">
                        { "Amount" }
                    </label>
                    <input name="amount" value={state.amount.clone()} oninput={on_input.clone()}
                        class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                        type="number" step="0.01" placeholder="0.00" required=true />
                </div>
                <div class="w-full md:w-1/2 px-3">
                     <label class="block text-gray-700 text-sm font-bold mb-2" for="currency">
                        { "Currency" }
                    </label>
                    <input name="currency" value={state.currency.clone()} oninput={on_input.clone()}
                        class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                        type="text" placeholder="e.g., USD" required=true />
                </div>
            </div>

            <div class="mb-4">
                <label class="block text-gray-700 text-sm font-bold mb-2" for="category">
                    { "Category" }
                </label>
                <select name="category" oninput={on_input.clone()}
                    class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline">
                    <option value="" disabled=true selected={state.category.is_empty()}>{"Select a category"}</option>
                    { for CATEGORIES.iter().map(|cat| html!{<option value={cat.to_string()} selected={&state.category == cat}>{cat}</option>}) }
                    <option value="Other" selected={state.category == "Other"}>{"Other"}</option>
                </select>
            </div>

            if state.category == "Other" {
                <div class="mb-4">
                    <label class="block text-gray-700 text-sm font-bold mb-2" for="other_category">
                      { "Custom Category" }
                    </label>
                    <input name="other_category" value={state.other_category.clone()} oninput={on_input.clone()}
                        class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                         type="text" placeholder="Specify category" required=true />
                </div>
            }

            <div class="mb-6">
                <label class="block text-gray-700 text-sm font-bold mb-2" for="transaction_date">
                    { "Transaction Date" }
                </label>
                <input name="transaction_date" value={state.transaction_date.clone()} oninput={on_input}
                    class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                    type="date" required=true />
            </div>
            
            if let Some(err) = &props.error {
                <p class="text-red-500 text-xs italic mb-4">{ err.clone() }</p>
            }

            <div class="flex items-center justify-between">
                <button type="submit" disabled={props.loading}
                    class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline disabled:bg-gray-400">
                    { if props.loading { "Saving..." } else { &props.submit_label } }
                </button>
            </div>
        </form>
    }
}