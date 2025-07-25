// packages/cpc-core/src/financial_forecasting/service.rs
use super::models::{ForecastScenario, ForecastResult, FinancialStatement};
use super::algorithms;
use anyhow::Result;
use tokio::sync::broadcast;
use std::sync::Arc;

/// A port for fetching accounting data from an external source.
/// This allows us to decouple the core logic from any specific accounting software or database.
pub trait AccountingAdapter: Send + Sync {
    fn get_latest_financials(&self) -> Result<FinancialStatement>;
}

/// The main service for the financial forecasting feature.
/// It orchestrates data flow between the API layer and the core algorithms.
pub struct FinancialForecastingService {
    // For long-running jobs, we notify on completion using a broadcast channel.
    // This supports multiple concurrent subscribers.
    result_notifier: broadcast::Sender<Arc<ForecastResult>>,
    // The service can be configured with an adapter to a real accounting system.
    accounting_adapter: Option<Arc<dyn AccountingAdapter>>,
}

impl FinancialForecastingService {
    pub fn new() -> Self {
        let (tx, _rx) = broadcast::channel(100);
        Self { result_notifier: tx, accounting_adapter: None }
    }

    /// Runs a new forecast scenario asynchronously.
    ///
    /// This function accepts a scenario, spawns a new Tokio task to process it,
    /// and upon completion, broadcasts the result to all active subscribers.
    /// This non-blocking approach is crucial for a responsive system.
    pub async fn run_forecast(&self, scenario: ForecastScenario) {
        let notifier = self.result_notifier.clone();
        tokio::spawn(async move {
            let result = algorithms::project_profit_and_loss(&scenario);
            // The result is broadcast to any active subscribers (e.g., GraphQL subscriptions).
            // We wrap it in an Arc for efficient sharing across threads.
            let _ = notifier.send(Arc::new(result));
        });
    }

    /// Subscribes to the results of completed forecast jobs.
    ///
    /// This allows a caller (like a GraphQL subscription handler) to receive
    /// `ForecastResult` objects as they are produced.
    pub fn get_result_stream(&self) -> broadcast::Receiver<Arc<ForecastResult>> {
        self.result_notifier.subscribe()
    }
}