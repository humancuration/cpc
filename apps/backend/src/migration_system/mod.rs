// Modular migration system for dynamic module management
pub mod system;

#[cfg(test)]
mod system_test;

pub use system::MigrationSystem;