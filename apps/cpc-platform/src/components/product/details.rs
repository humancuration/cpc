use yew::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Properties, PartialEq, Clone)]
pub struct ProductDetailsProps {
    pub product: ProductData,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductData {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub manufacturer: Option<String>,
    pub nutritional_info: Option<String>,
    pub packaging_type: Option<String>,
    pub current_stock: Option<u32>,
    pub reorder_level: Option<u32>,
    pub supplier: Option<String>,
    pub location: Option<WarehouseLocationData>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WarehouseLocationData {
    pub id: String,
    pub name: String,
}

#[function_component(ProductDetails)]
pub fn product_details(props: &ProductDetailsProps) -> Html {
    let product = &props.product;
    
    html! {
        <div class="bg-white shadow rounded-lg">
            <div class="px-6 py-4 border-b border-gray-200">
                <h2 class="text-lg font-medium text-gray-900">{ "Product Details" }</h2>
            </div>
            
            <div class="px-6 py-4 space-y-6">
                // Basic Information
                <div>
                    <h3 class="text-sm font-medium text-gray-900 mb-3">{ "Basic Information" }</h3>
                    <dl class="grid grid-cols-1 gap-x-4 gap-y-6 sm:grid-cols-2">
                        if let Some(manufacturer) = &product.manufacturer {
                            <div>
                                <dt class="text-sm font-medium text-gray-500">{ "Manufacturer" }</dt>
                                <dd class="mt-1 text-sm text-gray-900">{ manufacturer }</dd>
                            </div>
                        }
                        
                        if let Some(supplier) = &product.supplier {
                            <div>
                                <dt class="text-sm font-medium text-gray-500">{ "Supplier" }</dt>
                                <dd class="mt-1 text-sm text-gray-900">{ supplier }</dd>
                            </div>
                        }
                        
                        if let Some(packaging) = &product.packaging_type {
                            <div>
                                <dt class="text-sm font-medium text-gray-500">{ "Packaging Type" }</dt>
                                <dd class="mt-1 text-sm text-gray-900">{ packaging }</dd>
                            </div>
                        }
                        
                        if let Some(location) = &product.location {
                            <div>
                                <dt class="text-sm font-medium text-gray-500">{ "Warehouse Location" }</dt>
                                <dd class="mt-1 text-sm text-gray-900">{ &location.name }</dd>
                            </div>
                        }
                    </dl>
                </div>
                
                // Stock Information
                <div>
                    <h3 class="text-sm font-medium text-gray-900 mb-3">{ "Inventory" }</h3>
                    <div class="grid grid-cols-1 gap-x-4 gap-y-6 sm:grid-cols-2">
                        if let Some(stock) = product.current_stock {
                            <div class="flex items-center">
                                <div class="flex-1">
                                    <dt class="text-sm font-medium text-gray-500">{ "Current Stock" }</dt>
                                    <dd class="mt-1 text-sm text-gray-900">{ stock }</dd>
                                </div>
                                <div class="ml-4">
                                    <span class={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${
                                        if stock == 0 {
                                            "bg-red-100 text-red-800"
                                        } else if stock <= product.reorder_level.unwrap_or(0) {
                                            "bg-yellow-100 text-yellow-800"
                                        } else {
                                            "bg-green-100 text-green-800"
                                        }
                                    }`}>
                                        { 
                                            if stock == 0 {
                                                "Out of Stock"
                                            } else if stock <= product.reorder_level.unwrap_or(0) {
                                                "Low Stock"
                                            } else {
                                                "In Stock"
                                            }
                                        }
                                    </span>
                                </div>
                            </div>
                        }
                        
                        if let Some(reorder) = product.reorder_level {
                            <div>
                                <dt class="text-sm font-medium text-gray-500">{ "Reorder Level" }</dt>
                                <dd class="mt-1 text-sm text-gray-900">{ reorder }</dd>
                            </div>
                        }
                    </div>
                </div>
                
                // Nutritional Information
                if let Some(nutritional) = &product.nutritional_info {
                    <div>
                        <h3 class="text-sm font-medium text-gray-900 mb-3">{ "Nutritional Information" }</h3>
                        <div class="bg-gray-50 rounded-md p-4">
                            <pre class="text-sm text-gray-700 whitespace-pre-wrap">{ nutritional }</pre>
                        </div>
                    </div>
                }
                
                // Description
                if let Some(description) = &product.description {
                    <div>
                        <h3 class="text-sm font-medium text-gray-900 mb-3">{ "Description" }</h3>
                        <p class="text-sm text-gray-700">{ description }</p>
                    </div>
                }
            </div>
        </div>
    }
}