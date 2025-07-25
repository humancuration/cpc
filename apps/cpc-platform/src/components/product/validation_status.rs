use yew::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Properties, PartialEq, Clone)]
pub struct ValidationStatusProps {
    pub product: ProductDisplayData,
    pub validation_updates: Vec<ValidationUpdate>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductDisplayData {
    pub id: String,
    pub name: String,
    pub validation_status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidationUpdate {
    pub status: String,
    pub message: String,
    pub timestamp: String,
}

#[function_component(ValidationStatus)]
pub fn validation_status(props: &ValidationStatusProps) -> Html {
    let product = &props.product;
    let validation = &props.validation_updates;
    
    let validation_state = match product.validation_status.as_str() {
        "valid" => ValidationState::Valid,
        "invalid" => ValidationState::Invalid(vec!["Validation failed".to_string()]),
        _ => ValidationState::Pending,
    };
    
    let (icon, title, description, color_class) = match &validation_state {
        ValidationState::Valid => (
            "✅",
            "Product Validated",
            "All data has been verified and is accurate",
            "bg-green-50 border-green-200 text-green-800",
        ),
        ValidationState::Invalid(_) => (
            "⚠️",
            "Validation Issues",
            "Found validation errors",
            "bg-red-50 border-red-200 text-red-800",
        ),
        ValidationState::Pending => (
            "⏳",
            "Validation Pending",
            "Product data is being validated",
            "bg-yellow-50 border-yellow-200 text-yellow-800",
        ),
    };
    
    html! {
        <div class={`rounded-lg border p-6 ${color_class}`}>
            <div class="flex items-start">
                <div class="flex-shrink-0">
                    <span class="text-2xl">{ icon }</span>
                </div>
                <div class="ml-3">
                    <h3 class="text-sm font-medium">{ title }</h3>
                    <div class="mt-2 text-sm">
                        <p>{ description }</p>
                    </div>
                </div>
            </div>
            
            if !validation_updates.is_empty() {
                <div class="mt-4">
                    <h4 class="text-sm font-medium mb-2">{"Recent Updates"}</h4>
                    <ul class="space-y-2">
                        { for validation_updates.iter().take(3).map(|update| html! {
                            <li class="text-sm">
                                <div class="flex justify-between">
                                    <span>{ &update.message }</span>
                                    <span class="text-gray-500">{ &update.timestamp }</span>
                                </div>
                            </li>
                        })}
                    </ul>
                </div>
            }
            
            <div class="mt-4 pt-4 border-t border-gray-200">
                <div class="mt-3">
                    <button class="w-full bg-white border border-gray-300 rounded-md py-2 px-4 text-sm font-medium text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500">
                        { "Re-validate Product" }
                    </button>
                </div>
            </div>
        </div>
    }
}

#[derive(Debug, Clone, PartialEq)]
enum ValidationState {
    Valid,
    Invalid(Vec<String>),
    Pending,
}