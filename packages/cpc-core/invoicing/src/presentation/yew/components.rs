//! Yew components for invoicing and quoting

use yew::prelude::*;
use crate::domain::{Invoice, Quote, PaymentStatus, QuoteStatus, InvoiceItem, QuoteItem};
use uuid::Uuid;
use rust_decimal::Decimal;

/// Main component for invoicing functionality
pub struct InvoicingComponents;

/// Properties for invoice creation form
#[derive(Properties, PartialEq)]
pub struct InvoiceFormProps {
    pub on_create: Callback<Invoice>,
}

/// Properties for quote creation form
#[derive(Properties, PartialEq)]
pub struct QuoteFormProps {
    pub on_create: Callback<Quote>,
}

/// Properties for invoice list
#[derive(Properties, PartialEq)]
pub struct InvoiceListProps {
    pub invoices: Vec<Invoice>,
}

/// Properties for quote list
#[derive(Properties, PartialEq)]
pub struct QuoteListProps {
    pub quotes: Vec<Quote>,
}

/// Component for creating invoices
#[function_component(InvoiceForm)]
pub fn invoice_form(props: &InvoiceFormProps) -> Html {
    let client_name = use_state(|| String::new());
    let client_email = use_state(|| String::new());
    let description = use_state(|| String::new());
    let quantity = use_state(|| 1u32);
    let unit_price = use_state(|| String::new());
    
    let on_submit = {
        let on_create = props.on_create.clone();
        let client_name = client_name.clone();
        let client_email = client_email.clone();
        let description = description.clone();
        let quantity = *quantity;
        let unit_price = unit_price.clone();
        
        Callback::from(move |_| {
            // Parse unit price
            let price = unit_price.parse::<f64>().unwrap_or(0.0);
            
            // Create invoice item
            let item = InvoiceItem {
                description: (*description).clone(),
                quantity,
                unit_price: Decimal::from_f64(price).unwrap_or(Decimal::ZERO),
            };
            
            // Create invoice
            let invoice = Invoice {
                id: Uuid::new_v4(),
                client_id: Uuid::new_v4(), // In a real app, this would come from context
                client_name: (*client_name).clone(),
                client_email: (*client_email).clone(),
                items: vec![item],
                total_amount: Decimal::from_f64(price * (quantity as f64)).unwrap_or(Decimal::ZERO),
                due_date: chrono::Utc::now() + chrono::Duration::days(30),
                status: PaymentStatus::Draft,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };
            
            on_create.emit(invoice);
        })
    };
    
    html! {
        <div class="invoice-form">
            <h2>{"Create Invoice"}</h2>
            <form onsubmit={on_submit}>
                <div>
                    <label>{"Client Name:"}</label>
                    <input
                        type="text"
                        value={(*client_name).clone()}
                        oninput={Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            client_name.set(input.value());
                        })}
                    />
                </div>
                <div>
                    <label>{"Client Email:"}</label>
                    <input
                        type="email"
                        value={(*client_email).clone()}
                        oninput={Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            client_email.set(input.value());
                        })}
                    />
                </div>
                <div>
                    <label>{"Description:"}</label>
                    <input
                        type="text"
                        value={(*description).clone()}
                        oninput={Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            description.set(input.value());
                        })}
                    />
                </div>
                <div>
                    <label>{"Quantity:"}</label>
                    <input
                        type="number"
                        min="1"
                        value={quantity.to_string()}
                        oninput={Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            if let Ok(q) = input.value().parse::<u32>() {
                                quantity.set(q);
                            }
                        })}
                    />
                </div>
                <div>
                    <label>{"Unit Price:"}</label>
                    <input
                        type="number"
                        step="0.01"
                        value={(*unit_price).clone()}
                        oninput={Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            unit_price.set(input.value());
                        })}
                    />
                </div>
                <button type="submit">{"Create Invoice"}</button>
            </form>
        </div>
    }
}

