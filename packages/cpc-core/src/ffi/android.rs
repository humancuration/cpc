use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use serde_json;
use uuid::Uuid;

use crate::models::social::post::Post;
use crate::models::social::relationship::Relationship;

/// Helper function to convert C string to Rust string
unsafe fn c_str_to_string(c_str: *const c_char) -> String {
    if c_str.is_null() {
        return String::new();
    }
    CStr::from_ptr(c_str).to_string_lossy().into_owned()
}

/// Helper function to convert Rust string to C string
fn string_to_c_str(s: String) -> *mut c_char {
    CString::new(s).unwrap().into_raw()
}

/// Create a new post from JSON
#[no_mangle]
pub extern "C" fn create_post_native(post_json: *const c_char) -> *mut c_char {
    let json_str = unsafe { c_str_to_string(post_json) };
    
    let result = match serde_json::from_str::<Post>(&json_str) {
        Ok(mut post) => {
            // Generate ID if not provided
            if post.id.is_nil() {
                post.id = Uuid::new_v4();
            }
            
            // Serialize back to JSON
            match serde_json::to_string(&post) {
                Ok(json) => json,
                Err(e) => format!("{{\"error\":\"Failed to serialize post: {}\"}}", e)
            }
        }
        Err(e) => format!("{{\"error\":\"Failed to parse post JSON: {}\"}}", e)
    };
    
    string_to_c_str(result)
}

/// Get timeline for a user
#[no_mangle]
pub extern "C" fn get_timeline_native(user_id: *const c_char, limit: i32, offset: i32) -> *mut c_char {
    let user_id_str = unsafe { c_str_to_string(user_id) };
    
    // TODO: Implement actual timeline retrieval
    // For now, return empty array
    let posts: Vec<Post> = Vec::new();
    
    let result = match serde_json::to_string(&posts) {
        Ok(json) => json,
        Err(e) => format!("{{\"error\":\"Failed to serialize timeline: {}\"}}", e)
    };
    
    string_to_c_str(result)
}

/// Get a specific post by ID
#[no_mangle]
pub extern "C" fn get_post_native(post_id: *const c_char) -> *mut c_char {
    let post_id_str = unsafe { c_str_to_string(post_id) };
    
    // TODO: Implement actual post retrieval
    // For now, return not found error
    let result = format!("{{\"error\":\"Post not found: {}\"}}", post_id_str);
    
    string_to_c_str(result)
}

/// Create a new relationship (follow)
#[no_mangle]
pub extern "C" fn create_relationship_native(follower_id: *const c_char, followed_id: *const c_char) -> *mut c_char {
    let follower_id_str = unsafe { c_str_to_string(follower_id) };
    let followed_id_str = unsafe { c_str_to_string(followed_id) };
    
    let relationship = Relationship {
        id: Uuid::new_v4(),
        follower_id: Uuid::parse_str(&follower_id_str).unwrap_or(Uuid::new_v4()),
        followed_id: Uuid::parse_str(&followed_id_str).unwrap_or(Uuid::new_v4()),
        created_at: chrono::Utc::now(),
    };
    
    let result = match serde_json::to_string(&relationship) {
        Ok(json) => json,
        Err(e) => format!("{{\"error\":\"Failed to serialize relationship: {}\"}}", e)
    };
    
    string_to_c_str(result)
}

/// Get followers for a user
#[no_mangle]
pub extern "C" fn get_followers_native(user_id: *const c_char) -> *mut c_char {
    let user_id_str = unsafe { c_str_to_string(user_id) };
    
    // TODO: Implement actual followers retrieval
    let relationships: Vec<Relationship> = Vec::new();
    
    let result = match serde_json::to_string(&relationships) {
        Ok(json) => json,
        Err(e) => format!("{{\"error\":\"Failed to serialize followers: {}\"}}", e)
    };
    
    string_to_c_str(result)
}

/// Get following for a user
#[no_mangle]
pub extern "C" fn get_following_native(user_id: *const c_char) -> *mut c_char {
    let user_id_str = unsafe { c_str_to_string(user_id) };
    
    // TODO: Implement actual following retrieval
    let relationships: Vec<Relationship> = Vec::new();
    
    let result = match serde_json::to_string(&relationships) {
        Ok(json) => json,
        Err(e) => format!("{{\"error\":\"Failed to serialize following: {}\"}}", e)
    };
    
    string_to_c_str(result)
}

/// Free the memory allocated by the C string
#[no_mangle]
pub extern "C" fn free_c_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe { CString::from_raw(s) };
    }
}