//! Test suite for the cooperative fundraising system

#[cfg(test)]
mod domain_tests;

#[cfg(test)]
mod integration_tests;

#[cfg(test)]
mod build_test;

// Expose common test setup utilities
#[cfg(test)]
pub mod test_setup;