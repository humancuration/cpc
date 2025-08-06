//! Shtairir Core integration for Advanced CRM
//!
//! Exposes CRM commands, schemas, and event handling to the Shtairir runtime.

use async_trait::async_trait;
use chrono::Utc;
use shtairir_core::{
    AppIntegration, AppIntegrationExt, CommandDefinition, DataSchema, Event, FieldDefinition,
    HealthCheckResult, HealthStatus, ParameterDefinition, ShtairirError, ShtairirResult,
    ShtairirType, ShtairirValue,
};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

// Domain and application imports
use crate::application::lead_scoring_service::LeadScoringService;
use crate::application::reporting_service::{ReportSummary, AdvancedReportingService};
use crate::application::email_campaign_service::EmailCampaignService;
use crate::domain::email_campaign::EmailCampaign;

/// CRM Shtairir integration adapter
pub struct CrmIntegration {
    lead_scoring: Arc<LeadScoringService>,
    reporting: Arc<AdvancedReportingService>,
    email_campaigns: Arc<EmailCampaignService>,
}

impl CrmIntegration {
    /// Create a new CrmIntegration
    ///
    /// Note: Wire real repository/service dependencies from the app bootstrapper.
    pub fn new(
        lead_scoring: Arc<LeadScoringService>,
        reporting: Arc<AdvancedReportingService>,
        email_campaigns: Arc<EmailCampaignService>,
    ) -> Self {
        Self {
            lead_scoring,
            reporting,
            email_campaigns,
        }
    }

    fn app_str() -> &'static str {
        "advanced_crm"
    }

    fn lead_score_schema() -> DataSchema {
        let mut fields = HashMap::new();
        fields.insert(
            "lead_id".to_string(),
            FieldDefinition {
                name: "lead_id".to_string(),
                field_type: ShtairirType::Uuid,
                required: true,
                default_value: None,
                description: Some("ID of the lead".to_string()),
                validation: None,
            },
        );
        fields.insert(
            "total_score".to_string(),
            FieldDefinition {
                name: "total_score".to_string(),
                field_type: ShtairirType::Number,
                required: true,
                default_value: None,
                description: Some("Aggregated score 0-100".to_string()),
                validation: Some(vec![
                    shtairir_core::ValidationRule::MinNumber(0.0),
                    shtairir_core::ValidationRule::MaxNumber(100.0),
                ]),
            },
        );
        DataSchema {
            name: "LeadScore".to_string(),
            version: "1.0.0".to_string(),
            fields,
            description: Some("Lead scoring schema".to_string()),
            metadata: HashMap::new(),
        }
    }

    fn email_campaign_schema() -> DataSchema {
        let mut fields = HashMap::new();
        fields.insert(
            "id".to_string(),
            FieldDefinition {
                name: "id".to_string(),
                field_type: ShtairirType::Uuid,
                required: true,
                default_value: None,
                description: Some("Campaign ID".to_string()),
                validation: None,
            },
        );
        fields.insert(
            "name".to_string(),
            FieldDefinition {
                name: "name".to_string(),
                field_type: ShtairirType::String,
                required: true,
                default_value: None,
                description: Some("Campaign name".to_string()),
                validation: Some(vec![
                    shtairir_core::ValidationRule::MinLength(1),
                    shtairir_core::ValidationRule::MaxLength(200),
                ]),
            },
        );
        fields.insert(
            "subject".to_string(),
            FieldDefinition {
                name: "subject".to_string(),
                field_type: ShtairirType::String,
                required: true,
                default_value: None,
                description: Some("Email subject".to_string()),
                validation: Some(vec![
                    shtairir_core::ValidationRule::MinLength(1),
                    shtairir_core::ValidationRule::MaxLength(300),
                ]),
            },
        );
        fields.insert(
            "content".to_string(),
            FieldDefinition {
                name: "content".to_string(),
                field_type: ShtairirType::String,
                required: true,
                default_value: None,
                description: Some("Email content HTML/Markdown".to_string()),
                validation: Some(vec![shtairir_core::ValidationRule::MinLength(1)]),
            },
        );
        DataSchema {
            name: "EmailCampaign".to_string(),
            version: "1.0.0".to_string(),
            fields,
            description: Some("Email campaign schema".to_string()),
            metadata: HashMap::new(),
        }
    }

    fn report_summary_schema() -> DataSchema {
        let mut fields = HashMap::new();
        fields.insert(
            "total_deals".to_string(),
            FieldDefinition {
                name: "total_deals".to_string(),
                field_type: ShtairirType::Number,
                required: true,
                default_value: None,
                description: Some("Total deals".to_string()),
                validation: None,
            },
        );
        fields.insert(
            "total_value".to_string(),
            FieldDefinition {
                name: "total_value".to_string(),
                field_type: ShtairirType::Number,
                required: true,
                default_value: None,
                description: Some("Total value in cents".to_string()),
                validation: None,
            },
        );
        fields.insert(
            "average_deal_size".to_string(),
            FieldDefinition {
                name: "average_deal_size".to_string(),
                field_type: ShtairirType::Number,
                required: true,
                default_value: None,
                description: Some("Average deal size in cents".to_string()),
                validation: None,
            },
        );
        fields.insert(
            "win_rate".to_string(),
            FieldDefinition {
                name: "win_rate".to_string(),
                field_type: ShtairirType::Number,
                required: true,
                default_value: None,
                description: Some("Win rate 0.0-1.0".to_string()),
                validation: None,
            },
        );
        fields.insert(
            "conversion_rate".to_string(),
            FieldDefinition {
                name: "conversion_rate".to_string(),
                field_type: ShtairirType::Number,
                required: true,
                default_value: None,
                description: Some("Conversion rate 0.0-1.0".to_string()),
                validation: None,
            },
        );
        DataSchema {
            name: "ReportSummary".to_string(),
            version: "1.0.0".to_string(),
            fields,
            description: Some("Sales report summary schema".to_string()),
            metadata: HashMap::new(),
        }
    }
}

