use yew::prelude::*;
use yew_router::prelude::*;
use yew_hooks::use_async;
use wasm_bindgen_futures::spawn_local;
use log::info;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::api;
use cpc_core::supply_chain::models::{ProductionStageData, StageConnectionData};
use crate::supply_chain::routing::SupplyChainRoute;
use crate::api::supply_chain::get_supply_chain_by_product;

#[derive(Properties, Clone, PartialEq)]
pub struct SupplyChainEditProps {
    pub id: String,
}

#[function_component(SupplyChainEdit)]
pub fn supply_chain_edit(props: &SupplyChainEditProps) -> Html {
    let navigator = use_navigator().unwrap();
    let product_id = props.id.clone();
    let product_uuid = Uuid::parse_str(&product_id).unwrap_or_default();

    // State for form fields
    let stages_state = use_state(Vec::<ProductionStageData>::new);
    let connections_state = use_state(Vec::<StageConnectionData>::new);

    // Fetch initial data
    let supply_chain_query = {
        let product_id = product_id.clone();
        use_async(async move { api::supply_chain::get_supply_chain_details(product_id).await })
    };

    use_effect_with((), {
        let supply_chain_query = supply_chain_query.clone();
        move |_| {
            supply_chain_query.run();
            || ()
        }
    });

    // Populate form state when data is loaded
    use_effect_with(supply_chain_query.data.clone(), {
        let stages_state = stages_state.clone();
        let connections_state = connections_state.clone();
        move |data| {
            if let Some(sc) = data {
                let stages = sc.stages.iter().map(|s| {
                    ProductionStageData {
                        id: Uuid::parse_str(&s.id).unwrap_or_default(),
                        name: s.name.clone(),
                        description: s.description.clone(),
                        location: s.location.clone(),
                        start_date: DateTime::parse_from_rfc3339(&s.start_date).unwrap().with_timezone(&Utc),
                        end_date: DateTime::parse_from_rfc3339(&s.end_date).unwrap().with_timezone(&Utc),
                    }
                }).collect();
                stages_state.set(stages);

                let connections = sc.connections.iter().map(|c| {
                    StageConnectionData {
                        from_stage_id: Uuid::parse_str(&c.from_stage_id).unwrap_or_default(),
                        to_stage_id: Uuid::parse_str(&c.to_stage_id).unwrap_or_default(),
                        relationship_type: c.relationship_type.clone(),
                    }
                }).collect();
                connections_state.set(connections);
            }
        }
    });
    
    // Handle form submission
    let onsubmit = {
        let stages_state = stages_state.clone();
        let connections_state = connections_state.clone();
        let navigator = navigator.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let navigator = navigator.clone();
            let stages = (*stages_state).clone();
            let connections = (*connections_state).clone();
            
            spawn_local(async move {
                info!("Submitting form data...");
                match api::supply_chain::update_supply_chain(product_uuid, stages, connections).await {
                    Ok(_) => {
                        info!("Update successful!");
                        navigator.push(&SupplyChainRoute::Detail { id: product_id.clone() });
                    }
                    Err(e) => {
                        log::error!("Error updating supply chain: {}", e);
                        // Maybe show an error message to the user
                    }
                }
            });
        })
    };
    
    let handle_stage_change = {
        let stages_state = stages_state.clone();
        Callback::from(move |(index, field, value): (usize, String, String)| {
            let mut stages = (*stages_state).clone();
            if let Some(stage) = stages.get_mut(index) {
                match field.as_str() {
                    "name" => stage.name = value,
                    "description" => stage.description = Some(value),
                    "location" => stage.location = value,
                    // TODO: Handle date parsing more robustly
                    "start_date" => if let Ok(dt) = DateTime::parse_from_rfc3339(&format!("{}T00:00:00Z", value)) {
                        stage.start_date = dt.with_timezone(&Utc);
                    },
                    "end_date" => if let Ok(dt) = DateTime::parse_from_rfc3339(&format!("{}T00:00:00Z", value)) {
                        stage.end_date = dt.with_timezone(&Utc);
                    },
                    _ => {}
                }
            }
            stages_state.set(stages);
        })
    };

    html! {
        <div class="container mx-auto px-4 py-8">
            <h1 class="text-3xl font-bold mb-6">{ format!("Edit Supply Chain for Product {}", &props.id) }</h1>

            if supply_chain_query.loading {
                <p>{ "Loading..." }</p>
            } else if let Some(error) = &supply_chain_query.error {
                <p class="text-red-500">{ format!("Error loading data: {}", error) }</p>
            } else if stages_state.is_empty() {
                <p>{ "No supply chain stages found to edit." }</p>
            } else {
                <form {onsubmit} class="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4">
                    <h2 class="text-2xl font-bold mb-4">{"Production Stages"}</h2>
                    { for stages_state.iter().enumerate().map(|(index, stage)| {
                        let oninput_name = handle_stage_change.reform(move |e: InputEvent| (index, "name".to_string(), e.target_unchecked_into::<web_sys::HtmlInputElement>().value()));
                        let oninput_desc = handle_stage_change.reform(move |e: InputEvent| (index, "description".to_string(), e.target_unchecked_into::<web_sys::HtmlInputElement>().value()));
                        let oninput_loc = handle_stage_change.reform(move |e: InputEvent| (index, "location".to_string(), e.target_unchecked_into::<web_sys::HtmlInputElement>().value()));
                        let oninput_start = handle_stage_change.reform(move |e: InputEvent| (index, "start_date".to_string(), e.target_unchecked_into::<web_sys::HtmlInputElement>().value()));
                        let oninput_end = handle_stage_change.reform(move |e: InputEvent| (index, "end_date".to_string(), e.target_unchecked_into::<web_sys::HtmlInputElement>().value()));

                        html! {
                            <div key={stage.id.to_string()} class="mb-6 p-4 border rounded">
                                <div class="mb-4">
                                    <label class="block text-gray-700 text-sm font-bold mb-2" for={format!("stage_name_{}", stage.id)}>{"Stage Name"}</label>
                                    <input oninput={oninput_name} value={stage.name.clone()}
                                        id={format!("stage_name_{}", stage.id)}
                                        class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" type="text" />
                                </div>
                                <div class="mb-4">
                                    <label class="block text-gray-700 text-sm font-bold mb-2" for={format!("stage_desc_{}", stage.id)}>{"Description"}</label>
                                    <input oninput={oninput_desc} value={stage.description.clone().unwrap_or_default()}
                                        id={format!("stage_desc_{}", stage.id)}
                                        class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" type="text" />
                                </div>
                                <div class="mb-4">
                                    <label class="block text-gray-700 text-sm font-bold mb-2" for={format!("stage_loc_{}", stage.id)}>{"Location"}</label>
                                    <input oninput={oninput_loc} value={stage.location.clone()}
                                        id={format!("stage_loc_{}", stage.id)}
                                        class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" type="text" />
                                </div>
                                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                                    <div class="mb-4">
                                        <label class="block text-gray-700 text-sm font-bold mb-2" for={format!("stage_start_{}", stage.id)}>{"Start Date"}</label>
                                        <input oninput={oninput_start} value={stage.start_date.format("%Y-%m-%d").to_string()}
                                            id={format!("stage_start_{}", stage.id)}
                                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" type="date" />
                                    </div>
                                    <div class="mb-4">
                                        <label class="block text-gray-700 text-sm font-bold mb-2" for={format!("stage_end_{}", stage.id)}>{"End Date"}</label>
                                        <input oninput={oninput_end} value={stage.end_date.format("%Y-%m-%d").to_string()}
                                            id={format!("stage_end_{}", stage.id)}
                                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" type="date" />
                                    </div>
                                </div>
                            </div>
                        }
                    })}

                    // TODO: Add UI for editing connections

                    <div class="flex items-center justify-between">
                        <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline" type="submit">
                            { "Update Supply Chain" }
                        </button>
                    </div>
                </form>
            }
        </div>
    }
}