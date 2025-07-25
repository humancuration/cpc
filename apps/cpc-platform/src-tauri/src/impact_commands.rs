use tauri::{Builder, State, Wry};
use uuid::Uuid;
use anyhow::Result;
use cpc_core::impact::OrganizationImpactReport;
use crate::AppState;

#[tauri::command]
pub async fn get_organization_impact_report(
    org_id: String,
    year: i32,
    state: State<'_, AppState>
) -> Result<OrganizationImpactReport, String> {
    let uuid = Uuid::parse_str(&org_id)
        .map_err(|e| format!("Failed to parse organization ID: {}", e))?;

    let report = state.impact_service.get_organization_impact_report(uuid, year)
        .await
        .map_err(|e| format!("Failed to get impact report: {}", e))?
        .ok_or_else(|| "Impact report not found".to_string())?;

    Ok(OrganizationImpactReport {
        organization_id: report.organization_id,
        year: report.year,
        generated_at: report.generated_at,
        carbon_footprint: report.carbon_footprint,
        community_investment: report.community_investment,
        diversity_metrics: cpc_core::impact::DiversityMetrics {
            gender_diversity: report.gender_diversity,
            ethnic_diversity: report.ethnic_diversity,
        },
        supply_chain_score: report.supply_chain_score,
    })
}

/// Register impact commands with Tauri
pub fn register_commands(builder: Builder<Wry>) -> Builder<Wry> {
    builder.invoke_handler(tauri::generate_handler![
        get_organization_impact_report
    ])
}