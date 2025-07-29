use serde::{Deserialize, Serialize};
use uuid::Uuid;
use cpc_core::accounting::{
    dashboard::{AccountingDashboard, Metric},
    PeriodType,
};
use tauri::command;

/// Tauri command to get accounting dashboard data
#[command]
pub async fn get_accounting_dashboard(
    org_id: Uuid,
    period: PeriodType,
) -> Result<AccountingDashboard, String> {
    match cpc_core::accounting::dashboard::get_dashboard_data(org_id, period).await {
        Ok(dashboard) => Ok(dashboard),
        Err(e) => Err(format!("Failed to get dashboard data: {}", e)),
    }
}

/// Tauri command to refresh dashboard data
#[command]
pub async fn refresh_accounting_dashboard(
    org_id: Uuid,
    period: PeriodType,
) -> Result<AccountingDashboard, String> {
    get_accounting_dashboard(org_id, period).await
}