use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ExpenseDetailProps {
    pub expense_id: String,
}

#[function_component(ExpenseDetail)]
pub fn expense_detail(props: &ExpenseDetailProps) -> Html {
    html! {
        <div>
            <h1>{ format!("Expense Details: {}", props.expense_id) }</h1>
            // Expense details will go here
        </div>
    }
}