//! PostgreSQL repository implementation for the BI & Analytics module

use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::{
    domain::{
        dataset::{Dataset, DataPoint, DataSource, FieldDefinition, DataType},
        report::{Report, VisualizationType},
        dashboard::{Dashboard, DashboardReport, GridPosition},
        compliance::gdpr::{GdprConsent, ProcessingPurpose, ConsentStatus, DataAccessRequest, DataAccessRequestType, DataAccessRequestStatus},
        compliance::hipaa::{AccessPermission, AccessRole, PhiCategory, AuditLogEntry, AuditAction, HipaaConfig},
    },
    application::{
        data_ingestion::DataRepository,
        report_generation::{ReportRepository, DataRepository as ReportDataRepository},
        dashboard_management::{DashboardRepository, ReportRepository as DashboardReportRepository},
        compliance_management::{GdprRepository, HipaaRepository},
    },
};
use std::collections::HashMap;

/// PostgreSQL implementation for BI & Analytics repositories
pub struct PostgresBiRepository {
    pool: PgPool,
}

impl PostgresBiRepository {
    /// Create a new PostgreSQL repository
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

/// Database model for datasets
#[derive(sqlx::FromRow)]
struct DatasetDbModel {
    id: Uuid,
    name: String,
    source: String,
    fields: serde_json::Value,
    description: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    owner_id: Uuid,
}

impl DatasetDbModel {
    /// Convert from domain Dataset to database model
    fn from_domain(dataset: &Dataset) -> Result<Self, serde_json::Error> {
        let fields_json = serde_json::to_value(&dataset.fields)?;
        
        Ok(Self {
            id: dataset.id,
            name: dataset.name.clone(),
            source: match &dataset.source {
                DataSource::Crm => "crm".to_string(),
                DataSource::Finance => "finance".to_string(),
                DataSource::Calendar => "calendar".to_string(),
                DataSource::Messenger => "messenger".to_string(),
                DataSource::Custom(source) => format!("custom:{}", source),
            },
            fields: fields_json,
            description: dataset.description.clone(),
            created_at: dataset.created_at,
            updated_at: dataset.updated_at,
            owner_id: dataset.owner_id,
        })
    }
    
    /// Convert from database model to domain Dataset
    fn to_domain(&self) -> Result<Dataset, Box<dyn std::error::Error>> {
        let fields: Vec<FieldDefinition> = serde_json::from_value(self.fields.clone())?;
        
        let source = match self.source.as_str() {
            "crm" => DataSource::Crm,
            "finance" => DataSource::Finance,
            "calendar" => DataSource::Calendar,
            "messenger" => DataSource::Messenger,
            custom if custom.starts_with("custom:") => {
                DataSource::Custom(custom[7..].to_string())
            }
            _ => DataSource::Custom(self.source.clone()),
        };
        
        Ok(Dataset {
            id: self.id,
            name: self.name.clone(),
            source,
            fields,
            description: self.description.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
            owner_id: self.owner_id,
        })
    }
}

/// Database model for data points
#[derive(sqlx::FromRow)]
struct DataPointDbModel {
    id: Uuid,
    dataset_id: Uuid,
    timestamp: DateTime<Utc>,
    values: serde_json::Value,
    metadata: serde_json::Value,
}

impl DataPointDbModel {
    /// Convert from domain DataPoint to database model
    fn from_domain(dataset_id: Uuid, data_point: &DataPoint) -> Result<Self, serde_json::Error> {
        let values_json = serde_json::to_value(&data_point.values)?;
        let metadata_json = serde_json::to_value(&data_point.metadata)?;
        
        Ok(Self {
            id: Uuid::new_v4(),
            dataset_id,
            timestamp: data_point.timestamp,
            values: values_json,
            metadata: metadata_json,
        })
    }
    
    /// Convert from database model to domain DataPoint
    fn to_domain(&self) -> Result<DataPoint, Box<dyn std::error::Error>> {
        let values: HashMap<String, serde_json::Value> = serde_json::from_value(self.values.clone())?;
        let metadata: HashMap<String, String> = serde_json::from_value(self.metadata.clone())?;
        
        Ok(DataPoint {
            timestamp: self.timestamp,
            values,
            metadata,
        })
    }
}

/// Database model for reports
#[derive(sqlx::FromRow)]
struct ReportDbModel {
    id: Uuid,
    dataset_id: Uuid,
    name: String,
    description: Option<String>,
    query: String,
    visualization_type: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    owner_id: Uuid,
}

impl ReportDbModel {
    /// Convert from domain Report to database model
    fn from_domain(report: &Report) -> Self {
        Self {
            id: report.id,
            dataset_id: report.dataset_id,
            name: report.name.clone(),
            description: report.description.clone(),
            query: report.query.clone(),
            visualization_type: match &report.visualization_type {
                VisualizationType::Table => "table".to_string(),
                VisualizationType::BarChart => "bar_chart".to_string(),
                VisualizationType::LineChart => "line_chart".to_string(),
                VisualizationType::PieChart => "pie_chart".to_string(),
                VisualizationType::ScatterPlot => "scatter_plot".to_string(),
                VisualizationType::Heatmap => "heatmap".to_string(),
                VisualizationType::AreaChart => "area_chart".to_string(),
            },
            created_at: report.created_at,
            updated_at: report.updated_at,
            owner_id: report.owner_id,
        }
    }
    
