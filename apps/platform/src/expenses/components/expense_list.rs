use yew::prelude::*;

#[function_component(ExpenseList)]
pub fn expense_list() -> Html {
    html! {
        <div>
            <h1>{"Expenses"}</h1>
            // List of expenses will go here
        </div>
    }
}