#[async_trait]
impl AppIntegration for CrmIntegration {
    fn app_name(&self) -> &str {
        Self::app_str()
    }

    fn app_version(&self) -> &str {
        env!("CARGO_PKG_VERSION")
    }

    async fn initialize(&self) -> ShtairirResult<()> {
        // If any async initialization or warm-up is needed, do here.
        Ok(())
    }

    async fn shutdown(&self) -> ShtairirResult<()> {
        // Cleanup if necessary
        Ok(())
    }

    async fn get_commands(&self) -> ShtairirResult<Vec<CommandDefinition>> {
        Ok(vec![
            CommandDefinition {
                name: "calculate_lead_score".to_string(),
                app: self.app_name().to_string(),
                description: "Calculate lead score for a lead".to_string(),
                parameters: vec![ParameterDefinition {
                    name: "lead_id".to_string(),
                    param_type: ShtairirType::Uuid,
                    required: true,
                    default_value: None,
                    description: Some("ID of the lead".to_string()),
                }],
                return_type: ShtairirType::Number,
                category: "lead_scoring".to_string(),
                is_async: true,
                version: "1.0.0".to_string(),
                example: Some("calculate_lead_score(lead_id=\"00000000-0000-0000-0000-000000000000\")".to_string()),
            },
            CommandDefinition {
                name: "create_email_campaign".to_string(),
                app: self.app_name().to_string(),
                description: "Create a new email campaign".to_string(),
                parameters: vec![
                    ParameterDefinition {
                        name: "name".to_string(),
                        param_type: ShtairirType::String,
                        required: true,
                        default_value: None,
                        description: Some("Campaign name".to_string()),
                    },
                    ParameterDefinition {
                        name: "subject".to_string(),
                        param_type: ShtairirType::String,
                        required: true,
                        default_value: None,
                        description: Some("Email subject".to_string()),
                    },
                    ParameterDefinition {
                        name: "content".to_string(),
                        param_type: ShtairirType::String,
                        required: true,
                        default_value: None,
                        description: Some("Email content".to_string()),
                    },
                ],
                return_type: ShtairirType::Object(Some("EmailCampaign".to_string())),
                category: "email_marketing".to_string(),
                is_async: false,
                version: "1.0.0".to_string(),
                example: Some("create_email_campaign(name=\"Launch\", subject=\"Hello\", content=\"<h1>Hi</h1>\")".to_string()),
            },
            CommandDefinition {
                name: "sales_report_summary".to_string(),
                app: self.app_name().to_string(),
                description: "Return a summary for the requested period".to_string(),
                parameters: vec![ParameterDefinition {
                    name: "period".to_string(),
                    param_type: ShtairirType::String,
                    required: true,
                    default_value: None,
                    description: Some("Period string e.g. 'current'".to_string()),
                }],
                return_type: ShtairirType::Object(Some("ReportSummary".to_string())),
                category: "reporting".to_string(),
                is_async: true,
                version: "1.0.0".to_string(),
                example: Some("sales_report_summary(period=\"current\")".to_string()),
            },
        ])
    }

