//! GraphQL implementation for the BI & Analytics module

use async_graphql::{
    Schema, EmptySubscription, Object, Result, Context, SimpleObject, InputObject, Enum,
    connection::{Connection, EmptyFields},
};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde_json::Value as JsonValue;
use crate::{
    domain::{
        dataset::{Dataset, DataSource, FieldDefinition, DataType},
        report::{Report, VisualizationType},
        dashboard::{Dashboard, DashboardReport, GridPosition},
    },
    application::{
        data_ingestion::DataIngestionService,
        report_generation::ReportGenerationService,
        dashboard_management::DashboardManagementService,
        compliance_management::ComplianceManagementService,
    },
    infrastructure::{
        postgres_repository::PostgresBiRepository,
        p2p_data_source::P2PDataSource,
    },
};
use tracing::{info, error};

/// GraphQL schema for BI & Analytics
pub type BiAnalyticsGraphQLSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

/// Create a new GraphQL schema
pub fn create_schema(
    data_ingestion_service: DataIngestionService<PostgresBiRepository, P2PDataSource>,
    report_generation_service: ReportGenerationService<PostgresBiRepository, PostgresBiRepository>,
    dashboard_management_service: DashboardManagementService<PostgresBiRepository, PostgresBiRepository>,
    compliance_management_service: ComplianceManagementService<PostgresBiRepository, PostgresBiRepository>,
) -> BiAnalyticsGraphQLSchema {
    Schema::build(
        QueryRoot {
            data_ingestion_service,
            report_generation_service,
            dashboard_management_service,
            compliance_management_service,
        },
        MutationRoot {
            data_ingestion_service,
            report_generation_service,
            dashboard_management_service,
            compliance_management_service,
        },
        EmptySubscription,
    )
    .finish()
}

/// Root query object
pub struct QueryRoot {
    data_ingestion_service: DataIngestionService<PostgresBiRepository, P2PDataSource>,
    report_generation_service: ReportGenerationService<PostgresBiRepository, PostgresBiRepository>,
    dashboard_management_service: DashboardManagementService<PostgresBiRepository, PostgresBiRepository>,
    compliance_management_service: ComplianceManagementService<PostgresBiRepository, PostgresBiRepository>,
}

/// Root mutation object
pub struct MutationRoot {
    data_ingestion_service: DataIngestionService<PostgresBiRepository, P2PDataSource>,
    report_generation_service: ReportGenerationService<PostgresBiRepository, PostgresBiRepository>,
    dashboard_management_service: DashboardManagementService<PostgresBiRepository, PostgresBiRepository>,
    compliance_management_service: ComplianceManagementService<PostgresBiRepository, PostgresBiRepository>,
}

