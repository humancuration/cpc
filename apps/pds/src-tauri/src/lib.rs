pub mod camera;
pub mod vision;
pub mod impact;
pub mod secure_storage;
pub mod bevy_integration;
pub mod bevy_commands;
pub mod auth_commands;
pub mod governance_commands;
pub mod social;
pub mod social_integration;

#[cfg(test)]
mod vision_test;

use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use tauri::Manager;
use cpc_core::services::identity::IdentityService;
use cpc_core::services::governance::GovernanceService;
use cpc_core::services::social::SocialFeaturesService;
use cpc_core::repositories::social::post_repository::PostRepository;
use cpc_core::repositories::social::relationship_repository::RelationshipRepository;

/// Shared camera state
pub type CameraState = camera::CameraState;
/// Shared vision state
pub type VisionState = vision::VisionState;
/// Shared Bevy state
pub type BevyState = Arc<Mutex<Option<bevy_integration::BevyBridge>>>;
/// Shared social service state
pub type SharedSocialService = Arc<RwLock<SocialFeaturesService>>;

/// Setup function for Tauri app
pub fn setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Initialize camera state
    let camera_state = Arc::new(Mutex::new(camera::CameraState::default()));
    
    // Initialize vision state
    let vision_state = Arc::new(Mutex::new(vision::VisionState::default()));
    
    // Initialize Bevy state
    let bevy_state = Arc::new(Mutex::new(None));
    
    // Initialize core services
    let identity_service = IdentityService::new();
    let governance_service = GovernanceService::new();
    let social_service = Arc::new(RwLock::new(SocialFeaturesService::new()));
    
    // Initialize repositories
    let post_repository = PostRepository::new();
    let relationship_repository = RelationshipRepository::new();
    
    // Manage states and services
    app.manage(camera_state);
    app.manage(vision_state);
    app.manage(bevy_state);
    app.manage(identity_service);
    app.manage(governance_service);
    app.manage(social_service);
    app.manage(post_repository);
    app.manage(relationship_repository);
    
    Ok(())
}

/// Register Tauri commands
#[macro_export]
macro_rules! register_commands {
    () => {
        tauri::generate_handler![
            // Authentication commands
            auth_commands::login,
            auth_commands::register,
            auth_commands::get_current_user,
            auth_commands::update_profile,
            auth_commands::logout,
            auth_commands::validate_token,
            auth_commands::change_password,
            // Social commands
            social::create_post,
            social::get_posts_by_user,
            social::get_timeline,
            social::update_post,
            social::delete_post,
            social::follow_user,
            social::unfollow_user,
            social::get_followers,
            social::get_following,
            social::search_users,
            // Social integration commands
            social_integration::share_experience,
            social_integration::invite_friend,
            social_integration::add_comment,
            social_integration::get_comments,
            social_integration::get_visible_experiences,
            // Governance commands
            governance_commands::create_proposal,
            governance_commands::get_active_proposals,
            governance_commands::get_proposal,
            governance_commands::cast_vote,
            governance_commands::get_user_votes,
            governance_commands::get_user_proposals,
            governance_commands::update_proposal_status,
            governance_commands::get_governance_stats,
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