    /// Convert from database model to domain Report
    fn to_domain(&self) -> Report {
        let visualization_type = match self.visualization_type.as_str() {
            "table" => VisualizationType::Table,
            "bar_chart" => VisualizationType::BarChart,
            "line_chart" => VisualizationType::LineChart,
            "pie_chart" => VisualizationType::PieChart,
            "scatter_plot" => VisualizationType::ScatterPlot,
            "heatmap" => VisualizationType::Heatmap,
            "area_chart" => VisualizationType::AreaChart,
            _ => VisualizationType::Table,
        };
        
        Report {
            id: self.id,
            dataset_id: self.dataset_id,
            name: self.name.clone(),
            description: self.description.clone(),
            query: self.query.clone(),
            visualization_type,
            created_at: self.created_at,
            updated_at: self.updated_at,
            owner_id: self.owner_id,
        }
    }
}

/// Database model for dashboards
#[derive(sqlx::FromRow)]
struct DashboardDbModel {
    id: Uuid,
    name: String,
    description: Option<String>,
    layout: serde_json::Value,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    owner_id: Uuid,
}

impl DashboardDbModel {
    /// Convert from domain Dashboard to database model
    fn from_domain(dashboard: &Dashboard) -> Result<Self, serde_json::Error> {
        let layout_json = serde_json::to_value(&dashboard.layout)?;
        
        Ok(Self {
            id: dashboard.id,
            name: dashboard.name.clone(),
            description: dashboard.description.clone(),
            layout: layout_json,
            created_at: dashboard.created_at,
            updated_at: dashboard.updated_at,
            owner_id: dashboard.owner_id,
        })
    }
    
    /// Convert from database model to domain Dashboard
    fn to_domain(&self) -> Result<Dashboard, Box<dyn std::error::Error>> {
        let layout: HashMap<String, serde_json::Value> = serde_json::from_value(self.layout.clone())?;
        
        Ok(Dashboard {
            id: self.id,
            name: self.name.clone(),
            description: self.description.clone(),
            layout,
            created_at: self.created_at,
            updated_at: self.updated_at,
            owner_id: self.owner_id,
        })
    }
}

/// Database model for dashboard reports
#[derive(sqlx::FromRow)]
struct DashboardReportDbModel {
    id: Uuid,
    dashboard_id: Uuid,
    report_id: Uuid,
    position_x: i32,
    position_y: i32,
    position_width: i32,
    position_height: i32,
}

impl DashboardReportDbModel {
    /// Convert from domain DashboardReport to database model
    fn from_domain(dashboard_report: &DashboardReport) -> Self {
        Self {
            id: dashboard_report.id,
            dashboard_id: dashboard_report.dashboard_id,
            report_id: dashboard_report.report_id,
            position_x: dashboard_report.position.x as i32,
            position_y: dashboard_report.position.y as i32,
            position_width: dashboard_report.position.width as i32,
            position_height: dashboard_report.position.height as i32,
        }
    }
    
