//! GraphQL schema for the sheets application

use async_graphql::{Object, Schema, EmptyMutation, EmptySubscription, Context};
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

/// GraphQL schema
pub type SheetsSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

/// Create the GraphQL schema
pub fn create_schema() -> SheetsSchema {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish()
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