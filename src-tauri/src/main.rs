#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::download_file,
            commands::get_file_preview
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}