    /// Convert from database model to domain DashboardReport
    fn to_domain(&self) -> DashboardReport {
        DashboardReport {
            id: self.id,
            dashboard_id: self.dashboard_id,
            report_id: self.report_id,
            position: GridPosition {
                x: self.position_x as u32,
                y: self.position_y as u32,
                width: self.position_width as u32,
                height: self.position_height as u32,
            },
        }
    }
}

#[async_trait]
impl DataRepository for PostgresBiRepository {
    async fn save_dataset(&self, dataset: &Dataset) -> Result<(), super::super::application::data_ingestion::DataIngestionError> {
        let dataset_model = DatasetDbModel::from_domain(dataset)
            .map_err(|e| super::super::application::data_ingestion::DataIngestionError::DatasetError(e.to_string()))?;
        
        sqlx::query!(
            r#"
            INSERT INTO bi_datasets (id, name, source, fields, description, created_at, updated_at, owner_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (id) DO UPDATE SET
                name = EXCLUDED.name,
                source = EXCLUDED.source,
                fields = EXCLUDED.fields,
                description = EXCLUDED.description,
                updated_at = EXCLUDED.updated_at
            "#,
            dataset_model.id,
            dataset_model.name,
            dataset_model.source,
            dataset_model.fields,
            dataset_model.description,
            dataset_model.created_at,
            dataset_model.updated_at,
            dataset_model.owner_id,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| super::super::application::data_ingestion::DataIngestionError::StorageError(e.to_string()))?;
        
        Ok(())
    }
    
    async fn get_dataset(&self, id: Uuid) -> Result<Dataset, super::super::application::data_ingestion::DataIngestionError> {
        let dataset_record = sqlx::query_as!(
            DatasetDbModel,
            r#"
            SELECT id, name, source, fields, description, created_at, updated_at, owner_id
            FROM bi_datasets
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| super::super::application::data_ingestion::DataIngestionError::StorageError(e.to_string()))?;
        
        let dataset_model = dataset_record
            .ok_or_else(|| super::super::application::data_ingestion::DataIngestionError::DatasetError("Dataset not found".to_string()))?;
        
        let dataset = dataset_model.to_domain()
            .map_err(|e| super::super::application::data_ingestion::DataIngestionError::DatasetError(e.to_string()))?;
        
        Ok(dataset)
    }
    
    async fn store_data_points(&self, dataset_id: Uuid, data_points: Vec<DataPoint>) -> Result<(), super::super::application::data_ingestion::DataIngestionError> {
        for data_point in data_points {
            let data_point_model = DataPointDbModel::from_domain(dataset_id, &data_point)
                .map_err(|e| super::super::application::data_ingestion::DataIngestionError::StorageError(e.to_string()))?;
            
            sqlx::query!(
                r#"
                INSERT INTO bi_data_points (id, dataset_id, timestamp, values, metadata)
                VALUES ($1, $2, $3, $4, $5)
                "#,
                data_point_model.id,
                data_point_model.dataset_id,
                data_point_model.timestamp,
                data_point_model.values,
                data_point_model.metadata,
            )
            .execute(&self.pool)
            .await
            .map_err(|e| super::super::application::data_ingestion::DataIngestionError::StorageError(e.to_string()))?;
        }
        
        Ok(())
    }
    
    async fn get_data_points(&self, dataset_id: Uuid) -> Result<Vec<DataPoint>, super::super::application::data_ingestion::DataIngestionError> {
        let data_point_records = sqlx::query_as!(
            DataPointDbModel,
            r#"
            SELECT id, dataset_id, timestamp, values, metadata
            FROM bi_data_points
            WHERE dataset_id = $1
            ORDER BY timestamp
            "#,
            dataset_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| super::super::application::data_ingestion::DataIngestionError::StorageError(e.to_string()))?;
        
        let mut data_points = Vec::new();
        
        for record in data_point_records {
            let data_point = record.to_domain()
                .map_err(|e| super::super::application::data_ingestion::DataIngestionError::StorageError(e.to_string()))?;
            data_points.push(data_point);
        }
        
        Ok(data_points)
    }
}

#[async_trait]
impl ReportRepository for PostgresBiRepository {
    async fn save_report(&self, report: &Report) -> Result<(), super::super::application::report_generation::ReportGenerationError> {
        let report_model = ReportDbModel::from_domain(report);
        
        sqlx::query!(
            r#"
            INSERT INTO bi_reports (id, dataset_id, name, description, query, visualization_type, created_at, updated_at, owner_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (id) DO UPDATE SET
                dataset_id = EXCLUDED.dataset_id,
                name = EXCLUDED.name,
                description = EXCLUDED.description,
                query = EXCLUDED.query,
                visualization_type = EXCLUDED.visualization_type,
                updated_at = EXCLUDED.updated_at
            "#,
            report_model.id,
            report_model.dataset_id,
            report_model.name,
            report_model.description,
            report_model.query,
            report_model.visualization_type,
            report_model.created_at,
            report_model.updated_at,
            report_model.owner_id,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| super::super::application::report_generation::ReportGenerationError::ReportError(e.to_string()))?;
        
        Ok(())
    }
    
    async fn get_report(&self, id: Uuid) -> Result<Report, super::super::application::report_generation::ReportGenerationError> {
        let report_record = sqlx::query_as!(
            ReportDbModel,
            r#"
            SELECT id, dataset_id, name, description, query, visualization_type, created_at, updated_at, owner_id
            FROM bi_reports
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| super::super::application::report_generation::ReportGenerationError::ReportError(e.to_string()))?;
        
        let report_model = report_record
            .ok_or_else(|| super::super::application::report_generation::ReportGenerationError::ReportError("Report not found".to_string()))?;
        
        let report = report_model.to_domain();
        Ok(report)
    }
    
    async fn get_reports_by_owner(&self, owner_id: Uuid) -> Result<Vec<Report>, super::super::application::report_generation::ReportGenerationError> {
        let report_records = sqlx::query_as!(
            ReportDbModel,
            r#"
            SELECT id, dataset_id, name, description, query, visualization_type, created_at, updated_at, owner_id
            FROM bi_reports
            WHERE owner_id = $1
            ORDER BY name
            "#,
            owner_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| super::super::application::report_generation::ReportGenerationError::ReportError(e.to_string()))?;
        
        let reports: Vec<Report> = report_records
            .into_iter()
            .map(|record| record.to_domain())
            .collect();
        
        Ok(reports)
    }
}

#[async_trait]
impl ReportDataRepository for PostgresBiRepository {
    async fn get_dataset(&self, id: Uuid) -> Result<crate::domain::dataset::Dataset, super::super::application::report_generation::ReportGenerationError> {
        // Delegate to DataRepository implementation
        let data_repo = self as &dyn DataRepository;
        data_repo.get_dataset(id).await
            .map_err(|e| super::super::application::report_generation::ReportGenerationError::DatasetError(e.to_string()))
    }
    
