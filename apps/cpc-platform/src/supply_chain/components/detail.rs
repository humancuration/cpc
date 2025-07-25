use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::api::supply_chain::{get_supply_chain_by_product, get_supply_chain_details};

type SupplyChain = get_supply_chain_by_product::GetSupplyChainByProductGetSupplyChainByProduct;

#[derive(Properties, Clone, PartialEq)]
pub struct SupplyChainDetailProps {
    pub id: String,
}

#[function_component(SupplyChainDetail)]
pub fn supply_chain_detail(props: &SupplyChainDetailProps) -> Html {
    let supply_chain = use_state(|| None::<SupplyChain>);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    {
        let supply_chain = supply_chain.clone();
        let loading = loading.clone();
        let error = error.clone();
        let product_id = props.id.clone();

        use_effect_with(product_id.clone(), move |_| {
            spawn_local(async move {
                match get_supply_chain_details(product_id).await {
                    Ok(data) => {
                        supply_chain.set(Some(data));
                    }
                    Err(e) => {
                        error.set(Some(e));
                    }
                }
                loading.set(false);
            });
            || ()
        });
    }

    html! {
        <div class="container mx-auto px-4 py-8 max-w-4xl">
            if *loading {
                <p>{ "Loading..." }</p>
            } else if let Some(err) = &*error {
                <p class="text-red-500">{ format!("Error: {}", err) }</p>
            } else if let Some(sc) = &*supply_chain {
                <div>
                    <h1 class="text-3xl font-bold mb-4">{ format!("Supply Chain for Product {}", &props.id) }</h1>
                    
                    <div class="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4">
                        <h2 class="text-2xl font-bold mb-2">{"Cooperative Impact"}</h2>
                        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                            <div><p class="font-bold">{"Workers Benefited:"}</p><p>{ sc.cooperative_impact.workers_benefited }</p></div>
                            <div><p class="font-bold">{"Co-ops Involved:"}</p><p>{ sc.cooperative_impact.coops_involved }</p></div>
                            <div><p class="font-bold">{"Ethical Sourcing Score:"}</p><p>{ sc.cooperative_impact.ethical_sourcing_score }</p></div>
                        </div>
                    </div>

                    <div class="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4">
                        <h2 class="text-2xl font-bold mb-2">{"Production Stages"}</h2>
                        { for sc.stages.iter().map(|stage| html! {
                            <div class="mb-4 p-4 border rounded">
                                <h3 class="font-bold text-xl">{ &stage.name }</h3>
                                if let Some(desc) = &stage.description {
                                    <p>{ desc }</p>
                                }
                                <p><b>{"Location:"}</b>{ &stage.location }</p>
                                <p><b>{"From:"}</b>{ &stage.start_date } <b>{"To:"}</b>{ &stage.end_date }</p>
                            </div>
                        })}
                    </div>
                </div>
            } else {
                 <p>{ "Supply chain data not found." }</p>
            }
        </div>
    }
}