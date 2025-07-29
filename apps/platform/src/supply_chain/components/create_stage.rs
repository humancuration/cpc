use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen_futures::spawn_local;
use log::info;
use uuid::Uuid;
use chrono::{Utc, TimeZone};

use crate::api;
use crate::supply_chain::routing::SupplyChainRoute;
use crate::api::supply_chain::CreateStageData;

#[derive(Properties, Clone, PartialEq)]
pub struct CreateStageComponentProps {
    pub product_id: String,
}

#[function_component(CreateStageComponent)]
pub fn create_stage_component(props: &CreateStageComponentProps) -> Html {
    let navigator = use_navigator().unwrap();
    let product_id = props.product_id.clone();
    let product_uuid = Uuid::parse_str(&product_id).unwrap_or_default();

    let form_state = use_state(CreateStageData::default);
    let error_message = use_state(|| None::<String>); // New error state

    let onsubmit = {
        let form_state = form_state.clone();
        let navigator = navigator.clone();
        let error_message = error_message.clone(); // Capture error state
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let navigator = navigator.clone();
            let stage_data = (*form_state).clone();
            let product_uuid_for_nav = product_uuid;
            let error_message = error_message.clone(); // Capture for async block
            
            spawn_local(async move {
                info!("Submitting new stage data...");
                match api::supply_chain::create_supply_chain_stage(product_uuid, stage_data).await {
                    Ok(_) => {
                        info!("Stage creation successful!");
                        navigator.push(&SupplyChainRoute::ProductStages { id: product_uuid_for_nav });
                    }
                    Err(e) => {
                        let error_msg = format!("Failed to create stage: {}", e);
                        error_message.set(Some(error_msg));
                        log::error!("Error creating supply chain stage: {}", e);
                    }
                }
            });
        })
    };

    let handle_input = {
        let form_state = form_state.clone();
        Callback::from(move |(field, value): (String, String)| {
            let mut data = (*form_state).clone();
            match field.as_str() {
                "name" => data.name = value,
                "description" => data.description = Some(value),
                "location" => data.location = value,
                "start_date" => {
                    if let Ok(dt) = Utc.datetime_from_str(&format!("{} 00:00:00", value), "%Y-%m-%d %H:%M:%S") {
                         data.start_date = dt;
                    }
                },
                "end_date" => {
                    if let Ok(dt) = Utc.datetime_from_str(&format!("{} 00:00:00", value), "%Y-%m-%d %H:%M:%S") {
                        data.end_date = dt;
                    }
                },
                _ => {}
            }
            form_state.set(data);
        })
    };

    let oninput = |field: &str| {
        let handle_input = handle_input.clone();
        let field = field.to_string();
        Callback::from(move |e: InputEvent| {
            let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
            handle_input.emit((field.clone(), value));
        })
    };

    html! {
        <div class="container mx-auto px-4 py-8">
            <h1 class="text-3xl font-bold mb-6">{ format!("Add New Stage to Supply Chain for Product {}", &props.product_id) }</h1>

            <form {onsubmit} class="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4">
                // Error message display
                if let Some(msg) = &*error_message {
                    <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded relative mb-4" role="alert">
                        <span class="block sm:inline">{msg}</span>
                    </div>
                }
                
                <div class="mb-4">
                    <label class="block text-gray-700 text-sm font-bold mb-2" for="stage_name">{"Stage Name"}</label>
                    <input oninput={oninput("name")} value={form_state.name.clone()}
                        id="stage_name"
                        class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" type="text" required=true />
                </div>
                <div class="mb-4">
                    <label class="block text-gray-700 text-sm font-bold mb-2" for="stage_desc">{"Description"}</label>
                    <input oninput={oninput("description")} value={form_state.description.clone().unwrap_or_default()}
                        id="stage_desc"
                        class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" type="text" />
                </div>
                <div class="mb-4">
                    <label class="block text-gray-700 text-sm font-bold mb-2" for="stage_loc">{"Location"}</label>
                    <input oninput={oninput("location")} value={form_state.location.clone()}
                        id="stage_loc"
                        class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" type="text" required=true />
                </div>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <div class="mb-4">
                        <label class="block text-gray-700 text-sm font-bold mb-2" for="stage_start">{"Start Date"}</label>
                        <input oninput={oninput("start_date")} value={form_state.start_date.format("%Y-%m-%d").to_string()}
                            id="stage_start"
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" type="date" required=true />
                    </div>
                    <div class="mb-4">
                        <label class="block text-gray-700 text-sm font-bold mb-2" for="stage_end">{"End Date"}</label>
                        <input oninput={oninput("end_date")} value={form_state.end_date.format("%Y-%m-%d").to_string()}
                            id="stage_end"
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" type="date" required=true />
                    </div>
                </div>
                <div class="flex items-center justify-between">
                    <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline" type="submit">
                        { "Create Stage" }
                    </button>
                    <Link<SupplyChainRoute> to={SupplyChainRoute::ProductStages { id: product_uuid }} classes="bg-gray-500 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline">
                        { "Cancel" }
                    </Link<SupplyChainRoute>>
                </div>
            </form>
        </div>
    }
}