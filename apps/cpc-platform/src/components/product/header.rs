use yew::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Properties, PartialEq, Clone)]
pub struct ProductHeaderProps {
    pub product: ProductData,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductData {
    pub id: String,
    pub name: String,
    pub brand: Option<String>,
    pub barcode: Option<String>,
    pub description: Option<String>,
    pub cost: Option<MoneyData>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MoneyData {
    pub amount: f64,
    pub currency: String,
}

#[function_component(ProductHeader)]
pub fn product_header(props: &ProductHeaderProps) -> Html {
    let product = &props.product;
    
    html! {
        <div class="bg-white shadow rounded-lg overflow-hidden">
            <div class="px-6 py-8">
                <div class="flex items-start justify-between">
                    <div class="flex-1">
                        <div class="flex items-center space-x-3">
                            <h1 class="text-3xl font-bold text-gray-900">{ &product.name }</h1>
                            if let Some(brand) = &product.brand {
                                <span class="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-blue-100 text-blue-800">
                                    { brand }
                                </span>
                            }
                        </div>
                        
                        if let Some(description) = &product.description {
                            <p class="mt-3 text-lg text-gray-600">{ description }</p>
                        }
                        
                        <div class="mt-4 flex items-center space-x-6 text-sm text-gray-500">
                            if let Some(barcode) = &product.barcode {
                                <div class="flex items-center">
                                    <svg class="h-5 w-5 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 4a1 1 0 011-1h16a1 1 0 011 1v2.586a1 1 0 01-.293.707l-6.414 6.414a1 1 0 00-.293.707V17l-4 4v-6.586a1 1 0 00-.293-.707L3.293 7.293A1 1 0 013 6.586V4z" />
                                    </svg>
                                    { format!("Barcode: {}", barcode) }
                                </div>
                            }
                            
                            if let Some(cost) = &product.cost {
                                <div class="flex items-center">
                                    <svg class="h-5 w-5 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                                    </svg>
                                    { format!("${:.2} {}", cost.amount, cost.currency) }
                                </div>
                            }
                        </div>
                    </div>
                    
                    <div class="ml-4 flex-shrink-0">
                        <div class="w-24 h-24 bg-gray-200 rounded-lg flex items-center justify-center">
                            <svg class="w-12 h-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
                            </svg>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}