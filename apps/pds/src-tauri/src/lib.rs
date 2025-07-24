pub mod camera;
pub mod vision;
pub mod impact;
pub mod secure_storage;
pub mod bevy_integration;

#[cfg(test)]
mod vision_test;

use std::sync::Arc;
use tokio::sync::Mutex;
use tauri::Manager;

/// Shared camera state
pub type CameraState = camera::CameraState;
/// Shared vision state
pub type VisionState = vision::VisionState;
/// Shared Bevy state
pub type BevyState = Arc<Mutex<Option<bevy_integration::BevyBridge>>>;

/// Setup function for Tauri app
pub fn setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize camera state
    let camera_state = Arc::new(Mutex::new(camera::CameraState::default()));
    
    // Initialize vision state
    let vision_state = Arc::new(Mutex::new(vision::VisionState::default()));
    
    // Initialize Bevy state
    let bevy_state = Arc::new(Mutex::new(None));
    
    // Manage states
    app.manage(camera_state);
    app.manage(vision_state);
    app.manage(bevy_state);
    
    Ok(())
}

/// Register Tauri commands
#[macro_export]
macro_rules! register_commands {
    () => {
        tauri::generate_handler![
            // Camera commands
            camera::start_camera_capture,
            camera::stop_camera_capture,
            camera::get_latest_frame,
            // Vision commands
            vision::initialize_vision,
            vision::recognize_image,
            vision::recognize_batch,
            vision::get_available_models,
            vision::process_camera_image,
            vision::process_image_path,
            vision::get_model_info,
            vision::load_model,
            vision::unload_model,
            // Impact commands
            impact::get_impact_report,
            impact::generate_impact_report,
            impact::clear_impact_report,
            // Secure storage commands
            secure_storage::secure_store,
            secure_storage::secure_retrieve,
            secure_storage::secure_delete,
            secure_storage::secure_list_keys,
            // Bevy commands
            bevy_commands::initialize_bevy,
            bevy_commands::send_to_bevy,
            bevy_commands::control_bevy,
            bevy_commands::is_bevy_running,
            bevy_commands::get_bevy_status,
        ]
    };
}