use bevy::prelude::*;
use crate::bridge::{EngineBridge, TextureData};

// Platform-specific modules
#[cfg(target_os = "android")]
pub mod android;

// Thread management module
pub mod bevy_thread;

pub struct CpcBevyPlugin;

impl Plugin for CpcBevyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_engine_messages);
        
        // Add our UI plugin
        app.add_plugin(CpcUIPlugin);
    }
}

fn handle_engine_messages(
    mut commands: Commands,
    mut game_events: EventReader<GameEvent>,
    mut system_commands: EventReader<SystemCommand>,
    asset_server: Res<AssetServer>,
) {
    for event in game_events.iter() {
        // Handle game events from native
        println!("Received game event: {:?}", event);
    }
    
    for command in system_commands.iter() {
        // Handle system commands from native
        println!("Executing system command: {} with params {:?}", command.command, command.parameters);
    }
}

/// Renders native UI components and returns them as textures
#[cfg(target_os = "android")]
pub fn render_ui(component_name: &str, props: serde_json::Value) -> Option<Texture> {
    // Delegate to Android-specific implementation
    android::render_ui_component(component_name, props)
}

pub struct CpcUIPlugin;

impl Plugin for CpcUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_ui_requests);
    }
}

fn handle_ui_requests(
    mut commands: Commands,
    mut ui_requests: EventReader<UIRequest>,
    asset_server: Res<AssetServer>,
) {
    for request in ui_requests.iter() {
        if let Some(texture) = render_ui(&request.component_name, request.props.clone()) {
            commands.spawn(SpriteBundle {
                texture: asset_server.add(texture),
                transform: Transform::from_xyz(request.position.x, request.position.y, 0.0),
                ..default()
            });
        }
    }
}

#[derive(Event)]
pub struct UIRequest {
    pub component_name: String,
    pub props: serde_json::Value,
    pub position: Vec2,
}

#[derive(Event)]
pub struct GameEvent {
    pub event_type: String,
    pub data: serde_json::Value,
}

#[derive(Event)]
pub struct SystemCommand {
    pub command: String,
    pub parameters: Vec<String>,
}

// Android-specific implementation
#[cfg(target_os = "android")]
impl CpcBevyPlugin {
    pub fn create_android_engine(surface: *mut std::ffi::c_void) -> *mut App {
        let mut app = App::new();
        
        // Configure Bevy for Android
        app.add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    canvas: Some(surface as _),
                    ..default()
                }),
                ..default()
            })
        );
        
        // Add our plugins
        app.add_plugin(Self);
        
        Box::into_raw(Box::new(app))
    }
    
    pub fn init_surface(&self, surface: *mut c_void) {
        // Reconfigure Bevy with the new surface
        let mut app = self.app.lock().unwrap();
        app.world.resource_mut::<WindowDescriptor>().canvas = Some(surface as _);
    }
}

// macOS-specific implementation
#[cfg(target_os = "macos")]
impl CpcBevyPlugin {
    pub fn stop_engine(app_ptr: *mut App) {
        unsafe {
            if !app_ptr.is_null() {
                let _ = Box::from_raw(app_ptr);
            }
        }
    }
}

#[cfg(target_os = "android")]
pub fn render_ui_component(component_name: &str, props: serde_json::Value) -> Option<Texture> {
    // This will call into the Android-specific bridge
    if let Some(texture_data) = crate::bridge::android::request_ui_texture(component_name, props) {
        Some(Texture::new(
            Extent3d {
                width: texture_data.width,
                height: texture_data.height,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            texture_data.data,
            TextureFormat::Rgba8UnormSrgb,
        ))
    } else {
        None
    }
}