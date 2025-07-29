use tauri::{AppHandle, Manager, Runtime};
use std::sync::Mutex;

/// State tracking Bevy view visibility
pub struct BevyState {
    pub is_active: bool,
}

/// Toggle Bevy view visibility from Svelte
#[tauri::command]
pub fn show_bevy_view<R: Runtime>(app: AppHandle<R>, show: bool) -> Result<(), String> {
    // Emit event to native Android code
    app.emit_all("bevy-view-toggle", show)
        .map_err(|e| e.to_string())?;
    
    // Update internal state
    let state = app.state::<Mutex<BevyState>>();
    let mut state = state.lock()
        .map_err(|e| format!("State lock error: {}", e))?;
    state.is_active = show;

    Ok(())
}

/// Initialize the communication bridge
pub fn init_bridge<R: Runtime>(app: &mut tauri::App<R>) {
    app.manage(Mutex::new(BevyState { is_active: false }));
}