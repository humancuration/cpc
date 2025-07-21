use jni::{objects::JClass, JNIEnv};
use crate::bevy::android::get_bevy_app;
use crate::p2p::android::pause_p2p_network;
use crate::p2p::android::resume_p2p_network;

static IS_PAUSED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

#[no_mangle]
pub extern "system" fn Java_com_cpc_CPCCore_onCreate(
    _env: JNIEnv,
    _: JClass
) {
    println!("CPC Core initialized on Android");
}

#[no_mangle]
pub extern "system" fn Java_com_cpc_CPCCore_onPause(
    _env: JNIEnv,
    _: JClass
) {
    IS_PAUSED.store(true, std::sync::atomic::Ordering::SeqCst);
    println!("CPC Core paused");
    
    // Suspend Bevy engine
    if let Some(app) = unsafe { get_bevy_app() } {
        // Clean up textures to reduce memory pressure
        println!("Cleaning up Bevy textures");
        // In a real implementation, we'd call app.world.resource_mut::<Assets<Texture>>().clear();
    }
    
    // Pause P2P networking
    pause_p2p_network();
}

#[no_mangle]
pub extern "system" fn Java_com_cpc_CPCCore_onResume(
    _env: JNIEnv,
    _: JClass
) {
    IS_PAUSED.store(false, std::sync::atomic::Ordering::SeqCst);
    println!("CPC Core resumed");
    
    // Resume P2P networking
    resume_p2p_network();
    
    // Resume Bevy engine
    if let Some(app) = unsafe { get_bevy_app() } {
        println!("Resuming Bevy engine");
        // In a real implementation, we'd reload necessary textures
    }
}