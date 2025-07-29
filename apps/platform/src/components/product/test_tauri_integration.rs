use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ValidationUpdate {
    pub product_id: String,
    pub is_valid: bool,
    pub validation_errors: Vec<String>,
    pub last_updated: String,
    pub confidence_score: f64,
}

#[function_component(TestTauriIntegration)]
pub fn test_tauri_integration() -> Html {
    let product_id = use_state(|| "test-product-123".to_string());
    let validation_status = use_state(|| None::<ValidationUpdate>);
    let is_subscribed = use_state(|| false);
    
    // Listen for Tauri events
    use_effect_with_deps(
        |product_id| {
            if cfg!(target_os = "windows") || cfg!(target_os = "macos") || cfg!(target_os = "linux") {
                // Only run in Tauri environment
                spawn_local(async move {
                    // Listen for validation updates
                    // This would use tauri_sys in a real implementation
                });
            }
            || ()
        },
        product_id.clone(),
    );
    
    let on_get_details = {
        let product_id = product_id.clone();
        let validation_status = validation_status.clone();
        
        Callback::from(move |_| {
            let product_id = (*product_id).clone();
            let validation_status = validation_status.clone();
            
            spawn_local(async move {
                #[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
                {
                    use tauri_sys::tauri::invoke;
                    
                    match invoke::<_, ValidationUpdate>(
                        "get_validation_status",
                        &serde_wasm_bindgen::to_value(&product_id).unwrap(),
                    ).await {
                        Ok(status) => validation_status.set(Some(status)),
                        Err(e) => web_sys::console::error_1(&format!("Error: {:?}", e).into()),
                    }
                }
            });
        })
    };
    
    let on_subscribe = {
        let product_id = product_id.clone();
        let is_subscribed = is_subscribed.clone();
        
        Callback::from(move |_| {
            let product_id = (*product_id).clone();
            let is_subscribed = is_subscribed.clone();
            
            spawn_local(async move {
                #[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
                {
                    use tauri_sys::tauri::invoke;
                    
                    match invoke::<_, String>(
                        "subscribe_to_product_validation",
                        &serde_wasm_bindgen::to_value(&product_id).unwrap(),
                    ).await {
                        Ok(_) => is_subscribed.set(true),
                        Err(e) => web_sys::console::error_1(&format!("Error: {:?}", e).into()),
                    }
                }
            });
        })
    };
    
    html! {
        <div class="test-tauri-integration">
            <h3>{"Tauri Integration Test"}</h3>
            
            <div>
                <label>{"Product ID:"}</label>
                <input 
                    type="text" 
                    value={(*product_id).clone()}
                    oninput={Callback::from(|e: InputEvent| {
                        if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                            // This would update the product_id state
                        }
                    })}
                />
            </div>
            
            <div>
                <button onclick={on_get_details}>{"Get Validation Status"}</button>
                <button onclick={on_subscribe}>{"Subscribe to Updates"}</button>
            </div>
            
            if let Some(status) = &*validation_status {
                <div class="validation-status">
                    <h4>{"Current Validation Status"}</h4>
                    <p>{format!("Valid: {}", status.is_valid)}</p>
                    <p>{format!("Confidence: {:.2}%", status.confidence_score * 100.0)}</p>
                    <p>{format!("Updated: {}", status.last_updated)}</p>
                    if !status.validation_errors.is_empty() {
                        <div>
                            <h5>{"Errors:"}</h5>
                            <ul>
                                {status.validation_errors.iter().map(|error| {
                                    html! { <li>{error}</li> }
                                }).collect::<Html>()}
                            </ul>
                        </div>
                    }
                </div>
            }
            
            if *is_subscribed {
                <div class="subscription-status">
                    {"Subscribed to updates for this product"}
                </div>
            }
        </div>
    }
}