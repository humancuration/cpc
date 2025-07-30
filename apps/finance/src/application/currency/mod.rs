//! Currency application module
//!
//! This module provides application-level services for currency management,
//! including conversion, formatting, and user preferences.

pub mod service;
pub mod user_prefs;
pub mod mock_repo;

pub use service::{CurrencyService, CurrencyServiceError};
pub use user_prefs::{UserCurrencyPreferences, UserCurrencyPreferencesRepository, CurrencyPreferencesError};
pub use mock_repo::MockCurrencyPreferencesRepository;