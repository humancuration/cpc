//! Generators for different types of feedback data

pub mod products;
pub mod reviews;
pub mod surveys;
pub mod federation;

pub use reviews::generate_reviews;
pub use surveys::generate_survey_responses;
pub use federation::generate_federated_reviews;