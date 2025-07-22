use std::os::raw::c_void;

#[no_mangle]
pub extern "C" fn cpc_core_on_create() {
    println!("CPC Core initialized on iOS");
    // TODO: Initialize Bevy engine
    // TODO: Initialize P2P networking
}

#[no_mangle]
pub extern "C" fn cpc_core_on_pause() {
    println!("CPC Core paused (iOS)");
    // TODO: Suspend Bevy engine
    // TODO: Pause P2P networking
}

#[no_mangle]
pub extern "C" fn cpc_core_on_resume() {
    println!("CPC Core resumed (iOS)");
    // TODO: Resume Bevy engine
    // TODO: Resume P2P networking
}