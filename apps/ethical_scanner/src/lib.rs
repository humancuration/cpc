//! EthicalScanner - Combines product health scoring with supply chain transparency

pub mod scanner;         // Barcode/QR recognition
pub mod health_engine;   // Nutrition scoring algorithms
pub mod supply_chain;    // Ethical scoring and tracking
pub mod suggestions;     // Alternative recommendations
pub mod data_models;     // Core domain structures
pub mod api;             // GraphQL and gRPC interfaces