    async fn get_data_points(&self, dataset_id: Uuid) -> Result<Vec<crate::domain::dataset::DataPoint>, super::super::application::report_generation::ReportGenerationError> {
        // Delegate to DataRepository implementation
        let data_repo = self as &dyn DataRepository;
        data_repo.get_data_points(dataset_id).await
            .map_err(|e| super::super::application::report_generation::ReportGenerationError::DataQueryError(e.to_string()))
    }
}

#[async_trait]
impl DashboardRepository for PostgresBiRepository {
    async fn save_dashboard(&self, dashboard: &Dashboard) -> Result<(), super::super::application::dashboard_management::DashboardManagementError> {
        let dashboard_model = DashboardDbModel::from_domain(dashboard)
            .map_err(|e| super::super::application::dashboard_management::DashboardManagementError::DashboardError(e.to_string()))?;
        
        sqlx::query!(
            r#"
            INSERT INTO bi_dashboards (id, name, description, layout, created_at, updated_at, owner_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (id) DO UPDATE SET
                name = EXCLUDED.name,
                description = EXCLUDED.description,
                layout = EXCLUDED.layout,
                updated_at = EXCLUDED.updated_at
            "#,
            dashboard_model.id,
            dashboard_model.name,
            dashboard_model.description,
            dashboard_model.layout,
            dashboard_model.created_at,
            dashboard_model.updated_at,
            dashboard_model.owner_id,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| super::super::application::dashboard_management::DashboardManagementError::DashboardError(e.to_string()))?;
        
        Ok(())
    }
    
    async fn get_dashboard(&self, id: Uuid) -> Result<Dashboard, super::super::application::dashboard_management::DashboardManagementError> {
        let dashboard_record = sqlx::query_as!(
            DashboardDbModel,
            r#"
            SELECT id, name, description, layout, created_at, updated_at, owner_id
            FROM bi_dashboards
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| super::super::application::dashboard_management::DashboardManagementError::DashboardError(e.to_string()))?;
        
        let dashboard_model = dashboard_record
            .ok_or_else(|| super::super::application::dashboard_management::DashboardManagementError::DashboardError("Dashboard not found".to_string()))?;
        
        let dashboard = dashboard_model.to_domain()
            .map_err(|e| super::super::application::dashboard_management::DashboardManagementError::DashboardError(e.to_string()))?;
        
        Ok(dashboard)
    }
    
    async fn get_dashboards_by_owner(&self, owner_id: Uuid) -> Result<Vec<Dashboard>, super::super::application::dashboard_management::DashboardManagementError> {
        let dashboard_records = sqlx::query_as!(
            DashboardDbModel,
            r#"
            SELECT id, name, description, layout, created_at, updated_at, owner_id
            FROM bi_dashboards
            WHERE owner_id = $1
            ORDER BY name
            "#,
            owner_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| super::super::application::dashboard_management::DashboardManagementError::DashboardError(e.to_string()))?;
        
        let mut dashboards = Vec::new();
        
        for record in dashboard_records {
            let dashboard = record.to_domain()
                .map_err(|e| super::super::application::dashboard_management::DashboardManagementError::DashboardError(e.to_string()))?;
            dashboards.push(dashboard);
        }
        
        Ok(dashboards)
    }
    
    async fn save_dashboard_report(&self, dashboard_report: &DashboardReport) -> Result<(), super::super::application::dashboard_management::DashboardManagementError> {
        let dashboard_report_model = DashboardReportDbModel::from_domain(dashboard_report);
        
        sqlx::query!(
            r#"
            INSERT INTO bi_dashboard_reports (id, dashboard_id, report_id, position_x, position_y, position_width, position_height)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (id) DO UPDATE SET
                dashboard_id = EXCLUDED.dashboard_id,
                report_id = EXCLUDED.report_id,
                position_x = EXCLUDED.position_x,
                position_y = EXCLUDED.position_y,
                position_width = EXCLUDED.position_width,
                position_height = EXCLUDED.position_height
            "#,
            dashboard_report_model.id,
            dashboard_report_model.dashboard_id,
            dashboard_report_model.report_id,
            dashboard_report_model.position_x,
            dashboard_report_model.position_y,
            dashboard_report_model.position_width,
            dashboard_report_model.position_height,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| super::super::application::dashboard_management::DashboardManagementError::DashboardError(e.to_string()))?;
        
        Ok(())
    }
    
    async fn get_dashboard_report(&self, id: Uuid) -> Result<DashboardReport, super::super::application::dashboard_management::DashboardManagementError> {
        let dashboard_report_record = sqlx::query_as!(
            DashboardReportDbModel,
            r#"
            SELECT id, dashboard_id, report_id, position_x, position_y, position_width, position_height
            FROM bi_dashboard_reports
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| super::super::application::dashboard_management::DashboardManagementError::DashboardError(e.to_string()))?;
        
        let dashboard_report_model = dashboard_report_record
            .ok_or_else(|| super::super::application::dashboard_management::DashboardManagementError::DashboardError("Dashboard report not found".to_string()))?;
        
        let dashboard_report = dashboard_report_model.to_domain();
        Ok(dashboard_report)
    }
    
    async fn get_dashboard_reports(&self, dashboard_id: Uuid) -> Result<Vec<DashboardReport>, super::super::application::dashboard_management::DashboardManagementError> {
        let dashboard_report_records = sqlx::query_as!(
            DashboardReportDbModel,
            r#"
            SELECT id, dashboard_id, report_id, position_x, position_y, position_width, position_height
            FROM bi_dashboard_reports
            WHERE dashboard_id = $1
            ORDER BY id
            "#,
            dashboard_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| super::super::application::dashboard_management::DashboardManagementError::DashboardError(e.to_string()))?;
        
        let dashboard_reports: Vec<DashboardReport> = dashboard_report_records
            .into_iter()
            .map(|record| record.to_domain())
            .collect();
        
        Ok(dashboard_reports)
    }
    
    async fn delete_dashboard_report(&self, id: Uuid) -> Result<(), super::super::application::dashboard_management::DashboardManagementError> {
        sqlx::query!(
            r#"
            DELETE FROM bi_dashboard_reports
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| super::super::application::dashboard_management::DashboardManagementError::DashboardError(e.to_string()))?;
        
        Ok(())
    }
}

#[async_trait]
impl DashboardReportRepository for PostgresBiRepository {
    async fn get_report(&self, id: Uuid) -> Result<crate::domain::report::Report, super::super::application::dashboard_management::DashboardManagementError> {
        // Delegate to ReportRepository implementation
        let report_repo = self as &dyn ReportRepository;
        report_repo.get_report(id).await
    }
}

#[async_trait]
impl GdprRepository for PostgresBiRepository {
    async fn save_consent(&self, consent: &GdprConsent) -> Result<(), super::super::application::compliance_management::ComplianceManagementError> {
        let purpose = match consent.purpose {
            ProcessingPurpose::Analytics => "analytics",
            ProcessingPurpose::Reporting => "reporting",
            ProcessingPurpose::Research => "research",
            ProcessingPurpose::Marketing => "marketing",
        };
        
        let status = match consent.status {
            ConsentStatus::Granted => "granted",
            ConsentStatus::Denied => "denied",
            ConsentStatus::Revoked => "revoked",
        };
        
        sqlx::query!(
            r#"
            INSERT INTO bi_gdpr_consents (id, user_id, purpose, status, granted_at, revoked_at, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (id) DO UPDATE SET
                user_id = EXCLUDED.user_id,
                purpose = EXCLUDED.purpose,
                status = EXCLUDED.status,
                granted_at = EXCLUDED.granted_at,
                revoked_at = EXCLUDED.revoked_at,
                updated_at = EXCLUDED.updated_at
            "#,
            consent.id,
            consent.user_id,
            purpose,
            status,
            consent.granted_at,
            consent.revoked_at,
            consent.created_at,
            consent.updated_at,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| super::super::application::compliance_management::ComplianceManagementError::GdprError(e.to_string()))?;
        
        Ok(())
    }
    
    async fn get_consent(&self, id: Uuid) -> Result<GdprConsent, super::super::application::compliance_management::ComplianceManagementError> {
        let record = sqlx::query!(
            r#"
            SELECT id, user_id, purpose, status, granted_at, revoked_at, created_at, updated_at
            FROM bi_gdpr_consents
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| super::super::application::compliance_management::ComplianceManagementError::GdprError(e.to_string()))?;
        
        let record = record
            .ok_or_else(|| super::super::application::compliance_management::ComplianceManagementError::GdprError("Consent not found".to_string()))?;
        
        let purpose = match record.purpose.as_str() {
            "analytics" => ProcessingPurpose::Analytics,
            "reporting" => ProcessingPurpose::Reporting,
            "research" => ProcessingPurpose::Research,
            "marketing" => ProcessingPurpose::Marketing,
            _ => ProcessingPurpose::Analytics,
        };
        
        let status = match record.status.as_str() {
            "granted" => ConsentStatus::Granted,
            "denied" => ConsentStatus::Denied,
            "revoked" => ConsentStatus::Revoked,
            _ => ConsentStatus::Denied,
        };
        
        Ok(GdprConsent {
            id: record.id,
            user_id: record.user_id,
            purpose,
            status,
            granted_at: record.granted_at,
            revoked_at: record.revoked_at,
            created_at: record.created_at,
            updated_at: record.updated_at,
        })
    }
    
    async fn get_consents_by_user_and_purpose(&self, user_id: Uuid, purpose: &ProcessingPurpose) -> Result<Vec<GdprConsent>, super::super::application::compliance_management::ComplianceManagementError> {
        let purpose_str = match purpose {
            ProcessingPurpose::Analytics => "analytics",
            ProcessingPurpose::Reporting => "reporting",
            ProcessingPurpose::Research => "research",
            ProcessingPurpose::Marketing => "marketing",
        };
        
        let records = sqlx::query!(
            r#"
            SELECT id, user_id, purpose, status, granted_at, revoked_at, created_at, updated_at
            FROM bi_gdpr_consents
            WHERE user_id = $1 AND purpose = $2
            ORDER BY created_at DESC
            "#,
            user_id,
            purpose_str,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| super::super::application::compliance_management::ComplianceManagementError::GdprError(e.to_string()))?;
        
        let consents: Vec<GdprConsent> = records
            .into_iter()
            .map(|record| {
                let purpose = match record.purpose.as_str() {
                    "analytics" => ProcessingPurpose::Analytics,
                    "reporting" => ProcessingPurpose::Reporting,
                    "research" => ProcessingPurpose::Research,
                    "marketing" => ProcessingPurpose::Marketing,
                    _ => ProcessingPurpose::Analytics,
                };
                
                let status = match record.status.as_str() {
                    "granted" => ConsentStatus::Granted,
                    "denied" => ConsentStatus::Denied,
                    "revoked" => ConsentStatus::Revoked,
                    _ => ConsentStatus::Denied,
                };
                
                GdprConsent {
                    id: record.id,
                    user_id: record.user_id,
                    purpose,
                    status,
                    granted_at: record.granted_at,
                    revoked_at: record.revoked_at,
                    created_at: record.created_at,
                    updated_at: record.updated_at,
                }
            })
            .collect();
        
        Ok(consents)
    }
    
    async fn save_data_access_request(&self, request: &DataAccessRequest) -> Result<(), super::super::application::compliance_management::ComplianceManagementError> {
        let request_type = match request.request_type {
            DataAccessRequestType::DataExport => "export",
            DataAccessRequestType::DataDeletion => "deletion",
            DataAccessRequestType::DataCorrection => "correction",
            DataAccessRequestType::DataPortability => "portability",
        };
        
        let status = match request.status {
            DataAccessRequestStatus::Pending => "pending",
            DataAccessRequestStatus::Processing => "processing",
            DataAccessRequestStatus::Fulfilled => "fulfilled",
            DataAccessRequestStatus::Rejected => "rejected",
        };
        
        let requested_data_json = serde_json::to_string(&request.requested_data)
            .map_err(|e| super::super::application::compliance_management::ComplianceManagementError::GdprError(e.to_string()))?;
        
        sqlx::query!(
            r#"
            INSERT INTO bi_data_access_requests (id, user_id, request_type, status, requested_data, fulfillment_details, requested_at, fulfilled_at, rejected_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (id) DO UPDATE SET
                user_id = EXCLUDED.user_id,
                request_type = EXCLUDED.request_type,
                status = EXCLUDED.status,
                requested_data = EXCLUDED.requested_data,
                fulfillment_details = EXCLUDED.fulfillment_details,
                requested_at = EXCLUDED.requested_at,
                fulfilled_at = EXCLUDED.fulfilled_at,
                rejected_at = EXCLUDED.rejected_at
            "#,
            request.id,
            request.user_id,
            request_type,
            status,
            requested_data_json,
            request.fulfillment_details,
            request.requested_at,
            request.fulfilled_at,
            request.rejected_at,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| super::super::application::compliance_management::ComplianceManagementError::GdprError(e.to_string()))?;
        
        Ok(())
    }
    
    async fn get_data_access_request(&self, id: Uuid) -> Result<DataAccessRequest, super::super::application::compliance_management::ComplianceManagementError> {
        let record = sqlx::query!(
            r#"
            SELECT id, user_id, request_type, status, requested_data, fulfillment_details, requested_at, fulfilled_at, rejected_at
            FROM bi_data_access_requests
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| super::super::application::compliance_management::ComplianceManagementError::GdprError(e.to_string()))?;
        
        let record = record
            .ok_or_else(|| super::super::application::compliance_management::ComplianceManagementError::GdprError("Data access request not found".to_string()))?;
        
        let request_type = match record.request_type.as_str() {
            "export" => DataAccessRequestType::DataExport,
            "deletion" => DataAccessRequestType::DataDeletion,
            "correction" => DataAccessRequestType::DataCorrection,
            "portability" => DataAccessRequestType::DataPortability,
            _ => DataAccessRequestType::DataExport,
        };
        
        let status = match record.status.as_str() {
            "pending" => DataAccessRequestStatus::Pending,
            "processing" => DataAccessRequestStatus::Processing,
            "fulfilled" => DataAccessRequestStatus::Fulfilled,
            "rejected" => DataAccessRequestStatus::Rejected,
            _ => DataAccessRequestStatus::Pending,
        };
        
        let requested_data: Vec<String> = serde_json::from_str(&record.requested_data)
            .map_err(|e| super::super::application::compliance_management::ComplianceManagementError::GdprError(e.to_string()))?;
        
        Ok(DataAccessRequest {
            id: record.id,
            user_id: record.user_id,
            request_type,
            status,
            requested_data,
            fulfillment_details: record.fulfillment_details,
            requested_at: record.requested_at,
            fulfilled_at: record.fulfilled_at,
            rejected_at: record.rejected_at,
        })
    }
    
    async fn get_data_access_requests_by_user(&self, user_id: Uuid) -> Result<Vec<DataAccessRequest>, super::super::application::compliance_management::ComplianceManagementError> {
        let records = sqlx::query!(
            r#"
            SELECT id, user_id, request_type, status, requested_data, fulfillment_details, requested_at, fulfilled_at, rejected_at
            FROM bi_data_access_requests
            WHERE user_id = $1
            ORDER BY requested_at DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| super::super::application::compliance_management::ComplianceManagementError::GdprError(e.to_string()))?;
        
        let mut requests = Vec::new();
        
        for record in records {
            let request_type = match record.request_type.as_str() {
                "export" => DataAccessRequestType::DataExport,
                "deletion" => DataAccessRequestType::DataDeletion,
                "correction" => DataAccessRequestType::DataCorrection,
                "portability" => DataAccessRequestType::DataPortability,
                _ => DataAccessRequestType::DataExport,
            };
            
            let status = match record.status.as_str() {
                "pending" => DataAccessRequestStatus::Pending,
                "processing" => DataAccessRequestStatus::Processing,
                "fulfilled" => DataAccessRequestStatus::Fulfilled,
                "rejected" => DataAccessRequestStatus::Rejected,
                _ => DataAccessRequestStatus::Pending,
            };
            
            let requested_data: Vec<String> = serde_json::from_str(&record.requested_data)
                .map_err(|e| super::super::application::compliance_management::ComplianceManagementError::GdprError(e.to_string()))?;
            
            requests.push(DataAccessRequest {
                id: record.id,
                user_id: record.user_id,
                request_type,
                status,
                requested_data,
                fulfillment_details: record.fulfillment_details,
                requested_at: record.requested_at,
                fulfilled_at: record.fulfilled_at,
                rejected_at: record.rejected_at,
            });
        }
        
        Ok(requests)
    }
}

#[async_trait]
impl HipaaRepository for PostgresBiRepository {
    async fn save_access_permission(&self, permission: &AccessPermission) -> Result<(), super::super::application::compliance_management::ComplianceManagementError> {
        let role = match permission.role {
            AccessRole::Admin => "admin",
            AccessRole::Analyst => "analyst",
            AccessRole::Researcher => "researcher",
            AccessRole::Auditor => "auditor",
        };
        
        let phi_categories_json = serde_json::to_string(&permission.phi_categories)
            .map_err(|e| super::super::application::compliance_management::ComplianceManagementError::HipaaError(e.to_string()))?;
        
        sqlx::query!(
            r#"
            INSERT INTO bi_hipaa_permissions (id, user_id, role, phi_categories, granted_at, expires_at, is_active)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (id) DO UPDATE SET
                user_id = EXCLUDED.user_id,
                role = EXCLUDED.role,
                phi_categories = EXCLUDED.phi_categories,
                granted_at = EXCLUDED.granted_at,
                expires_at = EXCLUDED.expires_at,
                is_active = EXCLUDED.is_active
            "#,
            permission.id,
            permission.user_id,
            role,
            phi_categories_json,
            permission.granted_at,
            permission.expires_at,
            permission.is_active,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| super::super::application::compliance_management::ComplianceManagementError::HipaaError(e.to_string()))?;
        
        Ok(())
    }
    
    async fn get_permission(&self, id: Uuid) -> Result<AccessPermission, super::super::application::compliance_management::ComplianceManagementError> {
        let record = sqlx::query!(
            r#"
            SELECT id, user_id, role, phi_categories, granted_at, expires_at, is_active
            FROM bi_hipaa_permissions
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| super::super::application::compliance_management::ComplianceManagementError::HipaaError(e.to_string()))?;
        
        let record = record
            .ok_or_else(|| super::super::application::compliance_management::ComplianceManagementError::HipaaError("Permission not found".to_string()))?;
        
        let role = match record.role.as_str() {
            "admin" => AccessRole::Admin,
            "analyst" => AccessRole::Analyst,
            "researcher" => AccessRole::Researcher,
            "auditor" => AccessRole::Auditor,
            _ => AccessRole::Analyst,
        };
        
        let phi_categories: Vec<PhiCategory> = serde_json::from_str(&record.phi_categories)
            .map_err(|e| super::super::application::compliance_management::ComplianceManagementError::HipaaError(e.to_string()))?;
        
        Ok(AccessPermission {
            id: record.id,
            user_id: record.user_id,
            role,
            phi_categories,
            granted_at: record.granted_at,
            expires_at: record.expires_at,
            is_active: record.is_active,
        })
    }
    
    async fn get_permissions_by_user(&self, user_id: Uuid) -> Result<Vec<AccessPermission>, super::super::application::compliance_management::ComplianceManagementError> {
        let records = sqlx::query!(
            r#"
            SELECT id, user_id, role, phi_categories, granted_at, expires_at, is_active
            FROM bi_hipaa_permissions
            WHERE user_id = $1
            ORDER BY granted_at DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| super::super::application::compliance_management::ComplianceManagementError::HipaaError(e.to_string()))?;
        
        let mut permissions = Vec::new();
        
        for record in records {
            let role = match record.role.as_str() {
                "admin" => AccessRole::Admin,
                "analyst" => AccessRole::Analyst,
                "researcher" => AccessRole::Researcher,
                "auditor" => AccessRole::Auditor,
                _ => AccessRole::Analyst,
            };
            
            let phi_categories: Vec<PhiCategory> = serde_json::from_str(&record.phi_categories)
                .map_err(|e| super::super::application::compliance_management::ComplianceManagementError::HipaaError(e.to_string()))?;
            
            permissions.push(AccessPermission {
                id: record.id,
                user_id: record.user_id,
                role,
                phi_categories,
                granted_at: record.granted_at,
                expires_at: record.expires_at,
                is_active: record.is_active,
            });
        }
        
        Ok(permissions)
    }
    
    async fn save_audit_log_entry(&self, entry: &AuditLogEntry) -> Result<(), super::super::application::compliance_management::ComplianceManagementError> {
        let action = match entry.action {
            AuditAction::View => "view",
            AuditAction::Export => "export",
            AuditAction::Modify => "modify",
            AuditAction::Delete => "delete",
            AuditAction::Query => "query",
        };
        
        let phi_category_str = entry.phi_category.as_ref().map(|c| match c {
            PhiCategory::Demographic => "demographic",
            PhiCategory::MedicalHistory => "medical_history",
            PhiCategory::TreatmentInfo => "treatment_info",
            PhiCategory::PaymentInfo => "payment_info",
            PhiCategory::DeviceData => "device_data",
        });
        
        sqlx::query!(
            r#"
            INSERT INTO bi_hipaa_audit_log (id, user_id, action, phi_category, dataset_id, report_id, timestamp, ip_address, user_agent)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
            entry.id,
            entry.user_id,
            action,
            phi_category_str,
            entry.dataset_id,
            entry.report_id,
            entry.timestamp,
            entry.ip_address,
            entry.user_agent,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| super::super::application::compliance_management::ComplianceManagementError::AuditError(e.to_string()))?;
        
        Ok(())
    }
    
    async fn get_config(&self) -> Result<HipaaConfig, super::super::application::compliance_management::ComplianceManagementError> {
        let record = sqlx::query!(
            r#"
            SELECT encryption_enabled, audit_logging_enabled, access_control_enabled, data_retention_days, last_updated
            FROM bi_hipaa_config
            LIMIT 1
            "#
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| super::super::application::compliance_management::ComplianceManagementError::HipaaError(e.to_string()))?;
        
        if let Some(record) = record {
            Ok(HipaaConfig {
                encryption_enabled: record.encryption_enabled,
                audit_logging_enabled: record.audit_logging_enabled,
                access_control_enabled: record.access_control_enabled,
                data_retention_days: record.data_retention_days as u32,
                last_updated: record.last_updated,
            })
        } else {
            // Return default config if none exists
            Ok(HipaaConfig::default())
        }
    }
    
    async fn save_config(&self, config: &HipaaConfig) -> Result<(), super::super::application::compliance_management::ComplianceManagementError> {
        // First try to update existing config
        let result = sqlx::query!(
            r#"
            UPDATE bi_hipaa_config
            SET encryption_enabled = $1, audit_logging_enabled = $2, access_control_enabled = $3, data_retention_days = $4, last_updated = $5
            "#,
            config.encryption_enabled,
            config.audit_logging_enabled,
            config.access_control_enabled,
            config.data_retention_days as i32,
            config.last_updated,
        )
        .execute(&self.pool)
        .await;
        
        // If no rows were updated, insert a new config
        if let Ok(result) = result {
            if result.rows_affected() == 0 {
                sqlx::query!(
                    r#"
                    INSERT INTO bi_hipaa_config (encryption_enabled, audit_logging_enabled, access_control_enabled, data_retention_days, last_updated)
                    VALUES ($1, $2, $3, $4, $5)
                    "#,
                    config.encryption_enabled,
                    config.audit_logging_enabled,
                    config.access_control_enabled,
                    config.data_retention_days as i32,
                    config.last_updated,
                )
                .execute(&self.pool)
                .await
                .map_err(|e| super::super::application::compliance_management::ComplianceManagementError::HipaaError(e.to_string()))?;
            }
        } else {
            // If update failed, try insert
            sqlx::query!(
                r#"
                INSERT INTO bi_hipaa_config (encryption_enabled, audit_logging_enabled, access_control_enabled, data_retention_days, last_updated)
                VALUES ($1, $2, $3, $4, $5)
                "#,
                config.encryption_enabled,
                config.audit_logging_enabled,
                config.access_control_enabled,
                config.data_retention_days as i32,
                config.last_updated,
            )
            .execute(&self.pool)
            .await
            .map_err(|e| super::super::application::compliance_management::ComplianceManagementError::HipaaError(e.to_string()))?;
        }
        
        Ok(())
    }
}