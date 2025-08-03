//! Data generation service for creating sample feedback data

use crate::data_generator::config::DataGeneratorConfig;
use crate::data_generator::generators::products::Product;
use crate::data_generator::{generate_reviews, utils::create_sample_reviews};
use reviews::Review;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Debug, Clone)]
pub struct GeneratedData {
    pub reviews: Vec<Review<Product>>,
}

pub struct DataGeneratorService;

impl DataGeneratorService {
    /// Generate sample data based on configuration
    pub fn generate_sample_data(config: DataGeneratorConfig) -> GeneratedData {
        // For the demo, we'll use the sample reviews function
        // In a real implementation, this would use the config to generate data
        let reviews = create_sample_reviews();
        GeneratedData { reviews }
    }

    /// Generate data asynchronously
    pub fn generate_data_async<F>(config: DataGeneratorConfig, callback: F)
    where
        F: FnOnce(Result<GeneratedData, String>) + 'static,
    {
        // In a real implementation, this would be truly async
        // For now, we'll just simulate async behavior
        spawn_local(async move {
            // Simulate some async work
            gloo_timers::future::TimeoutFuture::new(500).await;
            
            let result = std::panic::catch_unwind(|| {
                Self::generate_sample_data(config)
            });
            
            match result {
                Ok(data) => callback(Ok(data)),
                Err(_) => callback(Err("Failed to generate data".to_string())),
            }
        });
    }
}