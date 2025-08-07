//! Machine Learning Feature Pipeline
//!
//! A pipeline for preparing data for machine learning models.
//! This module demonstrates how to compose Shtairir standard library blocks into an ML preprocessing workflow.

pub mod pipeline;
pub mod features;

// Re-export key types and functions
pub use pipeline::{MLPipelineMetrics, DataQuality, execute_pipeline, generate_mock_data, compute_dataset_stats};
pub use features::{FeatureEngineeringResult, add_polynomial_features, add_interaction_features, normalize_features, compute_feature_means, compute_feature_stds, create_train_test_split};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}