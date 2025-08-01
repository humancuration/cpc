# Advanced CRM & Sales Pipeline Module Architecture (v2)

This document outlines the architecture for the advanced CRM module that extends the simple CRM for SMB needs.

## 1. Module Structure (Updated)

```
apps/advanced_crm/
├── ARCHITECTURE.md
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── domain/
    │   ├── lead_scoring.rs       # Lead scoring models
    │   ├── email_campaign.rs     # Email marketing models
    │   └── integration_points.rs # Integration markers
    ├── application/
    │   ├── lead_scoring_service.rs # Lead scoring implementation
    │   ├── email_campaign_service.rs # Email campaign management
    │   └── hr_integration_service.rs # HR integration
    ├── infrastructure/
    │   ├── email_provider/      # Email service integrations
    │   │   └── smtp.rs
    │   └── reporting/
    │       ├── pdf_generator.rs # Report generation
    │       └── data_export.rs   # Data export functionality
    └── presentation/
        ├── yew/
        │   ├── lead_scoring_ui.rs # Lead scoring UI
        │   └── email_campaign_ui.rs # Email campaign UI
        └── bevy/
            └── advanced_viz.rs  # Enhanced visualizations
```

## 2. Core Domain Models (Updated)

### Lead Scoring System (Enhanced)

```
// domain/lead_scoring.rs
pub struct LeadScore {
    pub lead_id: Uuid,
    pub base_score: u8,       // 0-100
    pub engagement_score: u8, // 0-100
    pub fit_score: u8,        // 0-100
    pub wellness_score: u8,   // 0-100 (new health integration)
    pub total_score: u8,      // 0-100
    pub scoring_factors: ScoringFactors,
    pub last_updated: DateTime<Utc>,
    pub scoring_model_id: Uuid, // References active scoring model
}

pub struct ScoringFactors {
    pub website_visits: u32,
    pub email_opens: u32,
    pub content_downloads: u32,
    pub social_engagement: u32,
    pub company_size: CompanySize,
    pub industry_fit: f32,
    pub wellness_metrics: WellnessMetrics, // New health integration
}

pub struct WellnessMetrics {
    pub stress_level: Option<u8>,
    pub focus_level: Option<u8>,
    pub burnout_risk: Option<f32>,
}
```

### Email Marketing Integration (New)

```rust
pub struct EmailCampaign {
    pub id: Uuid,
    pub name: String,
    pub subject: String,
    pub content: String,
    pub status: CampaignStatus,
    pub scheduled_time: Option<DateTime<Utc>>,
    pub target_segment: TargetSegment,
    pub metrics: CampaignMetrics,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub enum CampaignStatus {
    Draft,
    Scheduled,
    Sending,
    Completed,
    Cancelled,
}
```

## 3. Key Application Services (Updated)

### LeadScoringService (Enhanced)
- `calculate_lead_score(lead_id: Uuid, external_data: Option<ExternalLeadData>) -> Result<LeadScore>`: Calculates lead score using multiple factors including external data sources
- `get_top_leads(limit: u32, filter: LeadFilter) -> Result<Vec<(Lead, LeadScore)>>`: Gets highest scoring leads with filtering capability
- `update_scoring_model(model: ScoringModel) -> Result<()>`: Updates scoring algorithm with validation
- `get_scoring_history(lead_id: Uuid) -> Result<Vec<LeadScore>>`: Retrieves historical scoring data for trend analysis

### EmailMarketingService (New)
- `connect_provider(provider: EmailProviderConfig) -> Result<()>`: Establishes connection to Mailchimp/SendGrid
- `sync_contact_lists() -> Result<Vec<ContactList>>`: Synchronizes contact lists from external providers
- `create_campaign(campaign: NewCampaign, provider: EmailProvider) -> Result<EmailCampaign>`: Creates campaign on selected provider
- `get_campaign_metrics(campaign_id: Uuid) -> Result<CampaignMetrics>`: Aggregates metrics from multiple providers
- `handle_webhook_event(event: WebhookEvent) -> Result<()>`: Processes webhook events from email providers

### AdvancedReportingService (New)
- `generate_sales_report(filters: ReportFilters) -> Result<SalesReport>`: Generates comprehensive sales reports
- `get_pipeline_analysis() -> Result<PipelineAnalysis>`: Provides deep analysis of sales pipeline health
- `export_to_csv(report: SalesReport) -> Result<Vec<u8>>`: Exports reports to CSV format
- `get_realtime_dashboard_data() -> Result<DashboardData>`: Supplies data for Bevy visualizations

