use crate::bevy_integration::{BevyBridge, BevyMessage};
use std::sync::{Arc, Mutex};
use tauri::{Runtime, State, Window};

// Shared Bevy state type
pub type SharedBevyState = Arc<Mutex<Option<BevyBridge>>>;

/// Initialize Bevy engine
#[tauri::command]
pub async fn initialize_bevy<R: Runtime>(
    window: Window<R>,
    state: State<'_, SharedBevyState>,
) -> Result<(), String> {
    let mut bridge = state.lock().unwrap();
    
    if bridge.is_none() {
        let mut new_bridge = BevyBridge::new();
        new_bridge.initialize(window);
        *bridge = Some(new_bridge);
        Ok(())
    } else {
        Err("Bevy already initialized".to_string())
    }
}

/// Send message to Bevy
#[tauri::command]
pub async fn send_to_bevy(
    message: String,
    state: State<'_, SharedBevyState>,
) -> Result<(), String> {
    let bridge = state.lock().unwrap();
    
    if let Some(bridge) = bridge.as_ref() {
        let sender = bridge.get_sender();
        sender.send(BevyMessage::Custom(message))
            .map_err(|e| format!("Failed to send message: {}", e))
    } else {
        Err("Bevy not initialized".to_string())
    }
}

/// Control Bevy lifecycle
#[tauri::command]
pub async fn control_bevy(
    action: String,
    state: State<'_, SharedBevyState>,
) -> Result<(), String> {
    let bridge = state.lock().unwrap();
    
    if let Some(bridge) = bridge.as_ref() {
        let sender = bridge.get_sender();
        let message = match action.as_str() {
            "pause" => BevyMessage::Pause,
            "resume" => BevyMessage::Resume,
            "stop" => BevyMessage::Stop,
            _ => return Err("Invalid action".to_string()),
        };
        
        sender.send(message)
            .map_err(|e| format!("Failed to send control message: {}", e))
    } else {
        Err("Bevy not initialized".to_string())
    }
}

/// Check if Bevy is running
#[tauri::command]
pub async fn is_bevy_running(state: State<'_, SharedBevyState>) -> Result<bool, String> {
    let bridge = state.lock().unwrap();
    
    match bridge.as_ref() {
        Some(bridge) => Ok(bridge.is_running()),
        None => Ok(false),
    }
}

/// Get Bevy status
#[tauri::command]
pub async fn get_bevy_status(state: State<'_, SharedBevyState>) -> Result<String, String> {
    let bridge = state.lock().unwrap();
    
    match bridge.as_ref() {
        Some(bridge) => {
            if bridge.is_running() {
                Ok("running".to_string())
            } else {
                Ok("paused".to_string())
            }
        }
        None => Ok("not_initialized".to_string()),
    }
}