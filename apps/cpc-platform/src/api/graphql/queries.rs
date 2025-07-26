use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBusinessMetrics {
    pub total_revenue: f64,
    pub revenue_growth: f64,
    pub impact_score: f64,
    pub active_projects: usize,
    pub community_investment: f64,
    pub investment_growth: f64,
    pub monthly_metrics: Vec<(String, f64)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessMetricsResponse {
    pub data: GetBusinessMetrics,
}

pub async fn get_business_metrics(user_id: String) -> Result<GetBusinessMetrics, String> {
    // Mock implementation - replace with actual GraphQL query
    Ok(GetBusinessMetrics {
        total_revenue: 125_000.0,
        revenue_growth: 15.3,
        impact_score: 8.7,
        active_projects: 12,
        community_investment: 45_000.0,
        investment_growth: 22.1,
        monthly_metrics: vec![
            ("January".to_string(), 10_000.0),
            ("February".to_string(), 12_500.0),
            ("March".to_string(), 15_000.0),
            ("April".to_string(), 13_200.0),
            ("May".to_string(), 16_800.0),
            ("June".to_string(), 18_500.0),
        ],
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFeatureFlags {
    pub business_tools_enabled: bool,
    pub advanced_analytics: bool,
    pub export_enabled: bool,
}

pub async fn get_feature_flags() -> Result<GetFeatureFlags, String> {
    // Mock implementation - replace with actual GraphQL query
    Ok(GetFeatureFlags {
        business_tools_enabled: true,
        advanced_analytics: true,
        export_enabled: true,
    })
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

pub async fn export_business_data(format: &str) -> Result<String, String> {
    let args = js_sys::Object::new();
    js_sys::Reflect::set(&args, &"format".into(), &format.into()).unwrap();
    
    let result = invoke("export_business_data", args.into()).await;
    result.as_string().ok_or_else(|| "Export failed".to_string())
}
