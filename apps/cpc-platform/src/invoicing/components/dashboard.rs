use yew::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::Route;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use crate::invoicing::components::invoice_chart::InvoiceChart;
use crate::invoicing::types::{Invoice, InvoiceSummary, InvoiceNode};
use crate::bindings::tauri::invoke;

#[derive(Properties, PartialEq, Clone)]
pub struct InvoiceDashboardProps {
    pub organization_id: String,
}

#[function_component(InvoiceDashboard)]
pub fn invoice_dashboard(props: &InvoiceDashboardProps) -> Html {
    let summary_data = use_state(|| None::<InvoiceSummary>);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    {
        let summary_data = summary_data.clone();
        let loading = loading.clone();
        let error = error.clone();
        let org_id = props.organization_id.clone();

        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    loading.set(true);
                    let args = serde_json::to_string(&serde_json::json!({ "organizationId": org_id })).unwrap();
                    let result = invoke("fetch_invoice_dashboard_data", &JsValue::from_str(&args)).await;

                    match result {
                        Ok(data) => {
                            if let Some(json_str) = data.as_string() {
                                match from_str::<Vec<InvoiceNode>>(&json_str) {
                                    Ok(nodes) => {
                                        let summary = process_invoice_data(nodes);
                                        summary_data.set(Some(summary));
                                    }
                                    Err(e) => {
                                        error.set(Some(format!("Deserialization error: {}", e)));
                                    }
                                }
                            } else {
                                error.set(Some("Received non-string data from Tauri command".to_string()));
                            }
                        }
                        Err(e) => {
                            error.set(Some(format!("Tauri command failed: {:?}", e)));
                        }
                    }
                    loading.set(false);
                });
                || ()
            },
            (),
        );
    }

    fn process_invoice_data(nodes: Vec<InvoiceNode>) -> InvoiceSummary {
        let mut total_revenue = 0.0;
        let mut outstanding_invoices = 0.0;
        let mut overdue_invoices = 0.0;

        for node in &nodes {
            if node.status == "PAID" {
                total_revenue += node.total;
            } else if node.status == "SENT" || node.status == "PARTIAL" {
                outstanding_invoices += node.total;
            } else if node.status == "OVERDUE" {
                overdue_invoices += node.total;
            }
        }

        let recent_invoices = nodes
            .into_iter()
            .map(|node| Invoice {
                id: node.id,
                invoice_number: node.invoice_number,
                customer_name: node.customer.name,
                total: node.total,
                status: node.status,
                due_date: node.due_date,
            })
            .collect();

        InvoiceSummary {
            total_revenue,
            outstanding_invoices,
            overdue_invoices,
            recent_invoices,
        }
    }

    html! {
        <div class="p-6">
            <h1 class="text-2xl font-semibold text-gray-800 mb-4">{ "Invoicing Dashboard" }</h1>
            
            {
                if *loading {
                    html! { <p>{ "Loading..." }</p> }
                } else if let Some(err) = &*error {
                    html! { <p class="text-red-500">{ format!("Error: {}", err) }</p> }
                } else if let Some(summary) = &*summary_data {
                    html! {
                        <>
                            // Metric Cards
                            <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-6">
                                <div class="bg-white p-4 rounded-lg shadow">
                                    <h3 class="text-sm font-medium text-gray-500">{ "Total Revenue" }</h3>
                                    <p class="mt-1 text-2xl font-semibold text-gray-900">{ format!("${:.2}", summary.total_revenue) }</p>
                                </div>
                                <div class="bg-white p-4 rounded-lg shadow">
                                    <h3 class="text-sm font-medium text-gray-500">{ "Outstanding" }</h3>
                                    <p class="mt-1 text-2xl font-semibold text-gray-900">{ format!("${:.2}", summary.outstanding_invoices) }</p>
                                </div>
                                <div class="bg-white p-4 rounded-lg shadow">
                                    <h3 class="text-sm font-medium text-gray-500">{ "Overdue" }</h3>
                                    <p class="mt-1 text-2xl font-semibold text-red-600">{ format!("${:.2}", summary.overdue_invoices) }</p>
                                </div>
                            </div>

                            // Quick Actions
                            <div class="mb-6">
                                <Link<Route> to={Route::NewInvoice}>
                                    <button class="bg-blue-600 text-white px-4 py-2 rounded-lg shadow hover:bg-blue-700">
                                        { "Create New Invoice" }
                                    </button>
                                </Link<Route>>
                            </div>

                            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                                // Recent Invoices List
                                <div class="bg-white rounded-lg shadow">
                                    <h2 class="px-4 py-3 text-lg font-medium text-gray-800 border-b">{ "Recent Invoices" }</h2>
                                <table class="min-w-full divide-y divide-gray-200">
                                    <thead class="bg-gray-50">
                                        <tr>
                                            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">{ "Invoice #" }</th>
                                            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">{ "Customer" }</th>
                                            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">{ "Amount" }</th>
                                            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">{ "Status" }</th>
                                            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">{ "Due Date" }</th>
                                            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">{ "Actions" }</th>
                                        </tr>
                                    </thead>
                                    <tbody class="bg-white divide-y divide-gray-200">
                                        { for summary.recent_invoices.iter().map(|invoice| {
                                           let invoice_id = invoice.id.clone();
                                           let invoice_number = invoice.invoice_number.clone();
                                           let on_download_click = Callback::from(move |_| {
                                               let id = invoice_id.clone();
                                               let num = invoice_number.clone();
                                               spawn_local(async move {
                                                   let args = serde_json::to_string(&serde_json::json!({
                                                       "invoiceId": id,
                                                       "invoiceNumber": num
                                                   })).unwrap();
                                                   invoke("generate_invoice_pdf", &JsValue::from_str(&args)).await;
                                               });
                                           });

                                           html! {
                                               <tr>
                                                   <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">{ &invoice.invoice_number }</td>
                                                   <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">{ &invoice.customer_name }</td>
                                                   <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">{ format!("${:.2}", invoice.total) }</td>
                                                   <td class="px-6 py-4 whitespace-nowrap text-sm">
                                                       // Status Badge Logic
                                                       <span class={classes!("px-2", "inline-flex", "text-xs", "leading-5", "font-semibold", "rounded-full",
                                                           match invoice.status.as_str() {
                                                               "PAID" => "bg-green-100 text-green-800",
                                                               "SENT" => "bg-blue-100 text-blue-800",
                                                               "OVERDUE" => "bg-red-100 text-red-800",
                                                               _ => "bg-gray-100 text-gray-800",
                                                           }
                                                       )}>
                                                           { &invoice.status }
                                                       </span>
                                                   </td>
                                                   <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">{ &invoice.due_date }</td>
                                                   <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                                                       <button
                                                           onclick={on_download_click}
                                                           class="text-blue-600 hover:text-blue-800"
                                                       >
                                                           { "Download PDF" }
                                                       </button>
                                                   </td>
                                               </tr>
                                           }
                                        }) }
                                    </tbody>
                                </table>
                                </div>
                                <InvoiceChart invoices={summary.recent_invoices.clone()} />
                            </div>
                        </>
                    }
                } else {
                    html! { <p>{ "No data available." }</p> }
                }
            }
        </div>
    }
}