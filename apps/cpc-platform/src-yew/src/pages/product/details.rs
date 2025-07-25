use yew::prelude::*;
use yew_router::prelude::*;
use crate::types::Product;
use crate::store::StoreContext;
use crate::graphql::{ProductById, ProductPricingHistory, SupplyChainByProduct};
use crate::graphql::GraphQLResponse;
use crate::components::business::CostBreakdown;
use crate::components::charts::PriceHistoryChart;
use crate::components::product::SupplyChainDisplay;

#[derive(Properties, PartialEq, Clone)]
pub struct ProductDetailsProps {
    pub id: String,
}

#[function_component(ProductDetails)]
pub fn product_details(props: &ProductDetailsProps) -> Html {
    let store = use_context::<StoreContext>().unwrap();
    let product = use_state(|| None);
    let pricing_history = use_state(|| None);
    let supply_chain = use_state(|| None);
    let loading = use_state(|| true);
    let history_loading = use_state(|| true);
    let supply_chain_loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let history_error = use_state(|| None::<String>);
    let supply_chain_error = use_state(|| None::<String>);

    // Fetch product details
    use_effect_with_deps({
        let id = props.id.clone();
        let store = store.clone();
        let product = product.clone();
        let loading = loading.clone();
        let error = error.clone();
        
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                error.set(None);

                match store.graphql_client.query::<_, GraphQLResponse<Product>>(ProductById::Variables {
                    id: id.clone(),
                }).await {
                    Ok(response) => {
                        if let Some(data) = response.data {
                            product.set(Some(data));
                        } else if let Some(errors) = response.errors {
                            error.set(Some(errors[0].message.clone()));
                        }
                    }
                    Err(e) => {
                        error.set(Some(e.to_string()));
                    }
                }
                loading.set(false);
            });
            || {}
        }
    }, (props.id.clone(),));

    // Fetch pricing history
    use_effect_with_deps({
        let id = props.id.clone();
        let store = store.clone();
        let pricing_history = pricing_history.clone();
        let history_loading = history_loading.clone();
        let history_error = history_error.clone();
        
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                history_loading.set(true);
                history_error.set(None);

                match store.graphql_client.query::<_, GraphQLResponse<Vec<(String, f64, f64)>>>(ProductPricingHistory::Variables {
                    id: id.clone(),
                }).await {
                    Ok(response) => {
                        if let Some(data) = response.data {
                            pricing_history.set(Some(data));
                        } else if let Some(errors) = response.errors {
                            history_error.set(Some(errors[0].message.clone()));
                        }
                    }
                    Err(e) => {
                        history_error.set(Some(e.to_string()));
                    }
                }
                history_loading.set(false);
            });
            || {}
        }
    }, (props.id.clone(),));

    // Fetch supply chain data
    use_effect_with_deps({
        let id = props.id.clone();
        let store = store.clone();
        let supply_chain = supply_chain.clone();
        let supply_chain_loading = supply_chain_loading.clone();
        let supply_chain_error = supply_chain_error.clone();
        
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                supply_chain_loading.set(true);
                supply_chain_error.set(None);

                match store.graphql_client.query::<_, GraphQLResponse<cpc_core::supply_chain::models::SupplyChain>>(SupplyChainByProduct::Variables {
                    productId: id.clone(),
                }).await {
                    Ok(response) => {
                        if let Some(data) = response.data {
                            supply_chain.set(Some(data));
                        } else if let Some(errors) = response.errors {
                            supply_chain_error.set(Some(errors[0].message.clone()));
                        }
                    }
                    Err(e) => {
                        supply_chain_error.set(Some(e.to_string()));
                    }
                }
                supply_chain_loading.set(false);
            });
            || {}
        }
    }, (props.id.clone(),));

    html! {
        <div class="product-details">
            { if *loading {
                html! { <div class="loading-indicator">{"Loading product details..."}</div> }
            } else if let Some(err) = &*error {
                html! { <div class="error">{"Product Error: "}{err}</div> }
            } else if let Some(product) = &*product {
                html! {
                    <>
                        <div class="product-header">
                            <h1>{&product.name}</h1>
                            <h2>{&product.brand}</h2>
                        </div>
                        
                        <div class="product-images">
                            <div class="image-placeholder">{"Product Image"}</div>
                        </div>
                        
                            <div class="product-info">
                            <p>{&product.description}</p>
                            <p><strong>{"Barcode: "}</strong>{&product.barcode}</p>
                            
                            <div class="sustainability">
                                <h3>{"Sustainability"}</h3>
                                <p><strong>{"Carbon Footprint: "}</strong>{product.carbon_footprint}</p>
                                <p><strong>{"Packaging: "}</strong>{&product.packaging_type}</p>
                            </div>
                            
                            <div class="supply-chain-section">
                                <h3>{"Supply Chain"}</h3>
                                { if *supply_chain_loading {
                                    html! { <div>{"Loading supply chain data..."}</div> }
                                } else if let Some(err) = &*supply_chain_error {
                                    html! { <div class="error">{"Supply Chain Error: "}{err}</div> }
                                } else if let Some(chain) = &*supply_chain {
                                    html! { <SupplyChainDisplay supply_chain={chain.clone()} /> }
                                } else {
                                    html! { <div>{"No supply chain data available"}</div> }
                                }}
                            </div>
                            
                            <div class="business-intel">
                                <h3>{"Business Intelligence"}</h3>
                                
                                <CostBreakdown
                                    material_cost={product.material_cost}
                                    labor_cost={product.labor_cost}
                                    price={product.price}
                                />
                                
                                <div class="pricing-history">
                                    <h4>{"Pricing History"}</h4>
                                    { if *history_loading {
                                        html! { <div>{"Loading pricing history..."}</div> }
                                    } else if let Some(err) = &*history_error {
                                        html! { <div class="error">{"Pricing Error: "}{err}</div> }
                                    } else if let Some(history) = &*pricing_history {
                                        html! { <PriceHistoryChart data={history.clone()} /> }
                                    } else {
                                        html! { <div>{"No pricing data available"}</div> }
                                    }}
                                </div>
                                
                                <div class="inventory">
                                    <h4>{"Inventory Management"}</h4>
                                    <p><strong>{"Current Stock: "}</strong>{product.current_stock}</p>
                                    <p><strong>{"Reorder Level: "}</strong>{product.reorder_level}</p>
                                    <p><strong>{"Supplier: "}</strong>{&product.supplier}</p>
                                    { if product.current_stock <= product.reorder_level {
                                        html! { <div class="alert">{"Reorder needed!"}</div> }
                                    } else {
                                        html! { <div>{"Stock level OK"}</div> }
                                    }}
                                </div>
                            </div>
                            
                            <div class="nutrition">
                                <h3>{"Nutritional Information"}</h3>
                                <p>{&product.nutritional_info}</p>
                            </div>
                            
                            <div class="manufacturer">
                                <h3>{"Manufacturer"}</h3>
                                <p>{&product.manufacturer}</p>
                            </div>
                        </div>
                    </>
                }
            } else {
                html! { <div>{"Product not found"}</div> }
            }}
        </div>
    }
}