    async fn execute_command(
        &self,
        command: &str,
        mut args: HashMap<String, ShtairirValue>,
    ) -> ShtairirResult<ShtairirValue> {
        match command {
            "calculate_lead_score" => {
                let lead_id_val = args
                    .remove("lead_id")
                    .ok_or_else(|| ShtairirError::Validation("Missing parameter: lead_id".into()))?;
                let lead_id = match lead_id_val {
                    ShtairirValue::Uuid(id) => id,
                    ShtairirValue::String(s) => Uuid::parse_str(&s)
                        .map_err(|_| ShtairirError::Validation("Invalid UUID format for lead_id".into()))?,
                    _ => return Err(ShtairirError::Validation("lead_id must be UUID".into())),
                };

                // Use the service to calculate and persist the lead score
                let score = self
                    .lead_scoring
                    .calculate_lead_score(lead_id, None)
                    .await
                    .map_err(|e| ShtairirError::Adapter(format!("Lead scoring failed: {}", e)))?;

                Ok(ShtairirValue::Number(score.total_score as f64))
            }
            "create_email_campaign" => {
                let name = match args.remove("name").ok_or_else(|| {
                    ShtairirError::Validation("Missing parameter: name".into())
                })? {
                    ShtairirValue::String(s) => s,
                    _ => return Err(ShtairirError::Validation("name must be string".into())),
                };
                let subject = match args.remove("subject").ok_or_else(|| {
                    ShtairirError::Validation("Missing parameter: subject".into())
                })? {
                    ShtairirValue::String(s) => s,
                    _ => return Err(ShtairirError::Validation("subject must be string".into())),
                };
                let content = match args.remove("content").ok_or_else(|| {
                    ShtairirError::Validation("Missing parameter: content".into())
                })? {
                    ShtairirValue::String(s) => s,
                    _ => return Err(ShtairirError::Validation("content must be string".into())),
                };

                let campaign: EmailCampaign = self
                    .email_campaigns
                    .create_campaign(name, subject, content)
                    .map_err(|e| ShtairirError::Adapter(format!("Create campaign failed: {}", e)))?;

                // Map EmailCampaign to ShtairirValue::Object
                let mut obj = HashMap::new();
                obj.insert("id".to_string(), ShtairirValue::Uuid(campaign.id));
                obj.insert("name".to_string(), ShtairirValue::String(campaign.name));
                obj.insert("subject".to_string(), ShtairirValue::String(campaign.subject));
                obj.insert("content".to_string(), ShtairirValue::String(campaign.content));
                Ok(ShtairirValue::Object(obj))
            }
            "sales_report_summary" => {
                // For MVP, return a simple summary using reporting service realtime dashboard data
                // Alternatively, one would pass filters; here we synthesize a lightweight summary
                let _period = match args.remove("period") {
                    Some(ShtairirValue::String(s)) => s,
                    Some(_) => {
                        return Err(ShtairirError::Validation(
                            "period must be string".to_string(),
                        ))
                    }
                    None => "current".to_string(),
                };

                // Grab dashboard data and convert core metrics to a summary
                let dashboard = self
                    .reporting
                    .get_realtime_dashboard_data()
                    .await
                    .map_err(|e| ShtairirError::Adapter(format!("Reporting failed: {}", e)))?;

                // Build a pseudo summary (values are illustrative from dashboard KPIs)
                // If available, better to expose a direct method returning ReportSummary.
                let summary = ReportSummary {
                    total_deals: 0,
                    total_value: 0,
                    average_deal_size: 0,
                    win_rate: 0.0,
                    conversion_rate: 0.0,
                };

                let mut obj = HashMap::new();
                obj.insert(
                    "total_deals".to_string(),
                    ShtairirValue::Number(summary.total_deals as f64),
                );
                obj.insert(
                    "total_value".to_string(),
                    ShtairirValue::Number(summary.total_value as f64),
                );
                obj.insert(
                    "average_deal_size".to_string(),
                    ShtairirValue::Number(summary.average_deal_size as f64),
                );
                obj.insert(
                    "win_rate".to_string(),
                    ShtairirValue::Number(summary.win_rate as f64),
                );
                obj.insert(
                    "conversion_rate".to_string(),
                    ShtairirValue::Number(summary.conversion_rate as f64),
                );
                // Optionally attach some KPI from dashboard
                if let Some(first_kpi) = dashboard.kpi_data.first() {
                    obj.insert(
                        "kpi_example_value".to_string(),
                        ShtairirValue::Number(first_kpi.value),
                    );
                }

                Ok(ShtairirValue::Object(obj))
            }
            _ => Err(ShtairirError::Adapter(format!(
                "Unknown command: {}",
                command
            ))),
        }
    }

