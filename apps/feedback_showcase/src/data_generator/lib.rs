//! Data generator module for the feedback showcase
//!
//! This module provides functionality to generate realistic sample data for
//! product reviews, survey responses, and federated reviews.

pub mod config;
pub mod generators;
pub mod utils;

#[cfg(test)]
mod tests;

pub use config::DataGeneratorConfig;
pub use generators::{generate_reviews, generate_survey_responses, generate_federated_reviews};