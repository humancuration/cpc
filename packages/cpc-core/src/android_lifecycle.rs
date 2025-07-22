use jni::{objects::JClass, JNIEnv};
use crate::bevy::bevy_thread::{BevyThread, BevyThreadCommand};
use std::sync::OnceLock;

static BEVY_THREAD: OnceLock<BevyThread> = OnceLock::new();

/// Resource that holds the paths of essential textures that must be reloaded after a pause.
/// The game should populate this resource with the paths of textures that are essential and must be reloaded on resume.
#[derive(Resource, Default)]
pub struct TextureManifest {
    pub paths: Vec<String>,
}

#[no_mangle]
pub extern "system" fn Java_com_wtf_MainActivity_androidOnCreate(
    _env: JNIEnv,
    _: JClass
) {
    println!("CPC Core initialized on Android (com.wtf)");
    // Initialize the Bevy thread
    let bevy_thread = BevyThread::new();
    BEVY_THREAD.set(bevy_thread).unwrap();
}

#[no_mangle]
pub extern "system" fn Java_com_wtf_MainActivity_androidOnPause(
    _env: JNIEnv,
    _: JClass
) {
    println!("CPC Core paused");
    
    if let Some(bevy_thread) = BEVY_THREAD.get() {
        bevy_thread.send_command(BevyThreadCommand::Pause);
    }
    
    // Pause P2P networking
    pause_p2p_network();
}

// Placeholder functions - replace with actual P2P implementation
fn pause_p2p_network() {
    println!("P2P networking paused");
    // TODO: Implement actual pausing logic
}

fn resume_p2p_network() {
    println!("P2P networking resumed");
    // TODO: Implement actual resuming logic
}

#[no_mangle]
pub extern "system" fn Java_com_wtf_MainActivity_androidOnResume(
    _env: JNIEnv,
    _: JClass
) {
    println!("CPC Core resumed");
    
    if let Some(bevy_thread) = BEVY_THREAD.get() {
        bevy_thread.send_command(BevyThreadCommand::Resume);
    }
    
    // Resume P2P networking
    resume_p2p_network();
}

// New JNI functions for BevyView
#[no_mangle]
pub extern "system" fn Java_com_wtf_BevyView_initializeBevy(
    env: JNIEnv,
    _: JClass,
    surface: jobject
) {
    println!("Initializing Bevy thread with surface");
    
    if let Some(bevy_thread) = BEVY_THREAD.get() {
        // Create a global reference to the surface
        let global_surface = env.new_global_ref(surface).unwrap();
        bevy_thread.send_command(BevyThreadCommand::Initialize(global_surface));
    }
}

#[no_mangle]
pub extern "system" fn Java_com_wtf_BevyView_resizeBevy(
    _env: JNIEnv,
    _: JClass,
    width: jint,
    height: jint
) {
    println!("Resizing Bevy surface to {}x{}", width, height);
    
    if let Some(bevy_thread) = BEVY_THREAD.get() {
        bevy_thread.send_command(BevyThreadCommand::Resize(width as u32, height as u32));
    }
}

#[no_mangle]
pub extern "system" fn Java_com_wtf_BevyView_destroyBevy(
    _env: JNIEnv,
    _: JClass
) {
    println!("Destroying Bevy thread");
    
    if let Some(bevy_thread) = BEVY_THREAD.get() {
        bevy_thread.send_command(BevyThreadCommand::Destroy);
    }
}

#[no_mangle]
pub extern "system" fn Java_com_wtf_BevyView_androidPauseBevyThread(
    _env: JNIEnv,
    _: JClass
) {
    println!("Pausing Bevy thread");
    
    if let Some(bevy_thread) = BEVY_THREAD.get() {
        bevy_thread.send_command(BevyThreadCommand::Pause);
    }
}

#[no_mangle]
pub extern "system" fn Java_com_wtf_BevyView_androidResumeBevyThread(
    _env: JNIEnv,
    _: JClass
) {
    println!("Resuming Bevy thread");
    
    if let Some(bevy_thread) = BEVY_THREAD.get() {
        bevy_thread.send_command(BevyThreadCommand::Resume);
    }
}