use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::*;
use uuid::Uuid;
use crate::api::supply_chain::{self, list_stages_for_product};
use crate::supply_chain::routing::SupplyChainRoute;

#[derive(Properties, PartialEq, Clone)]
pub struct StageListProps {
    pub product_id: String,
}

#[function_component(StageListComponent)]
pub fn stage_list_component(props: &StageListProps) -> Html {
    let product_id_uuid = Uuid::parse_str(&props.product_id).unwrap_or_default();

    let stages_handle = use_async(async move {
        list_stages_for_product(product_id_uuid).await
    });

    use_effect_with((), move |_| {
        stages_handle.run();
        || ()
    });

    html! {
        <div class="container">
            <div class="level">
                <div class="level-left">
                    <h1 class="title level-item">{ "Production Stages" }</h1>
                </div>
                <div class="level-right">
                    <Link<SupplyChainRoute>
                        to={SupplyChainRoute::CreateStage { product_id: props.product_id.clone() }}
                        classes="button is-primary level-item"
                    >
                        { "Add New Stage" }
                    </Link<SupplyChainRoute>>
                </div>
            </div>
            
            {
                if stages_handle.loading {
                    html! { <p>{ "Loading..." }</p> }
                } else if let Some(error) = &stages_handle.error {
                    html! { <p class="has-text-danger">{ format!("Error: {}", error) }</p> }
                } else if let Some(stages) = &stages_handle.data {
                    if stages.is_empty() {
                        html! { <p>{ "No production stages found for this product." }</p> }
                    } else {
                        html! {
                            <div class="list">
                                { for stages.iter().map(|stage| html! {
                                    <div class="list-item card">
                                        <div class="card-content">
                                            <div class="level">
                                                <div class="level-left">
                                                    <p class="title is-4">{ &stage.name }</p>
                                                </div>
                                                <div class="level-right">
                                                    <Link<SupplyChainRoute>
                                                        to={SupplyChainRoute::EditStage { stage_id: stage.id.to_string() }}
                                                        classes="button is-small is-info"
                                                    >
                                                        { "Edit" }
                                                    </Link<SupplyChainRoute>>
                                                </div>
                                            </div>
                                            <p class="subtitle is-6">{ &stage.location }</p>
                                            {if let Some(desc) = &stage.description {
                                                html! { <p>{ desc }</p> }
                                            } else {
                                                html!{}
                                            }}
                                            <div class="content">
                                                <p>
                                                    <strong>{ "Dates: " }</strong>
                                                    { &stage.start_date }
                                                    { " - " }
                                                    { &stage.end_date }
                                                </p>
                                            </div>
                                        </div>
                                    </div>
                                }) }
                            </div>
                        }
                    }
                } else {
                    html! { <p>{ "No data available." }</p> }
                }
            }
        </div>
    }
}