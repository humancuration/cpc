// DEPRECATED - Refactored to use collaboration_engine
// This module has been deprecated as part of the refactor to use the collaboration_engine package.
// The new implementation can be found in the application/collaboration_service.rs file.
pub mod p2p;
pub mod panda_network;
pub mod operation_queue;
pub mod service;
pub mod conversion;
pub mod sync;

#[cfg(feature = "p2p")]
pub mod transport;

#[cfg(test)]
mod service_test;