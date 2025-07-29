use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen_futures::spawn_local;
use log::info;
use uuid::Uuid;
use chrono::{Utc, TimeZone};

use crate::api;
use crate::supply_chain::routing::SupplyChainRoute;
use crate::api::supply_chain::{UpdateStageData, get_stage_by_id};

#[derive(Properties, Clone, PartialEq)]
pub struct EditStageComponentProps {
    pub stage_id: String,
}

#[function_component(EditStageComponent)]
pub fn edit_stage_component(props: &EditStageComponentProps) -> Html {
    let navigator = use_navigator().unwrap();
    let stage_id = props.stage_id.clone();
    let stage_uuid = Uuid::parse_str(&stage_id).unwrap_or_default();

    let stage_state = use_state(|| None::<api::supply_chain::ProductionStage>);
    let form_state = use_state(UpdateStageData::default);
    let error_message = use_state(|| None::<String>);

    // Fetch stage data on mount
    {
        let stage_state = stage_state.clone();
        let form_state = form_state.clone();
        let stage_uuid = stage_uuid;
        let error_message = error_message.clone();
        
        use_effect_with_deps(move |_| {
            spawn_local(async move {
                info!("Fetching stage data for {}", stage_uuid);
                match get_stage_by_id(stage_uuid).await {
                    Ok(stage) => {
                        stage_state.set(Some(stage.clone()));
                        form_state.set(UpdateStageData {
                            name: stage.name.clone(),
                            description: stage.description.clone(),
                            location: stage.location.clone(),
                            start_date: stage.start_date,
                            end_date: stage.end_date,
                        });
                    }
                    Err(e) => {
                        let error_msg = format!("Failed to load stage: {}", e);
                        error_message.set(Some(error_msg));
                        log::error!("Error fetching stage: {}", e);
                    }
                }
            });
            || ()
        }, stage_uuid);
    }

    let onsubmit = {
        let form_state = form_state.clone();
        let navigator = navigator.clone();
        let error_message = error_message.clone();
        let stage_uuid = stage_uuid;
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let navigator = navigator.clone();
            let stage_data = (*form_state).clone();
            let error_message = error_message.clone();
            
            spawn_local(async move {
                info!("Updating stage data...");
                match api::supply_chain::update_supply_chain_stage(stage_uuid, stage_data).await {
                    Ok(_) => {
                        info!("Stage update successful!");
                        if let Some(stage) = &*stage_state {
                            navigator.push(&SupplyChainRoute::ProductStages { id: stage.product_id });
                        }
                    }
                    Err(e) => {
                        let error_msg = format!("Failed to update stage: {}", e);
                        error_message.set(Some(error_msg));
                        log::error!("Error updating supply chain stage: {}", e);
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
            <h1 class="text-3xl font-bold mb-6">{ "Edit Production Stage" }</h1>

            if let Some(stage) = &*stage_state {
                <form {onsubmit} class="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4">
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
                            { "Update Stage" }
                        </button>
                        <Link<SupplyChainRoute> to={SupplyChainRoute::ProductStages { id: stage.product_id }} classes="bg-gray-500 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline">
                            { "Cancel" }
                        </Link<SupplyChainRoute>>
                    </div>
                </form>
            } else if let Some(msg) = &*error_message {
                <p class="text-red-500">{ msg }</p>
            } else {
                <p>{ "Loading stage data..." }</p>
            }
        </div>
    }
}