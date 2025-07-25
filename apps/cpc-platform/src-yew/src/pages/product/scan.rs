use yew::prelude::*;
use yewdux::prelude::*;
use yew_router::prelude::*;
use crate::components::product::barcode_scanner::BarcodeScanner;
use crate::store::product::ProductStore;
use crate::types::error::Error;
use cpc_core::models::product::Product;
use crate::routes::Route;

#[function_component(ProductScanPage)]
pub fn product_scan_page() -> Html {
    let (product_state, dispatch) = use_store::<ProductStore>();
    let navigator = use_navigator().unwrap();
    
    let on_product_found = {
        let dispatch = dispatch.clone();
        let navigator = navigator.clone();
        Callback::from(move |product: Product| {
            dispatch.reduce_mut(|store: &mut ProductStore| {
                store.current_product = Some(product.clone());
                store.error = None;
            });
            // Navigate to product details page
            navigator.push(&Route::ProductDetails { id: product.id });
        })
    };
    
    let on_error = {
        let dispatch = dispatch.clone();
        Callback::from(move |error: Error| {
            dispatch.reduce_mut(|store: &mut ProductStore| {
                store.error = Some(error);
                store.current_product = None;
            });
        })
    };
    
    html! {
        <div class="product-scan-page">
            <h1>{"Scan Product"}</h1>
            
            <BarcodeScanner
                on_product_found={on_product_found}
                on_error={on_error}
            />
            
            {match &product_state.error {
                Some(error) => html! {
                    <div class="error-notification">
                        {format!("Error: {}", error.message)}
                    </div>
                },
                None => html! {}
            }}
        </div>
    }
}