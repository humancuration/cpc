//! Integration tests for the visualization functionality in the API Integration module

use cpc_api_integration::application::visualization::{
    cache::VisualizationCache,
    request::{VisualizationParameters, VisualizationRequest, RequestContext},
    response::{VisualizationResponse, VisualizationData, AccessibilityMetadata, ResponseMetadata},
};
use tempfile::tempdir;
use uuid::Uuid;
use visualization_context::{AccessibilityMode, SharingScope, VisualizationContext};

#[test]
fn test_visualization_context_serialization() {
    let user_id = Uuid::new_v4();
    let context = VisualizationContext::new(
        "dashboard".to_string(),
        user_id,
        SharingScope::Public,
        AccessibilityMode::ScreenReader,
        2,
    );
    
    let headers = context.to_headers();
    let deserialized = VisualizationContext::from_headers(&headers).unwrap();
    
    assert_eq!(deserialized.originating_app, context.originating_app);
    assert_eq!(deserialized.user_id, context.user_id);
    assert_eq!(deserialized.sharing_scope, context.sharing_scope);
    assert_eq!(deserialized.accessibility_mode, context.accessibility_mode);
    assert_eq!(deserialized.lod_level, context.lod_level);
}

#[test]
fn test_visualization_cache() {
    let temp_dir = tempdir().unwrap();
    let cache = VisualizationCache::new(temp_dir.path().to_str().unwrap()).unwrap();
    
    let key = "test-key";
    let data = b"test-data".to_vec();
    
    // Test set and get
    cache.set(key, data.clone(), std::time::Duration::from_secs(60)).unwrap();
    let retrieved = cache.get(key).unwrap();
    assert_eq!(retrieved, Some(data));
    
    // Test expiration
    cache.set(key, data.clone(), std::time::Duration::from_nanos(1)).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(10));
    let expired = cache.get(key).unwrap();
    assert_eq!(expired, None);
}

#[test]
fn test_visualization_request_response_structs() {
    let user_id = Uuid::new_v4();
    let viz_id = Uuid::new_v4();
    
    let request = VisualizationRequest {
        visualization_id: viz_id,
        parameters: VisualizationParameters {
            width: 800,
            height: 600,
            lod_level: 2,
            visualization_type: "bar_chart".to_string(),
            accessibility_mode: AccessibilityMode::Standard,
        },
        context: RequestContext {
            app_id: "dashboard".to_string(),
            user_id,
            sharing_scope: SharingScope::Public,
            session_token: "test-token".to_string(),
        },
    };
    
    let response = VisualizationResponse {
        visualization_data: VisualizationData::Scene3D {
            payload: serde_json::json!({"test": "data"}),
            accessibility: AccessibilityMetadata {
                alt_text: "Test visualization".to_string(),
                navigation_map: Default::default(),
                aria_properties: Default::default(),
            },
        },
        metadata: ResponseMetadata::default(),
    };
    
    assert_eq!(request.visualization_id, viz_id);
    assert_eq!(request.parameters.width, 800);
    
    match response.visualization_data {
        VisualizationData::Scene3D { payload, .. } => {
            assert_eq!(payload["test"], "data");
        }
        _ => panic!("Expected Scene3D variant"),
    }
}

#[test]
fn test_accessibility_adapters() {
    use cpc_api_integration::application::visualization::accessibility::{
        AccessibilityAdapter, AccessibilityAdapterFactory,
        dashboard::DashboardAdapter,
        reporting::ReportingAdapter,
        collaboration::CollaborationAdapter,
    };
    
    let user_id = Uuid::new_v4();
    let context = VisualizationContext::new(
        "dashboard".to_string(),
        user_id,
        SharingScope::Public,
        AccessibilityMode::Standard,
        2,
    );
    
    let base_metadata = AccessibilityMetadata {
        alt_text: "Base alt text".to_string(),
        navigation_map: Default::default(),
        aria_properties: Default::default(),
    };
    
    // Test dashboard adapter
    let dashboard_adapter = DashboardAdapter;
    let enhanced_metadata = dashboard_adapter.enhance_metadata(base_metadata.clone(), &context);
    assert!(enhanced_metadata.alt_text.contains("Dashboard"));
    
    // Test reporting adapter
    let reporting_adapter = ReportingAdapter;
    let enhanced_metadata = reporting_adapter.enhance_metadata(base_metadata.clone(), &context);
    assert!(enhanced_metadata.alt_text.contains("Report"));
    
    // Test collaboration adapter
    let collaboration_adapter = CollaborationAdapter;
    let enhanced_metadata = collaboration_adapter.enhance_metadata(base_metadata.clone(), &context);
    assert!(enhanced_metadata.alt_text.contains("Collaborative"));
    
    // Test factory
    let adapter = AccessibilityAdapterFactory::create_adapter("dashboard");
    let enhanced_metadata = adapter.enhance_metadata(base_metadata, &context);
    assert!(enhanced_metadata.alt_text.contains("Dashboard"));
}