use yew::prelude::*;
use yew_hooks::use_async;
use web_sys::HtmlInputElement;
use crate::types::expense::Expense;
use crate::api::expenses::ExpenseService;

#[derive(Properties, PartialEq)]
pub struct ExpenseCreatorProps {
    pub on_expense_created: Option<Callback<String>>,
}

#[function_component(ExpenseCreator)]
pub fn expense_creator(props: &ExpenseCreatorProps) -> Html {
    let expense = use_state(Expense::default);
    let expense_service = use_state(|| ExpenseService::new("/api/graphql".to_string()));
    
    let create_expense = {
        let expense = expense.clone();
        let expense_service = expense_service.clone();
        let on_created = props.on_expense_created.clone();
        
        use_async(async move {
            let result = expense_service.create_expense_mutation((*expense).clone()).await;
            match result {
                Ok(created_expense) => {
                    if let Some(callback) = on_created {
                        callback.emit(created_expense.id.unwrap_or_default());
                    }
                    Ok(())
                }
                Err(e) => Err(e.to_string()),
            }
        })
    };
    
    let handle_project_id_change = {
        let expense = expense.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut new_expense = (*expense).clone();
            new_expense.project_id = if input.value().is_empty() { None } else { Some(input.value()) };
            expense.set(new_expense);
        })
    };
    
    let handle_description_change = {
        let expense = expense.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut new_expense = (*expense).clone();
            new_expense.description = input.value();
            expense.set(new_expense);
        })
    };
    
    let handle_category_change = {
        let expense = expense.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut new_expense = (*expense).clone();
            new_expense.category = input.value();
            expense.set(new_expense);
        })
    };
    
    let handle_amount_change = {
        let expense = expense.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut new_expense = (*expense).clone();
            new_expense.amount = input.value().parse().unwrap_or(0.0);
            expense.set(new_expense);
        })
    };
    
    let handle_date_change = {
        let expense = expense.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Ok(date) = chrono::DateTime::parse_from_rfc3339(&format!("{}T00:00:00Z", input.value())) {
                let mut new_expense = (*expense).clone();
                new_expense.transaction_date = date.with_timezone(&chrono::Utc);
                expense.set(new_expense);
            }
        })
    };
    
    let handle_currency_change = {
        let expense = expense.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut new_expense = (*expense).clone();
            new_expense.currency = input.value();
            expense.set(new_expense);
        })
    };
    
    let handle_submit = {
        let create_expense = create_expense.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if (*expense).is_valid() {
                create_expense.run();
            }
        })
    };
    
    html! {
        <div class="expense-creator">
            <h2>{ "Create New Expense" }</h2>
            
            if let Some(error) = &create_expense.error {
                <div class="error">{ error }</div>
            }
            
            <form onsubmit={handle_submit}>
                <div class="form-section">
                    <h3>{ "Expense Details" }</h3>
                    
                    <label>
                        { "Project ID (Optional)" }
                        <input 
                            type="text" 
                            placeholder="Project ID"
                            onchange={handle_project_id_change}
                            value={expense.project_id.clone().unwrap_or_default()}
                        />
                    </label>
                    
                    <label>
                        { "Category" }
                        <input 
                            type="text" 
                            placeholder="e.g., Office Supplies, Travel, Meals"
                            onchange={handle_category_change}
                            value={expense.category.clone()}
                            required=true
                        />
                    </label>
                    
                    <label>
                        { "Description" }
                        <input 
                            type="text" 
                            placeholder="Brief description of the expense"
                            onchange={handle_description_change}
                            value={expense.description.clone()}
                            required=true
                        />
                    </label>
                    
                    <label>
                        { "Amount" }
                        <input 
                            type="number" 
                            placeholder="0.00"
                            onchange={handle_amount_change}
                            value={expense.amount.to_string()}
                            min="0"
                            step="0.01"
                            required=true
                        />
                    </label>
                    
                    <label>
                        { "Currency" }
                        <input 
                            type="text" 
                            placeholder="e.g., USD, EUR"
                            onchange={handle_currency_change}
                            value={expense.currency.clone()}
                            required=true
                        />
                    </label>
                    
                    <label>
                        { "Date" }
                        <input 
                            type="date" 
                            onchange={handle_date_change}
                            value={expense.transaction_date.format("%Y-%m-%d").to_string()}
                            required=true 
                        />
                    </label>
                </div>
                
                <button type="submit" disabled={create_expense.loading}>
                    { if create_expense.loading { "Creating..." } else { "Create Expense" } }
                </button>
            </form>
        </div>
    }
}