## 4. Key Design Decisions (Enhanced)

### Extending Simple CRM
The Advanced CRM extends the Simple CRM through:

1. **Trait-based Extension**:
```rust
// In simple CRM
pub trait CrmService {
    fn get_contact(&self, id: Uuid) -> Result<Contact>;
    // ...
}

// In advanced CRM
pub trait AdvancedCrmService: CrmService {
    fn calculate_lead_score(&self, lead_id: Uuid) -> Result<LeadScore>;
    fn create_email_campaign(&self, campaign: NewCampaign) -> Result<EmailCampaign>;
    // ...
}
```

2. **Decorator Pattern**:
```rust
pub struct AdvancedCrmServiceDecorator<T: CrmService> {
    base_service: T,
    lead_scoring: LeadScoringEngine,
    email_service: EmailServiceProvider,
}

impl<T: CrmService> CrmService for AdvancedCrmServiceDecorator<T> {
    // Delegate to base service
    fn get_contact(&self, id: Uuid) -> Result<Contact> {
        self.base_service.get_contact(id)
    }
    
    // Add advanced functionality
    fn calculate_lead_score(&self, lead_id: Uuid) -> Result<LeadScore> {
        // Implementation
    }
}
```

### HR Module Integration
We integrate with the HR module through:

1. **Shared Performance Metrics**:
```rust
// In advanced_crm/src/domain/integration_points.rs
#[derive(Serialize, Deserialize)]
pub struct SalesPerformanceData {
    pub user_id: Uuid,
    pub period: Period,
    pub deals_closed: u32,
    pub revenue_generated: i64, // in cents
    pub average_deal_size: i64,
    pub conversion_rate: f32,
    pub sales_velocity: f32,
    pub pipeline_health: PipelineHealth,
    pub wellness_impact: Option<WellnessImpact>, // New health integration
}

#[derive(Serialize, Deserialize)]
pub struct WellnessImpact {
    pub stress_impact: f32,
    pub focus_impact: f32,
    pub burnout_risk_factor: f32,
}

pub enum PipelineHealth {
    Strong,
    Moderate,
    Weak,
}

// New Email Provider Integration Model
#[derive(Serialize, Deserialize)]
pub struct EmailProviderConfig {
    pub provider: EmailProvider,
    pub api_key: String,
    pub connected_at: DateTime<Utc>,
    pub sync_contacts: bool,
    pub sync_campaigns: bool,
    pub last_sync: Option<DateTime<Utc>>,
}

pub enum EmailProvider {
    Mailchimp,
    SendGrid,
    CustomSmtp,
}
```

2. **Two-way Data Flow**:
- CRM sends sales performance data to HR (with optional wellness metrics)
- HR sends team structure and role information to CRM
- CRM integrates with email providers via webhook-based synchronization

### Data Privacy Considerations
- All integrations require explicit user consent with granular permissions
- Performance data is anonymized for team-level reports
- Individual performance data requires manager+ permissions
- Email marketing data follows GDPR/CCPA compliance requirements
- Health data integration is completely opt-in with separate consent flows

## 5. Database Schema Extensions (Updated)

### Lead Scoring Tables

