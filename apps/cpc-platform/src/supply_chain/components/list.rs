use yew::prelude::*;
use yew_router::prelude::*;
use yew_hooks::use_async;
use log::error;

use crate::supply_chain::routing::SupplyChainRoute;
use cpc_core::supply_chain::models::ProductSummary;

#[function_component(SupplyChainList)]
pub fn supply_chain_list() -> Html {
    let products_query = use_async(async { crate::api::supply_chain::list_products_with_supply_chains().await });

    use_effect_with_deps(
        {
            let products_query = products_query.clone();
            move |_| {
                products_query.run();
                || ()
            }
        },
        (),
    );

    html! {
        <div class="container mx-auto px-4 py-8">
            <h1 class="text-3xl font-bold mb-6">{ "Products with Supply Chains" }</h1>
            {
                if products_query.loading {
                    html! { <p>{ "Loading..." }</p> }
                } else if let Some(error) = &products_query.error {
                    error!("Error loading products: {:?}", error);
                    html! { <p class="text-red-500">{ "Error loading products. Please try again later." }</p> }
                } else if let Some(products) = &products_query.data {
                    html! {
                        <ul class="list-disc pl-5">
                            { for products.iter().map(|product| html! {
                                <li key={product.id.to_string()}>
                                    <Link<SupplyChainRoute> to={SupplyChainRoute::Detail { id: product.id.to_string() }}>
                                        { &product.name }
                                    </Link<SupplyChainRoute>>
                                </li>
                            })}
                        </ul>
                    }
                } else {
                    html! { <p>{ "No products found." }</p> }
                }
            }
        </div>
    }
}