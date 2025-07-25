use wasm_bindgen::prelude::*;
use crate::types::impact::{ImpactDashboardData, ImpactReportRequest};
use crate::services::api::ApiClient;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

pub struct ImpactService;

impl ImpactService {
    pub fn new() -> Self {
        Self
    }

    pub async fn get_impact_report(org_id: &str, year: i32) -> Result<ImpactDashboardData, String> {
        let args = js_sys::Object::new();
        js_sys::Reflect::set(&args, &"org_id".into(), &org_id.into()).unwrap();
        js_sys::Reflect::set(&args, &"year".into(), &year.into()).unwrap();
        
        let result = invoke("get_impact_report", args.into()).await;
        
        let json_str = result.as_string().ok_or("Invalid response format")?;
        let report: ImpactDashboardData = serde_json::from_str(&json_str)
            .map_err(|e| format!("Failed to parse impact report: {}", e))?;
        
        Ok(report)
    }

    pub async fn get_carbon_footprint(org_id: &str, year: i32) -> Result<f64, String> {
        let args = js_sys::Object::new();
        js_sys::Reflect::set(&args, &"org_id".into(), &org_id.into()).unwrap();
        js_sys::Reflect::set(&args, &"year".into(), &year.into()).unwrap();
        
        let result = invoke("get_carbon_footprint", args.into()).await;
        result.as_string()
            .ok_or("Invalid response format".to_string())
            .and_then(|s| s.parse().map_err(|_| "Failed to parse carbon footprint".to_string()))
    }

    pub async fn get_diversity_metrics(org_id: &str, year: i32) -> Result<String, String> {
        let args = js_sys::Object::new();
        js_sys::Reflect::set(&args, &"org_id".into(), &org_id.into()).unwrap();
        js_sys::Reflect::set(&args, &"year".into(), &year.into()).unwrap();
        
        let result = invoke("get_diversity_metrics", args.into()).await;
        result.as_string().ok_or("Invalid response format".to_string())
    }

    pub async fn get_supply_chain_score(org_id: &str, year: i32) -> Result<String, String> {
        let args = js_sys::Object::new();
        js_sys::Reflect::set(&args, &"org_id".into(), &org_id.into()).unwrap();
        js_sys::Reflect::set(&args, &"year".into(), &year.into()).unwrap();
        
        let result = invoke("get_supply_chain_score", args.into()).await;
        result.as_string().ok_or("Invalid response format".to_string())
    }
}

// Create a global instance
pub static IMPACT_SERVICE: ImpactService = ImpactService;

// New methods
impl ImpactService {
    pub async fn recalculate_organization_impact(org_id: &str, year: i32) -> Result<(), String> {
        let args = js_sys::Object::new();
        js_sys::Reflect::set(&args, &"org_id".into(), &org_id.into()).unwrap();
        js_sys::Reflect::set(&args, &"year".into(), &year.into()).unwrap();
        
        let _ = invoke("recalculate_organization_impact", args.into()).await;
        Ok(())
    }

    pub async fn subscribe_to_impact_updates(org_id: &str, callback: js_sys::Function) -> Result<(), JsValue> {
        let args = js_sys::Object::new();
        js_sys::Reflect::set(&args, &"org_id".into(), &org_id.into()).unwrap();
        js_sys::Reflect::set(&args, &"callback".into(), &callback).unwrap();
        
        invoke("subscribe_to_impact_updates", args.into()).await;
        Ok(())
    }
}

// Convenience functions
pub async fn get_impact_report(org_id: &str, year: i32) -> Result<ImpactDashboardData, String> {
    IMPACT_SERVICE.get_impact_report(org_id, year).await
}

pub async fn recalculate_impact(org_id: &str, year: i32) -> Result<(), String> {
    ImpactService::recalculate_organization_impact(org_id, year).await
}

pub async fn subscribe_impact_updates(org_id: &str, callback: js_sys::Function) -> Result<(), JsValue> {
    ImpactService::subscribe_to_impact_updates(org_id, callback).await
}

pub async fn get_carbon_footprint(org_id: &str, year: i32) -> Result<f64, String> {
    IMPACT_SERVICE.get_carbon_footprint(org_id, year).await
}

pub async fn get_diversity_metrics(org_id: &str, year: i32) -> Result<String, String> {
    IMPACT_SERVICE.get_diversity_metrics(org_id, year).await
}

pub async fn get_supply_chain_score(org_id: &str, year: i32) -> Result<String, String> {
    IMPACT_SERVICE.get_supply_chain_score(org_id, year).await
}