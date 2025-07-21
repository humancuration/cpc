use super::*;
use jni::{objects::JObject, sys::jobject, JNIEnv};
use crate::bevy::BevyEngine;
use crate::events::{EventSystem, P2PEvent, UIEvent};

// EngineBridge implementation for Android
pub struct AndroidEngineBridge;

impl EngineBridge for AndroidEngineBridge {
    fn handle_system_command(&self, command: &str, params: &[String]) {
        // Forward system commands to Bevy
        let engine = BevyEngine::get_instance();
        engine.handle_system_command(command, params);
    }

    fn receive_texture(&self, texture: TextureData) {
        // Handle texture received from native UI
        let engine = BevyEngine::get_instance();
        engine.receive_texture(texture);
    }
}

// Global engine bridge instance
pub static ANDROID_ENGINE_BRIDGE: AndroidEngineBridge = AndroidEngineBridge;

#[no_mangle]
pub extern "system" fn Java_com_cpc_NativeBridge_requestUI(
    env: JNIEnv,
    _: JClass,
    component: JString,
    props: JString
) -> jobject {
    // Convert Java strings to Rust types
    let component_str: String = env.get_string(component).unwrap().into();
    let props_str: String = env.get_string(props).unwrap().into();
    let props_value: serde_json::Value = serde_json::from_str(&props_str).unwrap();
    
    // Get Bevy engine instance and render UI
    let engine = BevyEngine::get_instance();
    let texture = engine.render_ui(&component_str, props_value);
    
    // Convert TextureData to Android Bitmap
    let bitmap_class = env.find_class("android/graphics/Bitmap").unwrap();
    let create_bitmap = env.get_static_method_id(
        bitmap_class,
        "createBitmap",
        "(IILandroid/graphics/Bitmap$Config;)Landroid/graphics/Bitmap;"
    ).unwrap();
    
    let config_class = env.find_class("android/graphics/Bitmap$Config").unwrap();
    let argb_8888 = env.get_static_field_id(
        config_class,
        "ARGB_8888",
        "Landroid/graphics/Bitmap$Config;"
    ).unwrap();
    let config = env.get_static_field(config_class, argb_8888).unwrap();
    
    let bitmap = env.call_static_method(
        bitmap_class,
        create_bitmap,
        &[
            (texture.width as i32).into(),
            (texture.height as i32).into(),
            config.into()
        ]
    ).unwrap().l().unwrap();
    
    // Copy pixel data to Bitmap
    let pixel_buffer = env.new_direct_byte_buffer(texture.data.as_slice()).unwrap();
    let _ = env.call_method(
        bitmap,
        "copyPixelsFromBuffer",
        "(Ljava/nio/Buffer;)V",
        &[pixel_buffer.into()]
    );
    
    bitmap.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_cpc_NativeBridge_forwardUIEvent(
    env: JNIEnv,
    _: JClass,
    event_json: JString
) {
    let event_str: String = env.get_string(event_json).unwrap().into();
    let event: UIEvent = serde_json::from_str(&event_str).unwrap();
    
    // Convert to P2P event format
    let p2p_event = P2PEvent::from_ui_event(event);
    
    // Send to event system
    let event_system = EventSystem::get_instance();
    event_system.handle_incoming_event(p2p_event);
}

#[no_mangle]
pub extern "system" fn Java_com_cpc_NativeBridge_sendGameEvent(
    env: JNIEnv,
    _: JClass,
    event_type: JString,
    data: JString
) {
    // Convert Java strings to Rust types
    let event_type_str: String = env.get_string(event_type).unwrap().into();
    let data_str: String = env.get_string(data).unwrap().into();
    let data_value: serde_json::Value = serde_json::from_str(&data_str).unwrap();
    
    // Create UIEvent
    let ui_event = UIEvent {
        component: "game".to_string(),
        action: event_type_str,
        data: data_value,
    };
    
    // Convert to P2P event format
    let p2p_event = P2PEvent::from_ui_event(ui_event);
    
    // Send to event system
    let event_system = EventSystem::get_instance();
    event_system.handle_incoming_event(p2p_event);
}

#[no_mangle]
#[no_mangle]
pub extern "system" fn Java_com_cpc_NativeBridge_initBevySurface(
    env: JNIEnv,
    _: JClass,
    surface: jobject
) {
    // Get native surface pointer
    let surface_ptr = env.get_direct_buffer_address(surface).unwrap();
    let engine = BevyEngine::get_instance();
    engine.init_surface(surface_ptr as *mut c_void);
}

#[no_mangle]
pub extern "system" fn Java_com_cpc_NativeBridge_sendTextureToEngine(
    env: JNIEnv,
    _: JClass,
    bitmap: JObject
) {
    // Get bitmap info
    let width = env.call_method(bitmap, "getWidth", "()I", &[]).unwrap().i().unwrap();
    let height = env.call_method(bitmap, "getHeight", "()I", &[]).unwrap().i().unwrap();
    
    // Get bitmap pixels
    let buffer = env.new_direct_byte_buffer(&mut vec![0u8; (width * height * 4) as usize]).unwrap();
    env.call_method(bitmap, "copyPixelsToBuffer", "(Ljava/nio/Buffer;)V", &[buffer.into()]).unwrap();
    
    // Create TextureData
    let texture = TextureData {
        width: width as u32,
        height: height as u32,
        data: buffer.into_raw().as_slice().to_vec(),
    };
    
    // Send to engine
    ANDROID_ENGINE_BRIDGE.receive_texture(texture);
}