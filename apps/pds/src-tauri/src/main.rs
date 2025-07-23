#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use pds_tauri::setup;

fn main() {
    tauri::Builder::default()
        .setup(setup)
        .invoke_handler(pds_tauri::register_commands!())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}