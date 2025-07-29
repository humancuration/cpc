use yew::prelude::*;
use yew_hooks::use_async;
use web_sys::HtmlInputElement;
use crate::types::invoice::{Invoice, InvoiceItem, InvoiceTemplate, Contact};
use crate::api::invoicing::InvoicingService;

#[derive(Properties, PartialEq)]
pub struct InvoiceCreatorProps {
    pub on_invoice_created: Option<Callback<String>>,
}

#[function_component(InvoiceCreator)]
pub fn invoice_creator(props: &InvoiceCreatorProps) -> Html {
    let invoice = use_state(Invoice::default);
    let templates = use_state(|| Vec::<InvoiceTemplate>::new());
    let contacts = use_state(|| Vec::<Contact>::new());
    
    let invoicing_service = use_state(|| InvoicingService::new("/api/graphql".to_string()));
    
    let load_data = {
        let templates = templates.clone();
        let contacts = contacts.clone();
        let invoicing_service = invoicing_service.clone();
        
        use_async(async move {
            let templates_result = invoicing_service.get_templates().await;
            let contacts_result = invoicing_service.get_contacts().await;
            
            match (templates_result, contacts_result) {
                (Ok(templates_data), Ok(contacts_data)) => {
                    templates.set(templates_data);
                    contacts.set(contacts_data);
                    Ok(())
                }
                (Err(e), _) | (_, Err(e)) => Err(e.to_string()),
            }
        })
    };
    
    let create_invoice = {
        let invoice = invoice.clone();
        let invoicing_service = invoicing_service.clone();
        let on_created = props.on_invoice_created.clone();
        
        use_async(async move {
            let result = invoicing_service.create_invoice_mutation((*invoice).clone()).await;
            match result {
                Ok(created_invoice) => {
                    if let Some(callback) = on_created {
                        callback.emit(created_invoice.id.unwrap_or_default());
                    }
                    Ok(())
                }
                Err(e) => Err(e.to_string()),
            }
        })
    };
    
    {
        let load_data = load_data.clone();
        use_effect_with_deps(move |_| {
            load_data.run();
            || ()
        }, ());
    }
    
    let handle_recipient_change = {
        let invoice = invoice.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut new_invoice = (*invoice).clone();
            new_invoice.recipient_id = input.value();
            invoice.set(new_invoice);
        })
    };
    
    let handle_date_change = {
        let invoice = invoice.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Ok(date) = chrono::DateTime::parse_from_rfc3339(&format!("{}T00:00:00Z", input.value())) {
                let mut new_invoice = (*invoice).clone();
                new_invoice.due_date = date.with_timezone(&chrono::Utc);
                invoice.set(new_invoice);
            }
        })
    };
    
    let handle_template_change = {
        let invoice = invoice.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut new_invoice = (*invoice).clone();
            new_invoice.template_id = if input.value().is_empty() { None } else { Some(input.value()) };
            invoice.set(new_invoice);
        })
    };
    
    let handle_tax_rate_change = {
        let invoice = invoice.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut new_invoice = (*invoice).clone();
            new_invoice.tax_rate = input.value().parse().unwrap_or(0.0);
            invoice.set(new_invoice);
        })
    };
    
    let handle_discount_change = {
        let invoice = invoice.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut new_invoice = (*invoice).clone();
            new_invoice.discount = input.value().parse().unwrap_or(0.0);
            invoice.set(new_invoice);
        })
    };
    
    let handle_notes_change = {
        let invoice = invoice.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut new_invoice = (*invoice).clone();
            new_invoice.notes = if input.value().is_empty() { None } else { Some(input.value()) };
            invoice.set(new_invoice);
        })
    };
    
    let handle_item_description_change = {
        let invoice = invoice.clone();
        Callback::from(move |(index, value): (usize, String)| {
            let mut new_invoice = (*invoice).clone();
            if let Some(item) = new_invoice.items.get_mut(index) {
                item.description = value;
            }
            invoice.set(new_invoice);
        })
    };
    
    let handle_item_quantity_change = {
        let invoice = invoice.clone();
        Callback::from(move |(index, value): (usize, f64)| {
            let mut new_invoice = (*invoice).clone();
            if let Some(item) = new_invoice.items.get_mut(index) {
                item.quantity = value;
            }
            invoice.set(new_invoice);
        })
    };
    
    let handle_item_price_change = {
        let invoice = invoice.clone();
        Callback::from(move |(index, value): (usize, f64)| {
            let mut new_invoice = (*invoice).clone();
            if let Some(item) = new_invoice.items.get_mut(index) {
                item.unit_price = value;
            }
            invoice.set(new_invoice);
        })
    };
    
    let add_item = {
        let invoice = invoice.clone();
        Callback::from(move |_| {
            let mut new_invoice = (*invoice).clone();
            new_invoice.items.push(InvoiceItem::default());
            invoice.set(new_invoice);
        })
    };
    
    let remove_item = {
        let invoice = invoice.clone();
        Callback::from(move |index: usize| {
            let mut new_invoice = (*invoice).clone();
            if new_invoice.items.len() > 1 {
                new_invoice.items.remove(index);
                invoice.set(new_invoice);
            }
        })
    };
    
    let handle_submit = {
        let create_invoice = create_invoice.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if (*invoice).is_valid() {
                create_invoice.run();
            }
        })
    };
    
    let subtotal = invoice.subtotal();
    let tax = invoice.tax_amount();
    let total = invoice.total();
    
    html! {
        <div class="invoice-creator">
            <h2>{ "Create New Invoice" }</h2>
            
            if let Some(error) = &create_invoice.error {
                <div class="error">{ error }</div>
            }
            
            if load_data.loading {
                <div class="loading">{ "Loading..." }</div>
            } else if let Some(error) = &load_data.error {
                <div class="error">{ error }</div>
            } else {
                <form onsubmit={handle_submit}>
                    <div class="form-section">
                        <h3>{ "Invoice Details" }</h3>
                        
                        <label>
                            { "Recipient" }
                            <select 
                                onchange={handle_recipient_change} 
                                required=true
                                value={invoice.recipient_id.clone()}
                            >
                                <option value="">{ "Select recipient" }</option>
                                { for contacts.iter().map(|contact| html! {
                                    <option value={contact.id.clone()}>
                                        { format!("{} ({})", contact.name, contact.email) }
                                    </option>
                                })}
                            </select>
                        </label>

                        <label>
                            { "Due Date" }
                            <input 
                                type="date" 
                                onchange={handle_date_change}
                                value={invoice.due_date.format("%Y-%m-%d").to_string()}
                                required=true 
                            />
                        </label>

                        <label>
                            { "Template" }
                            <select onchange={handle_template_change}>
                                <option value="">{ "Default template" }</option>
                                { for templates.iter().map(|template| html! {
                                    <option value={template.id.clone()}>
                                        { &template.name }
                                    </option>
                                })}
                            </select>
                        </label>
                    </div>

                    <div class="form-section">
                        <h3>{ "Items" }</h3>
                        
                        { for invoice.items.iter().enumerate().map(|(index, item)| html! {
                            <div class="item-row">
                                <input 
                                    type="text" 
                                    placeholder="Description" 
                                    value={item.description.clone()}
                                    oninput={Callback::from({
                                        let handle_item_description_change = handle_item_description_change.clone();
                                        move |e: InputEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            handle_item_description_change.emit((index, input.value()));
                                        }
                                    })}
                                    required=true
                                />
                                <input 
                                    type="number" 
                                    placeholder="Quantity" 
                                    value={item.quantity.to_string()}
                                    oninput={Callback::from({
                                        let handle_item_quantity_change = handle_item_quantity_change.clone();
                                        move |e: InputEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            handle_item_quantity_change.emit((index, input.value().parse().unwrap_or(0.0)));
                                        }
                                    })}
                                    min="0"
                                    step="0.01"
                                    required=true
                                />
                                <input 
                                    type="number" 
                                    placeholder="Unit Price" 
                                    value={item.unit_price.to_string()}
                                    oninput={Callback::from({
                                        let handle_item_price_change = handle_item_price_change.clone();
                                        move |e: InputEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            handle_item_price_change.emit((index, input.value().parse().unwrap_or(0.0)));
                                        }
                                    })}
                                    min="0"
                                    step="0.01"
                                    required=true
                                />
                                <span>{ format!("${:.2}", item.calculate_total()) }</span>
                                { if invoice.items.len() > 1 {
                                    html! {
                                        <button 
                                            type="button" 
                                            onclick={Callback::from({
                                                let remove_item = remove_item.clone();
                                                move |_| remove_item.emit(index)
                                            })}
                                        >
                                            { "Remove" }
                                        </button>
                                    }
                                } else {
                                    html! { <span></span> }
                                }}
                            </div>
                        })}
                        
                        <button type="button" onclick={add_item}>
                            { "Add Item" }
                        </button>
                    </div>

                    <div class="form-section">
                        <h3>{ "Pricing" }</h3>
                        
                        <label>
                            { "Tax Rate (%)" }
                            <input 
                                type="number" 
                                value={invoice.tax_rate.to_string()} 
                                onchange={handle_tax_rate_change}
                                min="0" 
                                max="100" 
                                step="0.01" 
                            />
                        </label>

                        <label>
                            { "Discount ($)" }
                            <input 
                                type="number" 
                                value={invoice.discount.to_string()} 
                                onchange={handle_discount_change}
                                min="0" 
                                step="0.01" 
                            />
                        </label>

                        <div class="summary">
                            <p>{ format!("Subtotal: ${:.2}", subtotal) }</p>
                            <p>{ format!("Tax: ${:.2}", tax) }</p>
                            <p>{ format!("Discount: -${:.2}", invoice.discount) }</p>
                            <p><strong>{ format!("Total: ${:.2}", total) }</strong></p>
                        </div>
                    </div>

                    <div class="form-section">
                        <h3>{ "Notes" }</h3>
                        <textarea 
                            rows=4 
                            placeholder="Additional notes..."
                            onchange={handle_notes_change}
                        />
                    </div>

                    <button type="submit" disabled={create_invoice.loading}>
                        { if create_invoice.loading { "Creating..." } else { "Create Invoice" } }
                    </button>
                </form>
            }
        </div>
    }
}