/// Component for creating quotes
#[function_component(QuoteForm)]
pub fn quote_form(props: &QuoteFormProps) -> Html {
    let client_name = use_state(|| String::new());
    let client_email = use_state(|| String::new());
    let description = use_state(|| String::new());
    let quantity = use_state(|| 1u32);
    let unit_price = use_state(|| String::new());
    
    let on_submit = {
        let on_create = props.on_create.clone();
        let client_name = client_name.clone();
        let client_email = client_email.clone();
        let description = description.clone();
        let quantity = *quantity;
        let unit_price = unit_price.clone();
        
        Callback::from(move |_| {
            // Parse unit price
            let price = unit_price.parse::<f64>().unwrap_or(0.0);
            
            // Create quote item
            let item = QuoteItem {
                description: (*description).clone(),
                quantity,
                unit_price: Decimal::from_f64(price).unwrap_or(Decimal::ZERO),
            };
            
            // Create quote
            let quote = Quote {
                id: Uuid::new_v4(),
                client_id: Uuid::new_v4(), // In a real app, this would come from context
                client_name: (*client_name).clone(),
                client_email: (*client_email).clone(),
                items: vec![item],
                total_amount: Decimal::from_f64(price * (quantity as f64)).unwrap_or(Decimal::ZERO),
                validity_period: chrono::Duration::days(30),
                status: QuoteStatus::Draft,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };
            
            on_create.emit(quote);
        })
    };
    
    html! {
        <div class="quote-form">
            <h2>{"Create Quote"}</h2>
            <form onsubmit={on_submit}>
                <div>
                    <label>{"Client Name:"}</label>
                    <input
                        type="text"
                        value={(*client_name).clone()}
                        oninput={Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            client_name.set(input.value());
                        })}
                    />
                </div>
                <div>
                    <label>{"Client Email:"}</label>
                    <input
                        type="email"
                        value={(*client_email).clone()}
                        oninput={Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            client_email.set(input.value());
                        })}
                    />
                </div>
                <div>
                    <label>{"Description:"}</label>
                    <input
                        type="text"
                        value={(*description).clone()}
                        oninput={Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            description.set(input.value());
                        })}
                    />
                </div>
                <div>
                    <label>{"Quantity:"}</label>
                    <input
                        type="number"
                        min="1"
                        value={quantity.to_string()}
                        oninput={Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            if let Ok(q) = input.value().parse::<u32>() {
                                quantity.set(q);
                            }
                        })}
                    />
                </div>
                <div>
                    <label>{"Unit Price:"}</label>
                    <input
                        type="number"
                        step="0.01"
                        value={(*unit_price).clone()}
                        oninput={Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            unit_price.set(input.value());
                        })}
                    />
                </div>
                <button type="submit">{"Create Quote"}</button>
            </form>
        </div>
    }
}

/// Component for displaying a list of invoices
#[function_component(InvoiceList)]
pub fn invoice_list(props: &InvoiceListProps) -> Html {
    html! {
        <div class="invoice-list">
            <h2>{"Invoices"}</h2>
            <ul>
                {for props.invoices.iter().map(|invoice| {
                    html! {
                        <li key={invoice.id.to_string()}>
                            <div>
                                <strong>{&invoice.client_name}</strong>
                                <span>{format!(" - ${}", invoice.total_amount)}</span>
                            </div>
                            <div>
                                <span>{format!("Status: {:?}", invoice.status)}</span>
                                <span>{format!("Due: {}", invoice.due_date.format("%Y-%m-%d"))}</span>
                            </div>
                        </li>
                    }
                })}
            </ul>
        </div>
    }
}

/// Component for displaying a list of quotes
#[function_component(QuoteList)]
pub fn quote_list(props: &QuoteListProps) -> Html {
    html! {
        <div class="quote-list">
            <h2>{"Quotes"}</h2>
            <ul>
                {for props.quotes.iter().map(|quote| {
                    html! {
                        <li key={quote.id.to_string()}>
                            <div>
                                <strong>{&quote.client_name}</strong>
                                <span>{format!(" - ${}", quote.total_amount)}</span>
                            </div>
                            <div>
                                <span>{format!("Status: {:?}", quote.status)}</span>
                                <span>{format!("Valid until: {}", 
                                    (quote.created_at + quote.validity_period).format("%Y-%m-%d"))}</span>
                            </div>
                        </li>
                    }
                })}
            </ul>
        </div>
    }
}