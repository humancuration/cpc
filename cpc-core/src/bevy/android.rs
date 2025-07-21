use jni::{objects::JObject, sys::jlong, JNIEnv};
use bevy::prelude::*;
use winit::platform::android::EventLoopExtAndroid;
use crate::bridge::android::ANDROID_ENGINE_BRIDGE;

#[no_mangle]
pub extern "system" fn Java_com_cpc_BevyActivity_create_engine(
    env: JNIEnv,
    _: JClass,
    surface: JObject
) -> jlong {
    // Initialize Bevy and pass native surface
    let mut app = App::new();
    
    // Configure Bevy for Android
    app.add_plugins(DefaultPlugins
        .set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some(surface.into_raw() as _),
                ..default()
            }),
            ..default()
        })
    );
    
    // Register Android engine bridge
    app.insert_resource(ANDROID_ENGINE_BRIDGE);
    
    // Store the app in a Box and return as raw pointer
    Box::into_raw(Box::new(app)) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_cpc_BevyActivity_run_engine(
    _: JNIEnv,
    _: JClass,
    engine_ptr: jlong
) {
    let app = unsafe { &mut *(engine_ptr as *mut App) };
    app.run();
}

// Demo system for native UI integration
pub fn native_ui_demo_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    engine_bridge: Res<AndroidEngineBridge>
) {
    // Example: Spawn native UI texture as sprite
    // This would be replaced with actual texture from bridge
    commands.spawn(SpriteBundle {
        material: materials.add(ColorMaterial::color(Color::RED)),
        sprite: Sprite::new(Vec2::new(100.0, 100.0)),
        ..Default::default()
    });
    
    // Example: Send game event to native UI
    engine_bridge.send_game_event(
        "player_moved",
        serde_json::json!({"x": 10, "y": 20})
    );
}