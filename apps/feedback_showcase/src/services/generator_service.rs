//! Service for generating feedback data

use crate::data_generator::config::DataGeneratorConfig;
use crate::data_generator::generators::products::Product;
use crate::data_generator::{generate_reviews, generate_survey_responses, generate_federated_reviews};
use reviews::{Review, Entity};
use survey::Survey;
use uuid::Uuid;
use web_sys::window;
use wasm_bindgen::JsValue;
use web_sys::{Blob, Url, HtmlAnchorElement};
use js_sys::{JsString, Uint8Array};
use chrono::Utc;

#[derive(Debug, Clone, Default)]
pub struct GenerationMetrics {
    pub items_processed: usize,
    pub items_per_second: f64,
    pub memory_usage: usize, // in MB
    pub progress: u32,       // 0-100
}

pub struct GeneratorService;

impl GeneratorService {
    pub async fn generate_data(config: DataGeneratorConfig) -> Result<Vec<Review<Product>>, String> {
        // In a real implementation, this would call the actual generator functions
        // For now, we'll simulate the generation process
        
        // Generate sample product from the first product type
        let product_type = &config.product_types[0]; // Use first product type
        let product = crate::data_generator::generators::products::generate_product(product_type);
        
        // Generate sample product reviews with attributes
        let reviews = generate_reviews(&config, product.clone());
        
        // In a real implementation, we would also generate survey responses and federated reviews
        // let survey = create_sample_survey().map_err(|e| e.to_string())?;
        // let survey_responses = generate_survey_responses(&config, &survey, &mut reviews);
        // let federated_reviews = generate_federated_reviews(reviews.clone());
        
    }
    
    pub fn export_data(reviews: &[Review<Product>]) {
        match serde_json::to_string_pretty(reviews) {
            Ok(json_data) => {
                if let Some(window) = window() {
                    // Create blob
                    let array = Uint8Array::new_with_length(json_data.len() as u32);
                    array.copy_from(json_data.as_bytes());
                    let blob_parts = js_sys::Array::new();
                    blob_parts.push(&array.buffer());
                    
                    let blob = Blob::new_with_u8_array_sequence_and_options(
                        &blob_parts,
                        web_sys::BlobPropertyBag::new().type_("application/json")
                    ).expect("Failed to create blob");
                    
                    // Create download URL
                    let url = Url::create_object_url_with_blob(&blob).expect("Failed to create URL");
                    
                    // Create download link
                    let document = window.document().expect("No document found");
                    let a = document.create_element("a").expect("Failed to create element");
                    a.set_attribute("href", &url).expect("Failed to set href");
                    
                    // Generate filename with timestamp
                    let timestamp = Utc::now().format("%Y%m%d_%H%M%S").to_string();
                    let filename = format!("feedback_data_{}.json", timestamp);
                    a.set_attribute("download", &filename).expect("Failed to set download");
                    
                    // Trigger download
                    document.body().expect("No body found").append_child(&a).expect("Failed to append");
                    a.dyn_ref::<HtmlAnchorElement>()
                        .expect("Not an anchor element")
                        .click();
                    document.body().expect("No body found").remove_child(&a).expect("Failed to remove");
                    
                    // Clean up
                    Url::revoke_object_url(&url).expect("Failed to revoke URL");
                }
            }
            Err(e) => {
                web_sys::console::error_1(&format!("Error serializing data: {}", e).into());
            }
        }
    }
}

/// Create a sample survey (helper function)
fn create_sample_survey() -> Result<Survey, Box<dyn std::error::Error>> {
    let survey = Survey {
        id: Uuid::new_v4(),
        title: "Product Feedback Survey".to_string(),
        description: "Help us improve our eco-friendly water bottle".to_string(),
        questions: vec![], // Simplified for now
        scoring_config: None,
    };
    
    Ok(survey)
}