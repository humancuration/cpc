#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{Manager, RunEvent};
use jni::{JNIEnv, objects::JObject}; // for Android

mod bevy_jni;
mod svelte_bevy_bridge;
mod android_keystore;
mod api;
mod types;
mod invoicing;
#[cfg(not(target_os = "android"))]
mod bevy_plugin;

/// AndroidContext struct for Android platform
#[cfg(target_os = "android")]
#[derive(Clone)]
struct AndroidContext {
    env: jni::JavaVM,
    context: jni::sys::jobject,
}

/// Builds a Tauri application with common setup and handlers
///
/// This shared builder initializes:
/// - Svelte-Bevy communication bridge
/// - Common command handlers
/// - Desktop-specific page load handler
fn build_tauri_app() -> tauri::Builder {
    let mut builder = tauri::Builder::default()
        .setup(|app| {
            // Initialize Svelte-Bevy communication bridge (common for all platforms)
            svelte_bevy_bridge::init_bridge(app);
            
            // Initialize invoicing system
            invoicing::init_invoicing(app)?;
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            svelte_bevy_bridge::show_bevy_view,
            android_keystore::secure_store,
            android_keystore::secure_retrieve,
            api::invoicing::get_invoice_templates,
            api::invoicing::get_contacts,
            api::invoicing::create_invoice
        ]);
    
    // Register invoicing commands
    builder = invoicing::register_commands(builder);
    
    // Add desktop-specific handlers
    #[cfg(not(target_os = "android"))]
    {
        builder = builder.on_page_load(|window, _| {
            window.eval("console.log('Tauri initialized')").unwrap();
        });
    }
    
    builder
}

// Desktop entry point
#[cfg(not(target_os = "android"))]
fn main() {
    build_tauri_app()
        .plugin(bevy_plugin::BevyPlugin::new())
        .run(tauri::generate_context!())
        .expect("error running tauri application");
}

// Android entry point
#[cfg(target_os = "android")]
#[no_mangle]
pub extern "system" fn Java_com_wtf_TauriService_initializeTauri(
    env: JNIEnv,
    _: JObject,
    context: JObject,
) {
    // Initialize JVM for keystore operations
    let vm = env.get_java_vm().unwrap();
    android_keystore::init_jvm(vm);

    // Build and run Tauri app with Android-specific setup
    build_tauri_app()
        .setup(|app| {
            // Store Android context in manage state
            app.manage(AndroidContext {
                env: vm,
                context: context.as_raw(),
            });
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application");
    
    // Note: Event loop not run on Android as service lifecycle is managed by OS
}