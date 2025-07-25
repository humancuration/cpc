use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::bindings::tauri::invoke;
use crate::invoicing::types::InvoiceNode;

#[derive(Properties, PartialEq, Clone)]
pub struct InvoiceListProps {}

#[function_component(InvoiceList)]
pub fn invoice_list(_props: &InvoiceListProps) -> Html {
    let invoices = use_state(|| vec![]);
    let is_loading = use_state(|| true);
    let error = use_state(|| None);

    {
        let invoices = invoices.clone();
        let is_loading = is_loading.clone();
        let error = error.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                let args = serde_json::to_value(&serde_json::json!({})).unwrap();
                match invoke("get_invoice_summaries", &args).await {
                    Ok(response) => {
                        if let Ok(json_str) = response.as_string() {
                            match serde_json::from_str::<Vec<InvoiceNode>>(&json_str) {
                                Ok(inv) => {
                                    invoices.set(inv);
                                }
                                Err(e) => {
                                    error.set(Some(format!("Failed to parse invoices: {}", e)));
                                }
                            }
                        } else {
                            error.set(Some("Failed to get invoice summaries response.".to_string()));
                        }
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to fetch invoice summaries: {:?}", e)));
                    }
                }
                is_loading.set(false);
            });
            || ()
        });
    }

    fn format_currency(amount: f64) -> String {
        // Basic currency formatting
        format!("${:.2}", amount)
    }

    fn format_date(date: &str) -> String {
        // Basic date formatting
        date.to_string()
    }

    fn get_status_color(status: &str) -> &'static str {
        match status {
            "Draft" => "bg-gray-100 text-gray-800",
            "Sent" => "bg-blue-100 text-blue-800",
            "Viewed" => "bg-purple-100 text-purple-800",
            "Paid" => "bg-green-100 text-green-800",
            "Overdue" => "bg-red-100 text-red-800",
            "Cancelled" => "bg-gray-100 text-gray-800",
            _ => "bg-gray-100 text-gray-800",
        }
    }

    html! {
        <div class="bg-white shadow-sm rounded-lg">
            <div class="px-4 py-5 sm:p-6">
                <div class="flex items-center justify-between mb-4">
                    <h3 class="text-lg font-medium text-gray-900">{"Invoices"}</h3>
                    <button
                        class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700"
                    >
                        {"New Invoice"}
                    </button>
                </div>

                {
                    if *is_loading {
                        html! {
                            <div class="text-center py-8">
                                <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600 mx-auto"></div>
                                <p class="mt-2 text-gray-500">{"Loading invoices..."}</p>
                            </div>
                        }
                    } else if let Some(err) = &*error {
                        html! {
                            <div class="text-center py-8">
                                <p class="text-red-600">{ err.clone() }</p>
                            </div>
                        }
                    } else if invoices.is_empty() {
                         html! {
                            <div class="text-center py-8">
                                <p class="text-gray-500">{"No invoices found"}</p>
                            </div>
                        }
                    } else {
                        html! {
                            <div class="overflow-x-auto">
                                <table class="min-w-full divide-y divide-gray-200">
                                    <thead>
                                        <tr>
                                            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">{"Invoice"}</th>
                                            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">{"Customer"}</th>
                                            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">{"Due Date"}</th>
                                            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">{"Total"}</th>
                                            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">{"Status"}</th>
                                            <th class="relative px-6 py-3">
                                                <span class="sr-only">{"Actions"}</span>
                                            </th>
                                        </tr>
                                    </thead>
                                    <tbody class="bg-white divide-y divide-gray-200">
                                        { for invoices.iter().map(|summary| html! {
                                            <tr class="hover:bg-gray-50">
                                                <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
                                                    <a href={format!("/invoices/{}", summary.id)} class="text-blue-600 hover:underline">
                                                        { &summary.invoice_number }
                                                    </a>
                                                </td>
                                                <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                                                    { &summary.customer_name }
                                                </td>
                                                <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                                                    { format_date(&summary.due_date) }
                                                </td>
                                                <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                                                    { format_currency(summary.total) }
                                                </td>
                                                <td class="px-6 py-4 whitespace-nowrap">
                                                    <span class={classes!("inline-flex", "items-center", "px-2.5", "py-0.5", "rounded-full", "text-xs", "font-medium", get_status_color(&summary.status))}>
                                                        { &summary.status }
                                                    </span>
                                                </td>
                                                <td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                                                    <button
                                                        class="text-blue-600 hover:text-blue-900"
                                                        // on:click|stopPropagation
                                                    >
                                                        {"Edit"}
                                                    </button>
                                                </td>
                                            </tr>
                                        }) }
                                    </tbody>
                                </table>
                            </div>
                        }
                    }
                 }
            </div>
        </div>
    }
}