//! JNI bindings for user preferences functionality
//! 
//! This module provides the FFI layer between Kotlin and Rust for user preferences,
//! specifically currency preferences.

use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jstring;
use uuid::Uuid;
use packages::domains::finance::domain::primitives::Currency;
use packages::domains::finance::application::user_preferences::{UserPreferences, UserPreferencesService};
use std::sync::Arc;

/// Get the user's preferred currency
/// 
/// # Parameters
/// * `env` - JNI environment
/// * `_class` - Java class reference
/// * `user_id` - User ID as string
/// 
/// # Returns
/// Currency code as string
#[no_mangle]
pub extern "system" fn Java_cpc_android_features_userpreferences_UserPreferencesManager_getPreferredCurrency(
    env: JNIEnv,
    _class: JClass,
    user_id: JString,
) -> jstring {
    let result = || -> Result<String, Box<dyn std::error::Error>> {
        // Convert user_id from JString to Rust String
        let user_id_str: String = env.get_string(user_id)?.into();
        let user_id = Uuid::parse_str(&user_id_str)?;
        
        // In a real implementation, we would get the preferences service from a dependency container
        // For now, we'll create a mock implementation
        let currency = get_user_currency_from_local_storage(user_id)?;
        
        Ok(currency.code().to_string())
    }();
    
    match result {
        Ok(currency_code) => {
            // Convert Rust String to Java String
            env.new_string(currency_code)
                .expect("Couldn't create java string!")
                .into_inner()
        }
        Err(e) => {
            // Log error and return default currency
            eprintln!("Error getting preferred currency: {}", e);
            
            // Convert default currency to Java String
            env.new_string("USD")
                .expect("Couldn't create java string!")
                .into_inner()
        }
    }
}

/// Set the user's preferred currency
/// 
/// # Parameters
/// * `env` - JNI environment
/// * `_class` - Java class reference
/// * `user_id` - User ID as string
/// * `currency_code` - Currency code as string
/// 
/// # Returns
/// true if successful, false otherwise
#[no_mangle]
pub extern "system" fn Java_cpc_android_features_userpreferences_UserPreferencesManager_setPreferredCurrency(
    env: JNIEnv,
    _class: JClass,
    user_id: JString,
    currency_code: JString,
) -> bool {
    let result = || -> Result<(), Box<dyn std::error::Error>> {
        // Convert parameters from Java strings to Rust strings
        let user_id_str: String = env.get_string(user_id)?.into();
        let currency_code_str: String = env.get_string(currency_code)?.into();
        
        let user_id = Uuid::parse_str(&user_id_str)?;
        let currency = Currency::from_code(&currency_code_str)
            .ok_or("Invalid currency code")?;
        
        // In a real implementation, we would get the preferences service from a dependency container
        // For now, we'll use a mock implementation
        set_user_currency_in_local_storage(user_id, currency)?;
        
        Ok(())
    }();
    
    match result {
        Ok(()) => true,
        Err(e) => {
            eprintln!("Error setting preferred currency: {}", e);
            false
        }
    }
}

/// Mock function to get user currency from local storage
/// 
/// In a real implementation, this would use the Sled database
fn get_user_currency_from_local_storage(user_id: Uuid) -> Result<Currency, Box<dyn std::error::Error>> {
    // This is a placeholder implementation
    // In reality, this would read from Sled database
    Ok(Currency::USD)
}

/// Mock function to set user currency in local storage
/// 
/// In a real implementation, this would use the Sled database
fn set_user_currency_in_local_storage(user_id: Uuid, currency: Currency) -> Result<(), Box<dyn std::error::Error>> {
    // This is a placeholder implementation
    // In reality, this would write to Sled database
    println!("Setting currency for user {} to {}", user_id, currency.code());
    Ok(())
}

/// Extension trait to convert currency code strings to Currency enum
trait CurrencyFromCode {
    fn from_code(code: &str) -> Option<Currency>;
}

impl CurrencyFromCode for Currency {
    fn from_code(code: &str) -> Option<Currency> {
        match code {
            "USD" => Some(Currency::USD),
            "EUR" => Some(Currency::EUR),
            "GBP" => Some(Currency::GBP),
            "JPY" => Some(Currency::JPY),
            "CAD" => Some(Currency::CAD),
            "AUD" => Some(Currency::AUD),
            "CHF" => Some(Currency::CHF),
            "CNY" => Some(Currency::CNY),
            "SEK" => Some(Currency::SEK),
            "NZD" => Some(Currency::NZD),
            "MXN" => Some(Currency::MXN),
            "SGD" => Some(Currency::SGD),
            "HKD" => Some(Currency::HKD),
            "NOK" => Some(Currency::NOK),
            "KRW" => Some(Currency::KRW),
            "TRY" => Some(Currency::TRY),
            "RUB" => Some(Currency::RUB),
            "INR" => Some(Currency::INR),
            "BRL" => Some(Currency::BRL),
            "ZAR" => Some(Currency::ZAR),
            "DABLOONS" => Some(Currency::Dabloons),
            _ => None,
        }
    }
}