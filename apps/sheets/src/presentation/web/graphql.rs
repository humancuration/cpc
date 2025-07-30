//! GraphQL schema for the sheets application

use async_graphql::{Object, Schema, EmptyMutation, EmptySubscription, Context, InputObject};
use uuid::Uuid;

/// GraphQL query root
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Get a sheet by ID
    async fn sheet(&self, ctx: &Context<'_>, id: Uuid) -> async_graphql::Result<SheetDto> {
        // In a real implementation, this would call the sheet service
        Ok(SheetDto {
            id,
            name: "Sample Sheet".to_string(),
            owner: Uuid::nil(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            version: 1,
        })
    }
    
    /// Get all sheets owned by a user
    async fn sheets_by_owner(&self, ctx: &Context<'_>, owner_id: Uuid) -> async_graphql::Result<Vec<SheetDto>> {
        // In a real implementation, this would call the sheet service
        Ok(vec![SheetDto {
            id: Uuid::new_v4(),
            name: "Sample Sheet".to_string(),
            owner: owner_id,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            version: 1,
        }])
    }
}

/// GraphQL mutation root
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Create a chart from cell data
    async fn create_chart(
        &self,
        ctx: &Context<'_>,
        sheet_id: Uuid,
        chart_type: ChartTypeDto,
        data_range: String,
        title: Option<String>,
        context: VisualizationContextInput,
    ) -> async_graphql::Result<ChartDto> {
        // In a real implementation, this would call the chart service
        Ok(ChartDto {
            id: Uuid::new_v4(),
            title: title.unwrap_or("New Chart".to_string()),
            chart_type: format!("{:?}", chart_type),
        })
    }
}

/// GraphQL schema
pub type SheetsSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

/// Create the GraphQL schema
pub fn create_schema() -> SheetsSchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription).finish()
}

/// Data transfer object for Sheet
#[derive(async_graphql::SimpleObject)]
pub struct SheetDto {
    pub id: Uuid,
    pub name: String,
    pub owner: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub version: u64,
}

/// Data transfer object for Cell
#[derive(async_graphql::SimpleObject)]
pub struct CellDto {
    pub row: u32,
    pub column: u32,
    pub value: String,
    pub formatted_value: String,
}

/// Data transfer object for Chart
#[derive(async_graphql::SimpleObject)]
pub struct ChartDto {
    pub id: Uuid,
    pub title: String,
    pub chart_type: String,
}

/// GraphQL chart type enum
#[derive(async_graphql::Enum, Clone, Copy, PartialEq, Eq)]
pub enum ChartTypeDto {
    Bar,
    Line,
    Pie,
    Scatter,
}

/// GraphQL input for visualization context
#[derive(InputObject)]
pub struct VisualizationContextInput {
    pub sharing_scope: SharingScopeDto,
    pub accessibility_mode: AccessibilityModeDto,
    pub lod_level: i32,
}

/// GraphQL sharing scope enum
#[derive(async_graphql::Enum, Clone, Copy, PartialEq, Eq)]
pub enum SharingScopeDto {
    Public,
    Team,
    Private,
}

/// GraphQL accessibility mode enum
#[derive(async_graphql::Enum, Clone, Copy, PartialEq, Eq)]
pub enum AccessibilityModeDto {
    Standard,
    HighContrast,
    ScreenReader,
    KeyboardNavigation,
}