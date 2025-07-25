use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use uuid::Uuid;
use cpc_core::accounting::PeriodType;
use crate::types::{ApiResponse, DashboardRequest};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

pub mod accounting;
pub mod barcode;
pub mod camera;

pub struct AccountingService;

impl AccountingService {
    pub async fn get_dashboard_data(
        org_id: Uuid,
        period: PeriodType,
    ) -> Result<cpc_core::accounting::dashboard::AccountingDashboard, String> {
        let request = DashboardRequest {
            org_id,
            period: period.to_string(),
        };
        
        let args = serde_wasm_bindgen::to_value(&request)
            .map_err(|e| format!("Failed to serialize request: {}", e))?;
        
        let result = invoke("get_accounting_dashboard", args).await;
        let response: ApiResponse<cpc_core::accounting::dashboard::AccountingDashboard> =
            serde_wasm_bindgen::from_value(result)
                .map_err(|e| format!("Failed to deserialize response: {}", e))?;
        
        match response.success {
            true => response.data.ok_or("No data returned".to_string()),
            false => Err(response.error.unwrap_or("Unknown error".to_string())),
        }
    }

    pub async fn refresh_dashboard_data(
        org_id: Uuid,
        period: PeriodType,
    ) -> Result<cpc_core::accounting::dashboard::AccountingDashboard, String> {
        let request = DashboardRequest {
            org_id,
            period: period.to_string(),
        };
        
        let args = serde_wasm_bindgen::to_value(&request)
            .map_err(|e| format!("Failed to serialize request: {}", e))?;
        
        let result = invoke("refresh_accounting_dashboard", args).await;
        let response: ApiResponse<cpc_core::accounting::dashboard::AccountingDashboard> =
            serde_wasm_bindgen::from_value(result)
                .map_err(|e| format!("Failed to deserialize response: {}", e))?;
        
        match response.success {
            true => response.data.ok_or("No data returned".to_string()),
            false => Err(response.error.unwrap_or("Unknown error".to_string())),
        }
    }
}