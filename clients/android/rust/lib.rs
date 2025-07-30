//! Main library for Android Rust integration
//!
//! This module provides the FFI layer between Kotlin and Rust for all Android functionality.

// Public modules
pub mod user_preferences_kotlin;
pub mod expense_import;

// JNI imports
use jni::JNIEnv;
use jni::objects::{JClass, JObject, JString};
use jni::sys::{jboolean, jstring};
use jni::sys::c_char;
use std::ffi::CStr;
use std::sync::Arc;
use sled::Db;
use packages::infra::sync::queue::SyncQueue;
use packages::infra::sync::worker::SyncWorker;
use packages::infra::grpc::clients::user_preferences::{NetworkStatusMonitor, UserPreferencesClient};
use packages::infra::sled::adapters::user_preferences::SledUserPreferences;
use tokio::runtime::Runtime;

/// Global runtime for async operations
static mut RUNTIME: Option<Runtime> = None;

/// JNI_OnLoad function - called when the library is loaded
#[no_mangle]
pub extern "system" fn JNI_OnLoad(_vm: jni::JavaVM, _reserved: *mut std::os::raw::c_void) -> jni::sys::jint {
    // Initialize the Tokio runtime
    unsafe {
        RUNTIME = Some(Runtime::new().expect("Failed to create Tokio runtime"));
    }
    jni::sys::JNI_VERSION_1_6
}

/// Get all supported currencies
///
/// # Parameters
/// * `env` - JNI environment
/// * `_class` - Java class reference
///
/// # Returns
/// Array of supported currency codes
#[no_mangle]
pub extern "system" fn Java_cpc_android_features_userpreferences_UserPreferencesManager_getAllSupportedCurrencies(
    env: JNIEnv,
    _class: JClass,
) -> jobject {
    // Create array of supported currencies
    let currencies = vec![
        "USD", "EUR", "GBP", "JPY", "CAD", "AUD", "CHF", "CNY", "SEK", "NZD",
        "MXN", "SGD", "HKD", "NOK", "KRW", "TRY", "RUB", "INR", "BRL", "ZAR", "DABLOONS"
    ];
    
    // Convert to Java string array
    let java_strings: Vec<JString> = currencies
        .iter()
        .map(|&currency| {
            env.new_string(currency)
                .expect("Couldn't create java string!")
        })
        .collect();
    
    // Create Java array
    let array = env.new_object_array(
        java_strings.len() as i32,
        "java/lang/String",
        JObject::null(),
    ).expect("Couldn't create array");
    
    // Populate array
    for (i, java_string) in java_strings.iter().enumerate() {
        env.set_object_array_element(array, i as i32, *java_string)
            .expect("Couldn't set array element");
    }
    
    array.into_inner()
}

/// Initialize the sync system
///
/// # Parameters
/// * `db_path` - Path to the Sled database
#[no_mangle]
pub extern "C" fn init_sync_system(db_path: *const c_char) {
    let db_path = unsafe { CStr::from_ptr(db_path) }
        .to_str()
        .expect("Invalid DB path");
    
    let db = sled::open(db_path).expect("Failed to open DB");
    
    // Initialize network monitor
    let network_monitor = Arc::new(NetworkStatusMonitor::new());
    
    // Create sync queue
    let conflict_resolver = Arc::new(SledUserPreferences::new(&db));
    let sync_queue = Arc::new(SyncQueue::new(&db, conflict_resolver));
    
    // Create client (simplified for example)
    let client = UserPreferencesClient::new(); // This would need to be properly initialized
    
    // Start background worker
    let (worker, _shutdown_tx) = SyncWorker::new(
        sync_queue,
        client,
        network_monitor,
    );
    
    // Spawn the worker on the Tokio runtime
    unsafe {
        if let Some(runtime) = &RUNTIME {
            runtime.spawn(worker.start());
        }
    }
}