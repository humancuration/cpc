use wasm_bindgen::prelude::*;
use crate::types::AccountingDashboard;
use crate::services::api::ApiClient;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

pub struct AccountingService;

impl AccountingService {
    pub fn new() -> Self {
        Self
    }

    pub async fn get_dashboard_data() -> Result<AccountingDashboard, String> {
        let args = JsValue::NULL;
        let result = invoke("get_accounting_dashboard", args).await;
        
        let json_str = result.as_string().ok_or("Invalid response format")?;
        let dashboard: AccountingDashboard = serde_json::from_str(&json_str)
            .map_err(|e| format!("Failed to parse dashboard data: {}", e))?;
        
        Ok(dashboard)
    }

    pub async fn get_income_statement(period: &str) -> Result<String, String> {
        let args = js_sys::Object::new();
        js_sys::Reflect::set(&args, &"period".into(), &period.into()).unwrap();
        
        let result = invoke("get_income_statement", args.into()).await;
        result.as_string().ok_or("Invalid response format".to_string())
    }

    pub async fn get_balance_sheet(date: &str) -> Result<String, String> {
        let args = js_sys::Object::new();
        js_sys::Reflect::set(&args, &"date".into(), &date.into()).unwrap();
        
        let result = invoke("get_balance_sheet", args.into()).await;
        result.as_string().ok_or("Invalid response format".to_string())
    }

    pub async fn get_cash_flow_statement(period: &str) -> Result<String, String> {
        let args = js_sys::Object::new();
        js_sys::Reflect::set(&args, &"period".into(), &period.into()).unwrap();
        
        let result = invoke("get_cash_flow_statement", args.into()).await;
        result.as_string().ok_or("Invalid response format".to_string())
    }
}

// Create a global instance
pub static ACCOUNTING_SERVICE: AccountingService = AccountingService;

// Convenience functions
pub async fn get_dashboard_data() -> Result<AccountingDashboard, String> {
    ACCOUNTING_SERVICE.get_dashboard_data().await
}

pub async fn get_income_statement(period: &str) -> Result<String, String> {
    ACCOUNTING_SERVICE.get_income_statement(period).await
}

pub async fn get_balance_sheet(date: &str) -> Result<String, String> {
    ACCOUNTING_SERVICE.get_balance_sheet(date).await
}

pub async fn get_cash_flow_statement(period: &str) -> Result<String, String> {
    ACCOUNTING_SERVICE.get_cash_flow_statement(period).await
}