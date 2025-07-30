//! Web routes for the sheets application

use axum::{
    routing::{get, post, put, delete},
    Router,
};
use std::sync::Arc;

/// Create the web routes for the sheets application
pub fn create_routes() -> Router {
    Router::new()
        .route("/sheets", post(create_sheet))
        .route("/sheets/:id", get(get_sheet).put(update_sheet).delete(delete_sheet))
        .route("/sheets/:id/cells/:row/:col", put(update_cell))
        .route("/sheets/:id/formulas/:row/:col", put(update_formula))
        .route("/sheets/:id/charts", post(add_chart).get(get_charts))
        .route("/sheets/:id/charts/:chart_id", delete(remove_chart))
        .route("/sheets/:id/permissions", post(set_permission))
        .route("/sheets/user/:user_id", get(get_sheets_by_user))
        .route("/sheets/:id/import/expenses", post(import_expenses))
}

/// Create a new sheet
async fn create_sheet() -> String {
    "Create sheet endpoint".to_string()
}

/// Get a sheet by ID
async fn get_sheet() -> String {
    "Get sheet endpoint".to_string()
}

/// Update a sheet
async fn update_sheet() -> String {
    "Update sheet endpoint".to_string()
}

/// Delete a sheet
async fn delete_sheet() -> String {
    "Delete sheet endpoint".to_string()
}

/// Update a cell in a sheet
async fn update_cell() -> String {
    "Update cell endpoint".to_string()
}

/// Update a formula in a sheet
async fn update_formula() -> String {
    "Update formula endpoint".to_string()
}

/// Add a chart to a sheet
async fn add_chart() -> String {
    "Add chart endpoint".to_string()
}

/// Get charts from a sheet
async fn get_charts() -> String {
    "Get charts endpoint".to_string()
}

/// Remove a chart from a sheet
async fn remove_chart() -> String {
    "Remove chart endpoint".to_string()
}

/// Set permission for a user on a sheet
async fn set_permission() -> String {
    "Set permission endpoint".to_string()
}

/// Get sheets by user
async fn get_sheets_by_user() -> String {
    "Get sheets by user endpoint".to_string()
}

/// Import expenses from a sheet
async fn import_expenses() -> String {
    "Import expenses endpoint".to_string()
}