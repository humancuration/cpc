//! Tracing integration for the data lakehouse

use tracing::{info, error, debug, span, Level};
use uuid::Uuid;

/// Trace a data operation
pub fn trace_data_operation(operation: &str, asset_id: Uuid) -> tracing::Span {
    span!(
        Level::INFO,
        "data_operation",
        operation = operation,
        asset_id = %asset_id,
    )
}

/// Log an ingestion job execution
pub fn log_ingestion_job(job_id: Uuid, success: bool, records_processed: u64) {
    if success {
        info!(
            job_id = %job_id,
            records_processed = records_processed,
            "Ingestion job completed successfully"
        );
    } else {
        error!(
            job_id = %job_id,
            records_processed = records_processed,
            "Ingestion job failed"
        );
    }
}

/// Log a data access event
pub fn log_data_access(asset_id: Uuid, user_id: Option<Uuid>, action: &str) {
    debug!(
        asset_id = %asset_id,
        user_id = ?user_id,
        action = action,
        "Data access event"
    );
}