/// GraphQL representation of a dataset
#[derive(SimpleObject)]
pub struct DatasetObject {
    id: Uuid,
    name: String,
    source: String,
    fields: Vec<FieldDefinitionObject>,
    description: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

/// GraphQL representation of a field definition
#[derive(SimpleObject)]
pub struct FieldDefinitionObject {
    name: String,
    data_type: String,
    is_nullable: bool,
    description: Option<String>,
}

/// Convert domain Dataset to GraphQL DatasetObject
impl From<Dataset> for DatasetObject {
    fn from(dataset: Dataset) -> Self {
        Self {
            id: dataset.id,
            name: dataset.name,
            source: match dataset.source {
                DataSource::Crm => "crm".to_string(),
                DataSource::Finance => "finance".to_string(),
                DataSource::Calendar => "calendar".to_string(),
                DataSource::Messenger => "messenger".to_string(),
                DataSource::Custom(source) => format!("custom:{}", source),
            },
            fields: dataset.fields.into_iter().map(|f| f.into()).collect(),
            description: dataset.description,
            created_at: dataset.created_at,
            updated_at: dataset.updated_at,
        }
    }
}

/// Convert domain FieldDefinition to GraphQL FieldDefinitionObject
impl From<FieldDefinition> for FieldDefinitionObject {
    fn from(field: FieldDefinition) -> Self {
        Self {
            name: field.name,
            data_type: match field.data_type {
                DataType::String => "string".to_string(),
                DataType::Integer => "integer".to_string(),
                DataType::Float => "float".to_string(),
                DataType::Boolean => "boolean".to_string(),
                DataType::DateTime => "datetime".to_string(),
            },
            is_nullable: field.is_nullable,
            description: field.description,
        }
    }
}

/// GraphQL representation of a report
#[derive(SimpleObject)]
pub struct ReportObject {
    id: Uuid,
    dataset_id: Uuid,
    name: String,
    description: Option<String>,
    query: String,
    visualization_type: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

/// Convert domain Report to GraphQL ReportObject
impl From<Report> for ReportObject {
    fn from(report: Report) -> Self {
        Self {
            id: report.id,
            dataset_id: report.dataset_id,
            name: report.name,
            description: report.description,
            query: report.query,
            visualization_type: match report.visualization_type {
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
        }
    }
}

/// GraphQL representation of a dashboard
#[derive(SimpleObject)]
pub struct DashboardObject {
    id: Uuid,
    name: String,
    description: Option<String>,
    layout: JsonValue,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

/// Convert domain Dashboard to GraphQL DashboardObject
impl From<Dashboard> for DashboardObject {
    fn from(dashboard: Dashboard) -> Self {
        Self {
            id: dashboard.id,
            name: dashboard.name,
            description: dashboard.description,
            layout: serde_json::to_value(&dashboard.layout).unwrap_or(serde_json::Value::Object(serde_json::Map::new())),
            created_at: dashboard.created_at,
            updated_at: dashboard.updated_at,
        }
    }
}

/// GraphQL representation of a dashboard report
#[derive(SimpleObject)]
pub struct DashboardReportObject {
    id: Uuid,
    dashboard_id: Uuid,
    report_id: Uuid,
    position: GridPositionObject,
}

/// GraphQL representation of grid position
#[derive(SimpleObject)]
pub struct GridPositionObject {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

/// Convert domain DashboardReport to GraphQL DashboardReportObject
impl From<DashboardReport> for DashboardReportObject {
    fn from(dashboard_report: DashboardReport) -> Self {
        Self {
            id: dashboard_report.id,
            dashboard_id: dashboard_report.dashboard_id,
            report_id: dashboard_report.report_id,
            position: dashboard_report.position.into(),
        }
    }
}

/// Convert domain GridPosition to GraphQL GridPositionObject
impl From<GridPosition> for GridPositionObject {
    fn from(position: GridPosition) -> Self {
        Self {
            x: position.x,
            y: position.y,
            width: position.width,
            height: position.height,
        }
    }
}

/// Input object for creating a dataset
#[derive(InputObject)]
pub struct CreateDatasetInput {
    name: String,
    source: String,
    fields: Vec<CreateFieldDefinitionInput>,
    description: Option<String>,
}

/// Input object for creating a field definition
#[derive(InputObject)]
pub struct CreateFieldDefinitionInput {
    name: String,
    data_type: String,
    is_nullable: bool,
    description: Option<String>,
}

/// Convert GraphQL CreateFieldDefinitionInput to domain FieldDefinition
impl TryFrom<CreateFieldDefinitionInput> for FieldDefinition {
    type Error = String;
    
    fn try_from(input: CreateFieldDefinitionInput) -> Result<Self, Self::Error> {
        let data_type = match input.data_type.as_str() {
            "string" => DataType::String,
            "integer" => DataType::Integer,
            "float" => DataType::Float,
            "boolean" => DataType::Boolean,
            "datetime" => DataType::DateTime,
            _ => return Err(format!("Invalid data type: {}", input.data_type)),
        };
        
        Ok(Self {
            name: input.name,
            data_type,
            is_nullable: input.is_nullable,
            description: input.description,
        })
    }
}

/// Convert GraphQL CreateDatasetInput to domain DataSource
impl TryFrom<String> for DataSource {
    type Error = String;
    
    fn try_from(source: String) -> Result<Self, Self::Error> {
        match source.as_str() {
            "crm" => Ok(DataSource::Crm),
            "finance" => Ok(DataSource::Finance),
            "calendar" => Ok(DataSource::Calendar),
            "messenger" => Ok(DataSource::Messenger),
            custom if custom.starts_with("custom:") => {
                Ok(DataSource::Custom(custom[7..].to_string()))
            }
            _ => Err(format!("Invalid data source: {}", source)),
        }
    }
}

/// Input object for creating a report
#[derive(InputObject)]
pub struct CreateReportInput {
    dataset_id: Uuid,
    name: String,
    query: String,
    visualization_type: String,
    description: Option<String>,
}

/// Convert GraphQL visualization type string to domain VisualizationType
impl TryFrom<String> for VisualizationType {
    type Error = String;
    
