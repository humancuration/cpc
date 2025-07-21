#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod asset_commands;
mod editor_core;
mod scene_commands;

use editor_core::{EditorCore, EditorState};
use tauri::Manager;
use cpc_core::p2p::NetworkHandler;
use std::sync::{Arc, Mutex};
use editor_core::scene::manager::SceneManager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Create network handler
            let config = r#"{ "bootstrap_node": "/ip4/127.0.0.1/tcp/8080" }"#.to_string();
            let network_handler = NetworkHandler::get_instance(config);
            network_handler.start();
            
            // Create editor core with network handler
            let editor_core = EditorCore::new(Arc::new(network_handler.clone()));
            
            // Create editor state
            let editor_state = EditorState::default();
            
            // Create scene manager
            let scene_manager = Arc::new(Mutex::new(SceneManager::new()));
            
            // Manage state
            app.manage(network_handler.clone());
            app.manage(editor_core);
            app.manage(editor_state);
            app.manage(scene_manager);
            
            // Initialize asset manager
            let asset_manager = AssetManager::new(network_handler.clone());
            app.manage(asset_manager);
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            editor_core::load_project,
            editor_core::save_project,
            editor_core::create_entity,
            editor_core::delete_entity,
            editor_core::get_editor_state,
            asset_commands::get_assets_in_path,
            asset_commands::import_asset,
            asset_commands::acquire_asset_lock,
            scene_commands::update_component,
            scene_commands::add_component,
            scene_commands::remove_component,
            scene_commands::undo,
            scene_commands::redo
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}