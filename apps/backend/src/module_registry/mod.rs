// Module registry system for dynamic module management
pub mod registry;

#[cfg(test)]
mod registry_test;

#[cfg(test)]
mod dependency_test;

pub use registry::{Module, ModuleRegistry, DependencyRequirement};