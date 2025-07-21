use jni::{JNIEnv, objects::JClass};
use crate::p2p::NetworkHandler;
use crate::events::EventSystem;

#[no_mangle]
pub extern "system" fn Java_com_cpc_NativeBridge_startP2PService(
    env: JNIEnv,
    _: JClass,
    config: JString
) {
    let config_str: String = env.get_string(config).unwrap().into();
    // Get network handler instance
    let network_handler = NetworkHandler::get_instance(config_str.clone());
    
    // Initialize event system with network handler
    let _event_system = EventSystem::get_instance(network_handler.clone());
    
    // Start the network
    network_handler.start();
}

#[no_mangle]
pub extern "system" fn Java_com_cpc_NativeBridge_stopP2PService(
    _env: JNIEnv,
    _: JClass
) {
    // Stop P2P service - implementation will be added later
}