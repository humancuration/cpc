use yew::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use js_sys::Promise;

use crate::components::product::{
    ProductHeader,
    ProductDetails,
    CostBreakdown,
    SupplyChainDisplay,
    ValidationStatus,
};
use cpc_core::supply_chain::models::SupplyChain;

#[derive(Properties, PartialEq, Clone)]
pub struct ProductDisplayProps {
    pub product_id: String,
    pub on_back: Callback<()>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductDisplayData {
    pub id: String,
    pub name: String,
    pub description: String,
    pub cost_breakdown: Vec<CostItem>,
    pub total_cost: f64,
    pub profit_margin: f64,
    pub validation_status: String,
    pub image_urls: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostItem {
    pub category: String,
    pub amount: f64,
    pub percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationUpdate {
    pub status: String,
    pub message: String,
    pub timestamp: String,
}

#[function_component(ProductDisplay)]
pub fn product_display(props: &ProductDisplayProps) -> Html {
    let product_id = props.product_id.clone();
    let on_back = props.on_back.clone();
    
    let product_data = use_state(|| None::<ProductDisplayData>);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let validation_updates = use_state(|| vec![]);
    
    // Fetch product data on mount
    {
        let product_data = product_data.clone();
        let supply_chain_data = supply_chain_data.clone();
        let loading = loading.clone();
        let error = error.clone();
        let product_id = product_id.clone();
        
        use_effect_with_deps(move |_| {
            spawn_local(async move {
                // Fetch both product and supply chain data
                let product_future = fetch_product(&product_id);
                let supply_chain_future = fetch_supply_chain(&product_id);
                
                let (product_result, supply_chain_result) = futures::join!(product_future, supply_chain_future);
                
                match product_result {
                    Ok(data) => {
                        product_data.set(Some(data));
                    }
                    Err(err) => {
                        error.set(Some(err));
                    }
                }
                
                match supply_chain_result {
                    Ok(data) => {
                        supply_chain_data.set(Some(data));
                    }
                    Err(err) => {
                        web_sys::console::error_1(&format!("Failed to fetch supply chain: {}", err).into());
                    }
                }
                
                loading.set(false);
            });
            || ()
        }, ());
    }
    
    // Subscribe to validation updates
    {
        let validation_updates = validation_updates.clone();
        let product_id = product_id.clone();
        
        use_effect_with_deps(move |_| {
            let window = window().unwrap();
            let closure = Closure::wrap(Box::new(move |event: web_sys::CustomEvent| {
                if let Ok(update) = serde_wasm_bindgen::from_value::<ValidationUpdate>(event.detail()) {
                    validation_updates.update(|updates| {
                        updates.push(update);
                    });
                }
            }) as Box<dyn FnMut(_)>);
            
            let _ = window.add_event_listener_with_callback(
                "validation-update",
                closure.as_ref().unchecked_ref(),
            );
            
            // Start subscription
            spawn_local(async move {
                let _ = subscribe_validation_updates(&product_id).await;
            });
            
            || ()
        }, ());
    }
    
    // Subscribe to supply chain updates
    {
        let supply_chain_data = supply_chain_data.clone();
        let product_id = product_id.clone();
        
        use_effect_with_deps(move |_| {
            let window = window().unwrap();
            let closure = Closure::wrap(Box::new(move |event: web_sys::CustomEvent| {
                if let Ok(update) = serde_wasm_bindgen::from_value::<SupplyChain>(event.detail()) {
                    supply_chain_data.set(Some(update));
                }
            }) as Box<dyn FnMut(_)>);
            
            let _ = window.add_event_listener_with_callback(
                "supply-chain-update",
                closure.as_ref().unchecked_ref(),
            );
            
            // Start subscription
            spawn_local(async move {
                let _ = subscribe_supply_chain_updates(&product_id).await;
            });
            
            || ()
        }, ());
    }
    
    let loading_component = html! {
        <div class="flex justify-center items-center h-64">
            <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-primary-600"></div>
        </div>
    };
    
    let error_component = |error: String| html! {
        <div class="bg-red-50 border border-red-200 rounded-lg p-6">
            <div class="flex">
                <div class="flex-shrink-0">
                    <svg class="h-5 w-5 text-red-400" viewBox="0 0 20 20" fill="currentColor">
                        <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd" />
                    </svg>
                </div>
                <div class="ml-3">
                    <h3 class="text-sm font-medium text-red-800">{ "Error loading product" }</h3>
                    <div class="mt-2 text-sm text-red-700">
                        <p>{ error }</p>
                    </div>
                </div>
            </div>
        </div>
    };
    
    match (*product_data, *loading) {
        (_, true) => loading_component,
        (None, false) => error_component(error.clone().unwrap_or_else(|| "Unknown error".to_string())),
        (Some(product), false) => {
            html! {
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
                    <div class="mb-6">
                        <button
                            onclick={Callback::from(move |_| on_back.emit(()))}
                            class="flex items-center text-gray-600 hover:text-gray-900 transition-colors"
                        >
                            <svg class="h-5 w-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
                            </svg>
                            { "Back to products" }
                        </button>
                    </div>
                    
                    <div class="space-y-8">
                        <ProductHeader product={product.clone()} />
                        
                        <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
                            <div class="lg:col-span-2 space-y-6">
                                <ProductDetails product={product.clone()} />
                                <CostBreakdown product={product.clone()} />
                                <SupplyChainDisplay supply_chain={supply_chain_data.clone()} />
                            </div>
                            
                            <div class="lg:col-span-1">
                                <ValidationStatus
                                    product={product.clone()}
                                    validation_updates={(*validation_updates).clone()}
                                />
                            </div>
                        </div>
                    </div>
                </div>
            }
        }
    }
}

// Helper functions to call Tauri commands
async fn fetch_product(product_id: &str) -> Result<ProductDisplayData, String> {
    use wasm_bindgen::prelude::*;
    
    #[wasm_bindgen(module = "/src/tauri.ts")]
    extern "C" {
        #[wasm_bindgen(js_name = invoke)]
        fn invoke_tauri(command: &str, args: JsValue) -> Promise;
    }
    
    let args = js_sys::Object::new();
    js_sys::Reflect::set(&args, &"productId".into(), &product_id.into()).unwrap();
    
    let promise = invoke_tauri("get_product", args.into());
    let js_value = wasm_bindgen_futures::JsFuture::from(promise).await
        .map_err(|e| format!("{:?}", e))?;
    
    serde_wasm_bindgen::from_value(js_value)
        .map_err(|e| format!("{:?}", e))
}

async fn subscribe_validation_updates(product_id: &str) -> Result<(), String> {
    use wasm_bindgen::prelude::*;
    
    #[wasm_bindgen(module = "/src/tauri.ts")]
    extern "C" {
        #[wasm_bindgen(js_name = invoke)]
        fn invoke_tauri(command: &str, args: JsValue) -> Promise;
    }
    
    let args = js_sys::Object::new();
    js_sys::Reflect::set(&args, &"productId".into(), &product_id.into()).unwrap();
    
    let promise = invoke_tauri("subscribe_validation_updates", args.into());
    wasm_bindgen_futures::JsFuture::from(promise).await
        .map_err(|e| format!("{:?}", e))?;
    
    Ok(())
}

async fn fetch_supply_chain(product_id: &str) -> Result<SupplyChain, String> {
    use wasm_bindgen::prelude::*;
    
    #[wasm_bindgen(module = "/src/tauri.ts")]
    extern "C" {
        #[wasm_bindgen(js_name = invoke)]
        fn invoke_tauri(command: &str, args: JsValue) -> Promise;
    }
    
    let args = js_sys::Object::new();
    js_sys::Reflect::set(&args, &"productId".into(), &product_id.into()).unwrap();
    
    let promise = invoke_tauri("get_supply_chain", args.into());
    let js_value = wasm_bindgen_futures::JsFuture::from(promise).await
        .map_err(|e| format!("{:?}", e))?;
    
    serde_wasm_bindgen::from_value(js_value)
        .map_err(|e| format!("{:?}", e))
}

async fn subscribe_supply_chain_updates(product_id: &str) -> Result<(), String> {
    use wasm_bindgen::prelude::*;
    
    #[wasm_bindgen(module = "/src/tauri.ts")]
    extern "C" {
        #[wasm_bindgen(js_name = invoke)]
        fn invoke_tauri(command: &str, args: JsValue) -> Promise;
    }
    
    let args = js_sys::Object::new();
    js_sys::Reflect::set(&args, &"productId".into(), &product_id.into()).unwrap();
    
    let promise = invoke_tauri("subscribe_to_supply_chain_updates", args.into());
    wasm_bindgen_futures::JsFuture::from(promise).await
        .map_err(|e| format!("{:?}", e))?;
    
    Ok(())
}