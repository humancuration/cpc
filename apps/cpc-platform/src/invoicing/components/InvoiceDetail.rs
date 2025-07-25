use yew::prelude::*;
use crate::invoicing::types::{Customer, Invoice, LineItem};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;

use crate::bindings::tauri::invoke;

#[derive(Properties, PartialEq, Clone)]
pub struct InvoiceDetailProps {
    pub invoice_id: String,
}

#[function_component(InvoiceDetail)]
pub fn invoice_detail(props: &InvoiceDetailProps) -> Html {
    let invoice = use_state(|| None);
    let is_loading = use_state(|| true);
    let error = use_state(|| None);
    let is_downloading = use_state(|| false);

    {
        let invoice = invoice.clone();
        let is_loading = is_loading.clone();
        let error = error.clone();
        let invoice_id = props.invoice_id.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                let args = serde_json::to_value(&serde_json::json!({ "invoiceId": invoice_id })).unwrap();
                match invoke("get_invoice_details", &args).await {
                    Ok(response) => {
                        if let Ok(json_str) = response.as_string() {
                            match serde_json::from_str::<Invoice>(&json_str) {
                                Ok(inv) => {
                                    invoice.set(Some(inv));
                                }
                                Err(e) => {
                                    error.set(Some(format!("Failed to parse invoice: {}", e)));
                                }
                            }
                        } else {
                            error.set(Some("Failed to get invoice details response.".to_string()));
                        }
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to fetch invoice details: {:?}", e)));
                    }
                }
                is_loading.set(false);
            });
            || ()
        });
    }

    let on_download_click = {
        let is_downloading = is_downloading.clone();
        let error = error.clone();
        let invoice = invoice.clone();
        Callback::from(move |_| {
            if let Some(inv) = &*invoice {
                let is_downloading = is_downloading.clone();
                let error = error.clone();
                let invoice_id = inv.id.clone();
                let invoice_number = inv.invoice_number.clone();
                
                is_downloading.set(true);
                error.set(None);

                spawn_local(async move {
                    let args = serde_json::to_value(&serde_json::json!({
                        "invoice_id": invoice_id,
                        "invoice_number": invoice_number
                    })).unwrap();

                    match invoke("generate_invoice_pdf", &args).await {
                        Ok(_) => {
                            // Success message could be shown here
                        }
                        Err(e) => {
                            error.set(Some(format!("Failed to generate PDF: {:?}", e)));
                        }
                    }
                    is_downloading.set(false);
                });
            }
        })
    };


    html! {
        <div class="container mx-auto p-4">
            {
                if *is_loading {
                    html! { <p>{"Loading invoice details..."}</p> }
                } else if let Some(err) = &*error {
                    html! { <p class="text-red-500">{ err.clone() }</p> }
                } else if let Some(inv) = &*invoice {
                    html! {
                        <div class="bg-white shadow-md rounded-lg p-6">
                           <div class="flex justify-between items-center mb-4">
                                <h1 class="text-2xl font-bold">{format!("Invoice {}", inv.invoice_number)}</h1>
                                <button
                                    class={classes!("px-4", "py-2", "font-bold", "text-white", "bg-blue-500", "rounded", "hover:bg-blue-700", "disabled:bg-gray-400")}
                                    onclick={on_download_click}
                                    disabled={*is_downloading}
                                >
                                    { if *is_downloading { "Downloading..." } else { "Download PDF" } }
                                </button>
                            </div>
                            <div class="grid grid-cols-2 gap-8 mb-8">
                                <div>
                                    <h2 class="font-bold text-gray-700">{"Bill To:"}</h2>
                                    <p>{&inv.customer.name}</p>
                                    <p>{&inv.customer.email}</p>
                                    <p>{&inv.customer.address}</p>
                                </div>
                                <div class="text-right">
                                    <p><strong>{"Status:"}</strong> <span class="px-2 py-1 text-sm rounded-full bg-green-200 text-green-800">{&inv.status}</span></p>
                                    <p><strong>{"Issue Date:"}</strong> {&inv.issue_date}</p>
                                    <p><strong>{"Due Date:"}</strong> {&inv.due_date}</p>
                                </div>
                            </div>

                            <div class="w-full overflow-x-auto">
                                <table class="min-w-full bg-white">
                                    <thead class="bg-gray-100">
                                        <tr>
                                            <th class="py-2 px-4 text-left">{"Description"}</th>
                                            <th class="py-2 px-4 text-right">{"Quantity"}</th>
                                            <th class="py-2 px-4 text-right">{"Unit Price"}</th>
                                            <th class="py-2 px-4 text-right">{"Total"}</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        { for inv.line_items.iter().map(|item| html! {
                                            <tr class="border-b">
                                                <td class="py-2 px-4">{&item.description}</td>
                                                <td class="py-2 px-4 text-right">{format!("{:.2}", item.quantity)}</td>
                                                <td class="py-2 px-4 text-right">{format!("${:.2}", item.unit_price)}</td>
                                                <td class="py-2 px-4 text-right">{format!("${:.2}", item.total)}</td>
                                            </tr>
                                        })}
                                    </tbody>
                                </table>
                            </div>

                            <div class="mt-6 border-t pt-4">
                                <div class="flex justify-end">
                                    <div class="w-1/3">
                                        <div class="flex justify-between">
                                            <span>{"Subtotal"}</span>
                                            <span>{format!("${:.2}", inv.line_items.iter().map(|i| i.total).sum::<f64>())}</span>
                                        </div>
                                        <div class="flex justify-between font-bold text-lg mt-2">
                                            <span>{"Total"}</span>
                                            <span>{format!("${:.2}", inv.total)}</span>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    }
                } else {
                    html! { <p>{"No invoice found."}</p> }
                }
            }
        </div>
    }
}