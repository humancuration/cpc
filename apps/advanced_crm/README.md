# Advanced CRM

Quick Start: Register with Shtairir Core

This module integrates with Shtairir Core via the CrmIntegration adapter found at apps/advanced_crm/src/shtairir_integration.rs.

Bootstrap example (pseudocode; adjust for your host crate):

use std::sync::Arc;
use shtairir_core::AdapterRegistry;
use crate::shtairir_integration::CrmIntegration;
use crate::application::lead_scoring_service::LeadScoringService;
use crate::application::reporting_service::AdvancedReportingService;
use crate::application::email_campaign_service::EmailCampaignService;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Construct services (wire real repos/configs in production)
    let lead_scoring = Arc::new(LeadScoringService::new(/* repo, health */));
    let reporting = Arc::new(AdvancedReportingService::new(/* report_repo, crm_data_access */));
    let email_campaigns = Arc::new(EmailCampaignService::new(/* no external deps currently */));

    // Adapter
    let adapter = Arc::new(CrmIntegration::new(
        Arc::clone(&lead_scoring),
        Arc::clone(&reporting),
        Arc::clone(&email_campaigns),
    ));

    // Registry
    let registry = AdapterRegistry::new();
    registry.register_app(adapter).await?;

    // Optional: inspect schemas/capabilities
    // let schemas = registry.get_app_schemas("advanced_crm").await?;
    // let caps = registry.get_app("advanced_crm").await?.get_capabilities().await?;

    Ok(())
}

Commands exposed
- calculate_lead_score(lead_id: Uuid) -> Number
- create_email_campaign(name: String, subject: String, content: String) -> Object(EmailCampaign)
- sales_report_summary(period: String) -> Object(ReportSummary)

Schemas exposed
- LeadScore
- EmailCampaign
- ReportSummary

Events
- Consumed: UserCreated (from HR), PaymentProcessed (from Finance)
- Planned published events: LeadCreated, CampaignSent, ScoreUpdated

Health and capabilities
- Health: AppIntegrationExt::health_check returns Healthy with checked_at and version
- Capabilities: ["lead_scoring", "email_marketing", "sales_reporting", "event_handling"]
- Dependencies: ["database", "redis"]

Testing tips
- Build test doubles/mocks for:
  - LeadScoringService (e.g., stub calculate_lead_score returning predictable totals)
  - AdvancedReportingService (e.g., stub get_realtime_dashboard_data for predictable KPI)
- EmailCampaignService currently has no external deps in the integration example; can be used directly or stub create_campaign.

Workspace dependency
- This crate expects shtairir_core to be available from the workspace. If you consume this crate outside the workspace, add to Cargo.toml:
  shtairir_core = { workspace = true }
  Or specify a path/version as appropriate.

See also
- apps/advanced_crm/src/shtairir_integration.rs
- shared_packages/shtairir_core/examples/example_app.rs (for AdapterRegistry usage patterns)