```sql
CREATE TABLE lead_scoring_models (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    weights JSONB NOT NULL,  // Scoring weights configuration
    is_default BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE lead_scores (
    lead_id UUID PRIMARY KEY REFERENCES crm_contacts(id),
    base_score SMALLINT NOT NULL CHECK (base_score BETWEEN 0 AND 100),
    engagement_score SMALLINT NOT NULL CHECK (engagement_score BETWEEN 0 AND 100),
    fit_score SMALLINT NOT NULL CHECK (fit_score BETWEEN 0 AND 100),
    total_score SMALLINT NOT NULL CHECK (total_score BETWEEN 0 AND 100),
    scoring_factors JSONB NOT NULL,
    last_updated TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

// New Scoring Models Table
CREATE TABLE lead_scoring_models (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    weights JSONB NOT NULL,  // Scoring weights configuration
    is_default BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

// New Email Provider Configuration
CREATE TABLE email_provider_configs (
    id UUID PRIMARY KEY,
    provider VARCHAR(20) NOT NULL CHECK (provider IN ('mailchimp', 'sendgrid', 'custom_smtp')),
    api_key TEXT NOT NULL,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    sync_contacts BOOLEAN NOT NULL DEFAULT true,
    sync_campaigns BOOLEAN NOT NULL DEFAULT true,
    last_sync TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

// New Email Provider Webhook Events
CREATE TABLE email_webhook_events (
    id UUID PRIMARY KEY,
    provider VARCHAR(20) NOT NULL,
    event_type VARCHAR(50) NOT NULL,
    payload JSONB NOT NULL,
    processed BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

// New Sales Reporting Tables
CREATE TABLE sales_report_definitions (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    filters JSONB NOT NULL,
    visualization_type VARCHAR(50) NOT NULL,
    created_by UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE sales_report_instances (
    id UUID PRIMARY KEY,
    report_id UUID NOT NULL REFERENCES sales_report_definitions(id) ON DELETE CASCADE,
    generated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    data JSONB NOT NULL,
    format VARCHAR(20) NOT NULL CHECK (format IN ('csv', 'pdf', 'json')),
    generated_by UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE
);

### Email Campaign Tables (Enhanced)

```
CREATE TABLE email_campaigns (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    subject VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'draft' 
        CHECK (status IN ('draft', 'scheduled', 'sending', 'completed', 'cancelled')),
    scheduled_time TIMESTAMPTZ,
    created_by UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE email_campaign_recipients (
    campaign_id UUID NOT NULL REFERENCES email_campaigns(id) ON DELETE CASCADE,
    contact_id UUID NOT NULL REFERENCES crm_contacts(id) ON DELETE CASCADE,
    status VARCHAR(20) NOT NULL DEFAULT 'pending'
        CHECK (status IN ('pending', 'sent', 'opened', 'clicked', 'bounced', 'unsubscribed')),
    sent_time TIMESTAMPTZ,
    opened_time TIMESTAMPTZ,
    click_count INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (campaign_id, contact_id)
);
```

## 6. Integration with HR Module

### Data Flow Architecture

```mermaid
sequenceDiagram
    participant CRM as CRM Module
    participant AdvancedCRM as Advanced CRM
    participant HR as HR Module
    
    CRM->>AdvancedCRM: get_sales_performance(user_id, period)
    AdvancedCRM->>AdvancedCRM: Calculate metrics
    AdvancedCRM->>HR: sync_sales_performance(metrics)
    HR->>HR: Store in performance_data
    HR->>AdvancedCRM: return team_structure
    AdvancedCRM->>CRM: return enriched data
```

### Integration Points

1. **Performance Data Sharing**:
   - Endpoint: `POST /api/hr/performance/sales`
   - Request: `SalesPerformanceData`
   - Response: `IntegrationResult`

2. **Team Structure Access**:
   - Endpoint: `GET /api/hr/teams/{team_id}/members`
   - Response: `Vec<UserWithRoles>`

3. **Commission Calculation**:
   - Endpoint: `POST /api/hr/commissions/calculate`
   - Request: `CommissionCalculationRequest`
   - Response: `CommissionCalculationResult`

## 7. Dependencies

| Dependency | Purpose |
|------------|---------|
| `cpc-core::crm` | Base CRM functionality |
| `cpc-core::hr` | HR module integration |
| `cpc-net` | P2P data sharing for email campaigns |
| `rodio` | Audio notifications for campaign events |
| `pdf-rs` | PDF report generation |

## 8. Privacy and Cooperative Values Implementation

For information about our privacy policies and consent management, see our [Privacy Policy](../../../docs/privacy_policy.md).

This module extends the cooperative values established in the Simple CRM:

1. **Enhanced Consent Controls**:
   - Granular consent for sharing performance data with managers
   - Opt-in for anonymized data sharing to improve lead scoring models

2. **Transparency in Algorithms**:
   - Clear explanation of lead scoring factors
   - Ability to view and adjust scoring weights

3. **Data Ownership**:
   - Users own their sales performance data
   - Easy export of all CRM data in standard formats

4. **No Vendor Lock-in**:
   - Open data formats for email campaigns
   - Standard interfaces for email service providers

## 9. Migration Path from Simple CRM

1. **Database Migration**:
   - Add new tables for lead scoring and email campaigns
   - Create views for backward compatibility

2. **Service Layer Migration**:
   - Wrap Simple CRM services with Advanced CRM decorators
   - Maintain same interface for existing functionality

3. **UI Migration**:
   - Progressive enhancement of existing CRM UI
   - New features appear as optional modules in UI

This architecture provides a robust foundation for SMB CRM functionality while maintaining compatibility with our modular architecture and cooperative values.