use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use crate::services::impact::verify_signature_internal;

#[derive(Debug)]
#[repr(C)]
pub enum VerificationResult {
    Valid,
    Invalid(String),
}

#[no_mangle]
pub extern "C" fn verify_impact_report_signature(
    report_json: *const c_char,
    public_key: *const c_char,
) -> *mut VerificationResult {
    // Convert C strings to Rust strings
    let report_str = unsafe { CStr::from_ptr(report_json).to_string_lossy().into_owned() };
    let public_key_str = unsafe { CStr::from_ptr(public_key).to_string_lossy().into_owned() };
    
    // Call existing verification logic
    match verify_signature_internal(&report_str, &public_key_str) {
        Ok(()) => Box::into_raw(Box::new(VerificationResult::Valid)),
        Err(e) => Box::into_raw(Box::new(VerificationResult::Invalid(e.to_string()))),
    }
}

// Accessor for VerificationResult type (0 = Valid, 1 = Invalid)
#[no_mangle]
pub extern "C" fn get_verification_result_type(ptr: *const VerificationResult) -> i32 {
    let result = unsafe { &*ptr };
    match result {
        VerificationResult::Valid => 0,
        VerificationResult::Invalid(_) => 1,
    }
}

// Accessor for error message (returns null pointer for Valid results)
#[no_mangle]
pub extern "C" fn get_verification_result_error(ptr: *const VerificationResult) -> *const c_char {
    let result = unsafe { &*ptr };
    match result {
        VerificationResult::Valid => std::ptr::null(),
        VerificationResult::Invalid(s) => {
            let c_str = CString::new(s.as_str()).unwrap();
            c_str.into_raw()
        }
    }
}

// Memory management function for Kotlin
#[no_mangle]
pub extern "C" fn free_verification_result(ptr: *mut VerificationResult) {
    if !ptr.is_null() {
        unsafe { Box::from_raw(ptr) };
    }
}

// Convert C string pointer to Rust string
#[no_mangle]
pub extern "C" fn ptr_to_string(ptr: *const c_char) -> *mut c_char {
    if ptr.is_null() {
        return std::ptr::null_mut();
    }
    let c_str = unsafe { CStr::from_ptr(ptr) };
    // Duplicate the string so caller gets ownership
    match c_str.to_str() {
        Ok(s) => CString::new(s).unwrap().into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

// Free error string from get_verification_result_error
#[no_mangle]
pub extern "C" fn free_error_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe { CString::from_raw(ptr) };
    }
}