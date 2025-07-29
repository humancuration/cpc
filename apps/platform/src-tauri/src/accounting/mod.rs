use tauri::Builder;

pub mod commands;

/// Initialize accounting system
pub fn init_accounting(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize any accounting-related setup here
    Ok(())
}

/// Register accounting commands with Tauri
pub fn register_commands(builder: Builder) -> Builder {
    builder.invoke_handler(tauri::generate_handler![
        commands::get_accounting_dashboard,
        commands::refresh_accounting_dashboard
    ])
}