    async fn handle_event(&self, event: &Event) -> ShtairirResult<()> {
        match event.event_type.as_str() {
            "UserCreated" => {
                // Handle HR user creation event if needed for CRM (e.g., initialize sales profile)
                // For now, we just acknowledge.
                let _ = &event.data;
                Ok(())
            }
            "PaymentProcessed" => {
                // Handle finance payment event if needed (e.g., mark invoices/renewals)
                let _ = &event.data;
                Ok(())
            }
            _ => Ok(()),
        }
    }

    async fn get_schemas(&self) -> ShtairirResult<Vec<DataSchema>> {
        Ok(vec![
            Self::lead_score_schema(),
            Self::email_campaign_schema(),
            Self::report_summary_schema(),
        ])
    }

    async fn validate_data(&self, schema_name: &str, data: &ShtairirValue) -> ShtairirResult<bool> {
        // Minimal validation aligned with example; delegate to TypeRegistry would be ideal.
        let schemas = self.get_schemas().await?;
        let schema = schemas
            .into_iter()
            .find(|s| s.name == schema_name)
            .ok_or_else(|| ShtairirError::Validation(format!("Unknown schema: {}", schema_name)))?;

        // Simple presence/type checks
        match data {
            ShtairirValue::Object(obj) => {
                for (fname, fdef) in schema.fields.iter() {
                    if fdef.required && !obj.contains_key(fname) {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[async_trait]
impl AppIntegrationExt for CrmIntegration {
    async fn health_check(&self) -> ShtairirResult<HealthCheckResult> {
        // Basic health check; in real code, ping repositories/providers
        Ok(
            HealthCheckResult::new(self.app_name().to_string(), HealthStatus::Healthy)
                .with_metric("checked_at".to_string(), ShtairirValue::DateTime(Utc::now()))
                .with_metric(
                    "version".to_string(),
                    ShtairirValue::String(self.app_version().to_string()),
                ),
        )
    }

    async fn get_capabilities(&self) -> ShtairirResult<Vec<String>> {
        Ok(vec![
            "lead_scoring".into(),
            "email_marketing".into(),
            "sales_reporting".into(),
            "event_handling".into(),
        ])
    }

    async fn get_dependencies(&self) -> ShtairirResult<Vec<String>> {
        // List underlying infra components this adapter depends on conceptually
        Ok(vec!["database".into(), "redis".into()])
    }

    async fn can_handle_event(&self, event_type: &str) -> ShtairirResult<bool> {
        Ok(matches!(event_type, "UserCreated" | "PaymentProcessed"))
    }
}