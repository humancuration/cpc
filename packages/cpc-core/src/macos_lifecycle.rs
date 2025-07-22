use std::os::raw::c_void;
use crate::bevy::CpcBevyPlugin;
use crate::p2p::NetworkHandler;
use bevy::prelude::*;

// Add global storage for the Bevy app pointer
static mut BEVY_APP: Option<*mut App> = None;

#[no_mangle]
pub extern "C" fn cpc_core_on_create() {
    println!("CPC Core initialized on macOS");
    // Initialize Bevy app
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugin(CpcBevyPlugin);
    unsafe {
        BEVY_APP = Some(Box::into_raw(Box::new(app)));
    }
    
    // Initialize networking
    let handler = NetworkHandler::get_instance("{}".to_string());
    handler.start();
}

#[no_mangle]
pub extern "C" fn cpc_core_on_pause() {
    println!("CPC Core paused (macOS)");
    // TODO: Suspend Bevy engine
    // TODO: Pause P2P networking
}

#[no_mangle]
pub extern "C" fn cpc_core_on_resume() {
    println!("CPC Core resumed (macOS)");
    // TODO: Resume Bevy engine
    // TODO: Resume P2P networking
}

#[no_mangle]
pub extern "C" fn stop_engine() {
    println!("Stopping CPC Core (macOS)");
    // Stop Bevy engine
    unsafe {
        if let Some(app_ptr) = BEVY_APP {
            CpcBevyPlugin::stop_engine(app_ptr);
            BEVY_APP = None;
        }
    }
    
    // Stop networking
    let handler = NetworkHandler::get_instance("{}".to_string());
    handler.shutdown();
}