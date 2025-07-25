use yew::prelude::*;
use yew_router::prelude::use_navigator;
use crate::routes::Route;
use crate::invoicing::types::{LineItem};
use crate::bindings::tauri::invoke;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use serde::{Deserialize, Serialize};
use chrono::Utc;
use validator::{Validate, ValidationError};

#[derive(Properties, PartialEq, Clone)]
pub struct InvoiceFormProps {
    pub organization_id: String,
}

#[derive(Validate, Debug, Clone, Default, Serialize)]
struct FormData {
    #[validate(length(min = 1, message = "Customer name is required"))]
    customer_name: String,
    #[validate(email(message = "Must be a valid email"))]
    customer_email: String,
    #[validate(length(min = 1, message = "Customer address is required"))]
    customer_address: String,
    issue_date: String,
    due_date: String,
    #[validate]
    line_items: Vec<LineItem>,
    notes: String,
    #[validate(range(min = 0, max = 100, message = "Tax rate must be between 0 and 100"))]
    tax_rate: f64,
}

#[function_component(InvoiceForm)]
pub fn invoice_form(props: &InvoiceFormProps) -> Html {
    let form_data = use_state(FormData::default);
    let is_saving = use_state(|| false);
    let error = use_state(|| None::<String>);
    let success = use_state(|| None::<String>);
    let navigator = use_navigator().unwrap();

    let on_add_item = {
        let form_data = form_data.clone();
        Callback::from(move |_| {
            let mut data = (*form_data).clone();
            data.line_items.push(LineItem::default());
            form_data.set(data);
        })
    };

    let on_remove_item = {
        let form_data = form_data.clone();
        Callback::from(move |index: usize| {
            let mut data = (*form_data).clone();
            data.line_items.remove(index);
            form_data.set(data);
        })
    };
    
    let onsubmit = {
        let form_data = form_data.clone();
        let is_saving = is_saving.clone();
        let error = error.clone();
        let success = success.clone();
        let navigator = navigator.clone();

        Callback::from(move |e: FocusEvent| {
            e.prevent_default();
            error.set(None);
            success.set(None);
            
            let data = (*form_data).clone();
            if let Err(e) = data.validate() {
                error.set(Some(format!("{}", e)));
                return;
            }

            is_saving.set(true);
            let navigator = navigator.clone();

            spawn_local(async move {
                let args = serde_json::to_string(&data).unwrap();
                let result = invoke("create_invoice", &JsValue::from_str(&args)).await;

                match result {
                    Ok(response) => {
                        let response_str = response.as_string().unwrap_or_default();
                        #[derive(Deserialize)]
                        struct CreateInvoiceResponse {
                            id: String,
                        }
                        let new_invoice: CreateInvoiceResponse = serde_json::from_str(&response_str).unwrap();

                        success.set(Some("Invoice created successfully!".to_string()));
                        is_saving.set(false);
                        // Redirect to the new invoice's detail page
                        navigator.push(&Route::InvoiceDetails { id: new_invoice.id });
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to create invoice: {:?}", e)));
                        is_saving.set(false);
                    }
                }
            });
        })
    };

    html! {
        <div class="p-8 bg-gray-50 min-h-screen">
            <h1 class="text-3xl font-bold text-gray-800 mb-6">{ "Create New Invoice" }</h1>
            
            <form {onsubmit} class="space-y-8 bg-white p-8 rounded-lg shadow-md">
                <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                    <div class="space-y-2">
                        <label class="block text-sm font-medium text-gray-700">{ "Customer Name" }</label>
                        <input type="text" class="w-full border-gray-300 rounded-lg shadow-sm" value={form_data.customer_name.clone()}
                            oninput={form_data.clone().reform(|e: InputEvent| { let mut data = (*form_data).clone(); data.customer_name = e.target_unchecked_into::<web_sys::HtmlInputElement>().value(); data })}/>
                    </div>
                    <div class="space-y-2">
                        <label class="block text-sm font-medium text-gray-700">{ "Customer Email" }</label>
                        <input type="email" class="w-full border-gray-300 rounded-lg shadow-sm" value={form_data.customer_email.clone()}
                            oninput={form_data.clone().reform(|e: InputEvent| { let mut data = (*form_data).clone(); data.customer_email = e.target_unchecked_into::<web_sys::HtmlInputElement>().value(); data })}/>
                    </div>
                </div>

                <div class="space-y-2">
                    <label class="block text-sm font-medium text-gray-700">{ "Customer Address" }</label>
                    <textarea class="w-full border-gray-300 rounded-lg shadow-sm" rows="3" value={form_data.customer_address.clone()}
                        oninput={form_data.clone().reform(|e: InputEvent| { let mut data = (*form_data).clone(); data.customer_address = e.target_unchecked_into::<web_sys::HtmlTextAreaElement>().value(); data })}></textarea>
                </div>
                
                <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                    <div class="space-y-2">
                        <label class="block text-sm font-medium text-gray-700">{ "Issue Date" }</label>
                        <input type="date" class="w-full border-gray-300 rounded-lg shadow-sm" value={form_data.issue_date.clone()}
                            oninput={form_data.clone().reform(|e: InputEvent| { let mut data = (*form_data).clone(); data.issue_date = e.target_unchecked_into::<web_sys::HtmlInputElement>().value(); data })}/>
                    </div>
                    <div class="space-y-2">
                        <label class="block text-sm font-medium text-gray-700">{ "Due Date" }</label>
                        <input type="date" class="w-full border-gray-300 rounded-lg shadow-sm" value={form_data.due_date.clone()}
                            oninput={form_data.clone().reform(|e: InputEvent| { let mut data = (*form_data).clone(); data.due_date = e.target_unchecked_into::<web_sys::HtmlInputElement>().value(); data })}/>
                    </div>
                </div>

                // Line Items
                <div class="space-y-4">
                    <h3 class="text-lg font-medium text-gray-800 border-b pb-2">{ "Line Items" }</h3>
                    { for form_data.line_items.iter().enumerate().map(|(index, item)| {
                        let form_data = form_data.clone();
                        let on_item_change = Callback::from(move |(field, value): (String, String)| {
                            let mut data = (*form_data).clone();
                            let item = &mut data.line_items[index];
                            match field.as_str() {
                                "description" => item.description = value,
                                "quantity" => item.quantity = value.parse().unwrap_or(0.0),
                                "unit_price" => item.unit_price = value.parse().unwrap_or(0.0),
                                _ => {}
                            }
                            form_data.set(data);
                        });

                        html! {
                        <div class="grid grid-cols-12 gap-4 items-center">
                            <div class="col-span-4 space-y-2">
                                <label class="text-sm font-medium text-gray-700">{ "Description" }</label>
                                <input type="text" class="w-full border-gray-300 rounded-lg" value={item.description.clone()}
                                    oninput={on_item_change.reform(move |e: InputEvent| ("description".to_string(), e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))} />
                            </div>
                            <div class="col-span-2 space-y-2">
                                <label class="text-sm font-medium text-gray-700">{ "Quantity" }</label>
                                <input type="number" class="w-full border-gray-300 rounded-lg" value={item.quantity.to_string()}
                                    oninput={on_item_change.reform(move |e: InputEvent| ("quantity".to_string(), e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))} />
                            </div>
                            <div class="col-span-2 space-y-2">
                                <label class="text-sm font-medium text-gray-700">{ "Unit Price" }</label>
                                <input type="number" class="w-full border-gray-300 rounded-lg" value={item.unit_price.to_string()}
                                    oninput={on_item_change.reform(move |e: InputEvent| ("unit_price".to_string(), e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))} />
                            </div>
                            <div class="col-span-2 space-y-2">
                                <label class="text-sm font-medium text-gray-700">{ "Total" }</label>
                                <p class="pt-2">{ format!("${:.2}", item.quantity * item.unit_price) }</p>
                            </div>
                            <div class="col-span-2 flex items-end">
                                <button type="button" onclick={on_remove_item.reform(move |_| index)} class="text-red-500 hover:text-red-700">
                                    { "Remove" }
                                </button>
                            </div>
                        </div>
                    }}) }
                    <button type="button" onclick={on_add_item} class="text-blue-600 hover:text-blue-800">
                        { "+ Add Item" }
                    </button>
                </div>
                
                // Notes, Tax, and Summary
                <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                    <div class="space-y-2">
                        <label class="block text-sm font-medium text-gray-700">{ "Notes" }</label>
                        <textarea class="w-full border-gray-300 rounded-lg shadow-sm" rows="4" value={form_data.notes.clone()}
                            oninput={form_data.clone().reform(|e: InputEvent| { let mut data = (*form_data).clone(); data.notes = e.target_unchecked_into::<web_sys::HtmlTextAreaElement>().value(); data })}></textarea>
                    </div>

                    <div class="space-y-6">
                        <div class="flex justify-between items-center">
                            <label class="text-sm font-medium text-gray-700">{ "Tax Rate (%)" }</label>
                            <input type="number" class="w-1/2 border-gray-300 rounded-lg shadow-sm" value={form_data.tax_rate.to_string()}
                                oninput={form_data.clone().reform(|e: InputEvent| { let mut data = (*form_data).clone(); data.tax_rate = e.target_unchecked_into::<web_sys::HtmlInputElement>().value().parse().unwrap_or(0.0); data })} />
                        </div>

                        <div class="border-t pt-4 space-y-2">
                            <div class="flex justify-between">
                                <span>{ "Subtotal" }</span>
                                <span>{ format!("${:.2}", form_data.line_items.iter().map(|i| i.quantity * i.unit_price).sum::<f64>()) }</span>
                            </div>
                            <div class="flex justify-between">
                                <span>{ "Tax" }</span>
                                <span>{ format!("${:.2}", form_data.line_items.iter().map(|i| i.quantity * i.unit_price).sum::<f64>() * (form_data.tax_rate / 100.0)) }</span>
                            </div>
                            <div class="flex justify-between font-bold text-lg">
                                <span>{ "Total" }</span>
                                <span>{ format!("${:.2}", form_data.line_items.iter().map(|i| i.quantity * i.unit_price).sum::<f64>() * (1.0 + form_data.tax_rate / 100.0)) }</span>
                            </div>
                        </div>
                    </div>
                </div>
                
                // General error / success messages
                if let Some(msg) = &*error {
                    <p class="text-red-500">{msg}</p>
                }
                if let Some(msg) = &*success {
                    <p class="text-green-500">{msg}</p>
                }

                <div class="flex justify-end">
                    <button type="submit" disabled={*is_saving} class="bg-blue-600 text-white px-6 py-2 rounded-lg shadow hover:bg-blue-700 disabled:bg-gray-400">
                        { if *is_saving { "Saving..." } else { "Save Invoice" } }
                    </button>
                </div>
            </form>
        </div>
    }
}