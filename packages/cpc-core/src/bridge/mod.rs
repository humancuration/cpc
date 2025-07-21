use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum BridgeMessage {
    RenderUI {
        component: String,
        props: serde_json::Value,
    },
    GameEvent {
        event_type: String,
        data: serde_json::Value,
    },
    SystemCommand {
        command: String,
        parameters: Vec<String>,
    },
}

#[derive(Serialize, Deserialize)]
pub struct TextureData {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>, // RGBA format
}

pub trait NativeBridge {
    fn request_ui(&self, component: &str, props: serde_json::Value) -> TextureData;
    fn send_game_event(&self, event_type: &str, data: serde_json::Value);
}

pub trait EngineBridge {
    fn handle_system_command(&self, command: &str, params: &[String]);
    fn receive_texture(&self, texture: TextureData);
}

// Platform-specific modules
#[cfg(target_os = "android")]
pub mod android;

#[cfg(target_family = "wasm")]
pub mod web;

#[cfg(not(any(target_os = "android", target_family = "wasm")))]
pub mod desktop;