    fn try_from(visualization_type: String) -> Result<Self, Self::Error> {
        match visualization_type.as_str() {
            "table" => Ok(VisualizationType::Table),
            "bar_chart" => Ok(VisualizationType::BarChart),
            "line_chart" => Ok(VisualizationType::LineChart),
            "pie_chart" => Ok(VisualizationType::PieChart),
            "scatter_plot" => Ok(VisualizationType::ScatterPlot),
            "heatmap" => Ok(VisualizationType::Heatmap),
            "area_chart" => Ok(VisualizationType::AreaChart),
            _ => Err(format!("Invalid visualization type: {}", visualization_type)),
        }
    }
}

/// Input object for creating a dashboard
#[derive(InputObject)]
pub struct CreateDashboardInput {
    name: String,
    description: Option<String>,
    layout: JsonValue,
}

/// Input object for grid position
#[derive(InputObject)]
pub struct GridPositionInput {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

#[Object]
impl QueryRoot {
    /// Get a dataset by ID
    async fn dataset(&self, ctx: &Context<'_>, id: Uuid) -> Result<DatasetObject> {
        info!("Fetching dataset: {}", id);
        
        let dataset = self.data_ingestion_service.repository.get_dataset(id)
            .await
            .map_err(|e| {
                error!("Failed to fetch dataset {}: {}", id, e);
                async_graphql::Error::new(format!("Failed to fetch dataset: {}", e))
            })?;
        
        Ok(dataset.into())
    }
    
    /// Get a report by ID
    async fn report(&self, ctx: &Context<'_>, id: Uuid) -> Result<ReportObject> {
        info!("Fetching report: {}", id);
        
        let report = self.report_generation_service.report_repository.get_report(id)
            .await
            .map_err(|e| {
                error!("Failed to fetch report {}: {}", id, e);
                async_graphql::Error::new(format!("Failed to fetch report: {}", e))
            })?;
        
        Ok(report.into())
    }
    
    /// Get dashboards by user ID
    async fn dashboards(&self, ctx: &Context<'_>, user_id: Uuid) -> Result<Vec<DashboardObject>> {
        info!("Fetching dashboards for user: {}", user_id);
        
        let dashboards = self.dashboard_management_service.dashboard_repository.get_dashboards_by_owner(user_id)
            .await
            .map_err(|e| {
                error!("Failed to fetch dashboards for user {}: {}", user_id, e);
                async_graphql::Error::new(format!("Failed to fetch dashboards: {}", e))
            })?;
        
        Ok(dashboards.into_iter().map(|d| d.into()).collect())
    }
}

#[Object]
impl MutationRoot {
    /// Create a new dataset
    async fn create_dataset(&self, ctx: &Context<'_>, input: CreateDatasetInput) -> Result<DatasetObject> {
        info!("Creating dataset: {}", input.name);
        
        let fields: Result<Vec<FieldDefinition>, _> = input.fields
            .into_iter()
            .map(FieldDefinition::try_from)
            .collect();
        let fields = fields.map_err(|e| async_graphql::Error::new(format!("Invalid field definition: {}", e)))?;
        
        let source = DataSource::try_from(input.source)
            .map_err(|e| async_graphql::Error::new(format!("Invalid data source: {}", e)))?;
        
        // In a real implementation, we would get the owner ID from the context (authenticated user)
        let owner_id = Uuid::new_v4(); // Placeholder
        
        let dataset = self.data_ingestion_service.create_dataset(
            input.name,
            source,
            fields,
            owner_id,
            input.description,
        ).await
        .map_err(|e| {
            error!("Failed to create dataset: {}", e);
            async_graphql::Error::new(format!("Failed to create dataset: {}", e))
        })?;
        
        Ok(dataset.into())
    }
    
    /// Create a new report
    async fn create_report(&self, ctx: &Context<'_>, input: CreateReportInput) -> Result<ReportObject> {
        info!("Creating report: {}", input.name);
        
        let visualization_type = VisualizationType::try_from(input.visualization_type)
            .map_err(|e| async_graphql::Error::new(format!("Invalid visualization type: {}", e)))?;
        
        // In a real implementation, we would get the owner ID from the context (authenticated user)
        let owner_id = Uuid::new_v4(); // Placeholder
        
        let report = self.report_generation_service.create_report(
            input.dataset_id,
            input.name,
            input.query,
            visualization_type,
            owner_id,
            input.description,
        ).await
        .map_err(|e| {
            error!("Failed to create report: {}", e);
            async_graphql::Error::new(format!("Failed to create report: {}", e))
        })?;
        
        Ok(report.into())
    }
    
    /// Create a new dashboard
    async fn create_dashboard(&self, ctx: &Context<'_>, input: CreateDashboardInput) -> Result<DashboardObject> {
        info!("Creating dashboard: {}", input.name);
        
        let layout: std::collections::HashMap<String, JsonValue> = serde_json::from_value(input.layout)
            .map_err(|e| async_graphql::Error::new(format!("Invalid layout JSON: {}", e)))?;
        
        // In a real implementation, we would get the owner ID from the context (authenticated user)
        let owner_id = Uuid::new_v4(); // Placeholder
        
        let dashboard = self.dashboard_management_service.create_dashboard(
            input.name,
            owner_id,
            input.description,
            layout,
        ).await
        .map_err(|e| {
            error!("Failed to create dashboard: {}", e);
            async_graphql::Error::new(format!("Failed to create dashboard: {}", e))
        })?;
        
        Ok(dashboard.into())
    }
}