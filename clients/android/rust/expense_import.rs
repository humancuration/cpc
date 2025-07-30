//! JNI bindings for expense import functionality
//! 
//! This module provides the FFI layer between Kotlin and Rust for expense import operations,
//! including currency integration.

use jni::JNIEnv;
use jni::objects::{JClass, JString, JList};
use jni::sys::{jboolean, jobject};
use uuid::Uuid;
use std::path::Path;
use packages::domains::sheets::application::expense_import::import_processor::ExpenseImportProcessor;
use packages::domains::finance::application::user_preferences::{UserPreferences, UserPreferencesService};
use packages::domains::finance::domain::primitives::Currency;

/// Import expenses from a file
/// 
/// # Parameters
/// * `env` - JNI environment
/// * `_class` - Java class reference
/// * `file_path` - Path to the file to import
/// * `user_id` - User ID as string
/// 
/// # Returns
/// Import result object
#[no_mangle]
pub extern "system" fn Java_cpc_android_features_expenses_ExpenseImportManager_importExpenses(
    env: JNIEnv,
    _class: JClass,
    file_path: JString,
    user_id: JString,
) -> jobject {
    let result = || -> Result<jobject, Box<dyn std::error::Error>> {
        // Convert parameters from Java strings to Rust strings
        let file_path_str: String = env.get_string(file_path)?.into();
        let user_id_str: String = env.get_string(user_id)?.into();
        
        let user_id = Uuid::parse_str(&user_id_str)?;
        
        // In a real implementation, we would get the import processor and user preferences
        // from a dependency container. For now, we'll create mock implementations.
        
        // Get user's preferred currency
        let currency = get_user_currency(user_id)?;
        
        // Validate currency before processing
        if !is_valid_currency(&currency) {
            return Err(format!("Invalid currency: {}", currency.code()).into());
        }
        
        // Process the import
        let import_result = process_expense_import(&file_path_str, user_id, currency)?;
        
        // Convert result to Java object
        convert_import_result_to_java(&env, import_result)
    }();
    
    match result {
        Ok(obj) => obj,
        Err(e) => {
            // In a real implementation, we would throw a Java exception
            eprintln!("Error importing expenses: {}", e);
            
            // Return null object for now
            std::ptr::null_mut() as jobject
        }
    }
}

/// Get user's preferred currency
/// 
/// In a real implementation, this would use the Sled database or gRPC client
fn get_user_currency(user_id: Uuid) -> Result<Currency, Box<dyn std::error::Error>> {
    // This is a placeholder implementation
    // In reality, this would read from Sled database or call gRPC service
    Ok(Currency::USD)
}

/// Validate currency
fn is_valid_currency(currency: &Currency) -> bool {
    // All currencies from the Currency enum are valid
    true
}

/// Process expense import
/// 
/// In a real implementation, this would use the ExpenseImportProcessor
fn process_expense_import(file_path: &str, user_id: Uuid, currency: Currency) -> Result<ImportResult, Box<dyn std::error::Error>> {
    // This is a placeholder implementation
    // In reality, this would read the file and process it with ExpenseImportProcessor
    
    // For now, we'll simulate a successful import
    Ok(ImportResult {
        total_rows: 10,
        successful_imports: 8,
        failed_rows: vec![
            FailedRow { row_number: 3, error: "Invalid date format".to_string() },
            FailedRow { row_number: 7, error: "Missing amount".to_string() },
        ],
    })
}

/// Convert import result to Java object
fn convert_import_result_to_java(env: &JNIEnv, result: ImportResult) -> Result<jobject, Box<dyn std::error::Error>> {
    // Create Java ImportResult object
    let import_result_class = env.find_class("cpc/android/features/expenses/ImportResult")?;
    
    // Get constructor
    let constructor = env.get_method_id(import_result_class, "<init>", "(IILjava/util/List;)V")?;
    
    // Create failed rows list
    let failed_rows_list = create_failed_rows_list(env, &result.failed_rows)?;
    
    // Create ImportResult object
    let import_result_obj = env.new_object(
        import_result_class,
        "(IILjava/util/List;)V",
        &[
            result.total_rows.into(),
            result.successful_imports.into(),
            failed_rows_list.into(),
        ],
    )?;
    
    Ok(import_result_obj.into_inner())
}

/// Create failed rows list
fn create_failed_rows_list(env: &JNIEnv, failed_rows: &[FailedRow]) -> Result<jobject, Box<dyn std::error::Error>> {
    // Create ArrayList
    let array_list_class = env.find_class("java/util/ArrayList")?;
    let array_list = env.new_object(array_list_class, "()V", &[])?;
    
    // Get add method
    let add_method = env.get_method_id(array_list_class, "add", "(Ljava/lang/Object;)Z")?;
    
    // Add each failed row
    for failed_row in failed_rows {
        // Create FailedRow object
        let failed_row_class = env.find_class("cpc/android/features/expenses/FailedRow")?;
        let failed_row_obj = env.new_object(
            failed_row_class,
            "(ILjava/lang/String;)V",
            &[
                failed_row.row_number.into(),
                env.new_string(&failed_row.error)?.into(),
            ],
        )?;
        
        // Add to list
        env.call_method(
            array_list,
            "add",
            "(Ljava/lang/Object;)Z",
            &[failed_row_obj.into()],
        )?;
    }
    
    Ok(array_list.into_inner())
}

/// Import result structure matching the Kotlin ImportResult class
#[derive(Debug, Clone)]
pub struct ImportResult {
    pub total_rows: i32,
    pub successful_imports: i32,
    pub failed_rows: Vec<FailedRow>,
}

/// Failed row structure matching the Kotlin FailedRow class
#[derive(Debug, Clone)]
pub struct FailedRow {
    pub row_number: i32,
    pub error: String,
}