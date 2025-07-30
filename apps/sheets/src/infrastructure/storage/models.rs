//! Database models for sheets application

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::domain::{Sheet, Cell, CellAddress, Formula, ChartSpec, PermissionLevel};

/// Database model for Sheet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SheetModel {
    pub id: Uuid,
    pub name: String,
    pub owner: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub version: u64,
}

impl From<Sheet> for SheetModel {
    fn from(sheet: Sheet) -> Self {
        Self {
            id: sheet.id,
            name: sheet.name,
            owner: sheet.owner,
            created_at: sheet.created_at,
            updated_at: sheet.updated_at,
            version: sheet.version,
        }
    }
}

impl From<SheetModel> for Sheet {
    fn from(model: SheetModel) -> Self {
        Self {
            id: model.id,
            name: model.name,
            owner: model.owner,
            created_at: model.created_at,
            updated_at: model.updated_at,
            version: model.version,
            cells: std::collections::HashMap::new(), // These would be loaded separately
            formulas: std::collections::HashMap::new(), // These would be loaded separately
            charts: Vec::new(), // These would be loaded separately
            permissions: std::collections::HashMap::new(), // These would be loaded separately
        }
    }
}

/// Database model for Cell
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellModel {
    pub sheet_id: Uuid,
    pub row: u32,
    pub column: u32,
    pub value: String, // Serialized CellValue
    pub formatted_value: String,
    pub style: String, // Serialized CellStyle
}

impl From<&Cell> for CellModel {
    fn from(cell: &Cell) -> Self {
        Self {
            sheet_id: Uuid::nil(), // This would be set when saving
            row: cell.address.row,
            column: cell.address.column,
            value: serde_json::to_string(&cell.value).unwrap_or_default(),
            formatted_value: cell.formatted_value.clone(),
            style: serde_json::to_string(&cell.style).unwrap_or_default(),
        }
    }
}

/// Database model for Formula
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormulaModel {
    pub sheet_id: Uuid,
    pub row: u32,
    pub column: u32,
    pub expression: String,
    pub dependencies: String, // Serialized Vec<CellAddress>
    pub last_evaluated: DateTime<Utc>,
    pub cache: Option<String>, // Serialized Option<CellValue>
}

impl From<&Formula> for FormulaModel {
    fn from(formula: &Formula) -> Self {
        Self {
            sheet_id: Uuid::nil(), // This would be set when saving
            row: 0, // This would be set when saving
            column: 0, // This would be set when saving
            expression: formula.expression.clone(),
            dependencies: serde_json::to_string(&formula.dependencies).unwrap_or_default(),
            last_evaluated: formula.last_evaluated,
            cache: formula.cache.as_ref().map(|v| serde_json::to_string(v).unwrap_or_default()),
        }
    }
}

/// Database model for ChartSpec
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartModel {
    pub id: Uuid,
    pub sheet_id: Uuid,
    pub title: String,
    pub chart_type: String, // Serialized ChartType
    pub data_range: String, // Serialized CellRange
    pub series_config: String, // Serialized Vec<SeriesConfig>
    pub options: String, // Serialized ChartOptions
}

impl From<&ChartSpec> for ChartModel {
    fn from(chart: &ChartSpec) -> Self {
        Self {
            id: chart.id,
            sheet_id: Uuid::nil(), // This would be set when saving
            title: chart.title.clone(),
            chart_type: serde_json::to_string(&chart.chart_type).unwrap_or_default(),
            data_range: serde_json::to_string(&chart.data_range).unwrap_or_default(),
            series_config: serde_json::to_string(&chart.series_config).unwrap_or_default(),
            options: serde_json::to_string(&chart.options).unwrap_or_default(),
        }
    }
}

/// Database model for Permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionModel {
    pub sheet_id: Uuid,
    pub user_id: Uuid,
    pub level: String, // Serialized PermissionLevel
}

impl From<(&Uuid, &Uuid, &PermissionLevel)> for PermissionModel {
    fn from((sheet_id, user_id, level): (&Uuid, &Uuid, &PermissionLevel)) -> Self {
        Self {
            sheet_id: *sheet_id,
            user_id: *user_id,
            level: serde_json::to_string(level).unwrap_or_default(),
        }
    }
}