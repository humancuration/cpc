// Use generated JNI bindings from protobuf compilation
use cpc_protos::jni_bindings::*;
use jni::{
    objects::{JClass, JObject, JString},
    sys::{jbyteArray, jdouble, jfloat, jint, jlong, jobjectArray},
    JNIEnv,
};
use std::ffi::c_void;
use tokio::runtime::Runtime;

use cpc_core::models::product::Product;
use cpc_core::product::extensions::ProductExt;

// Keep metrics-related functions
#[no_mangle]
pub extern "system" fn Java_com_cpc_android_JniBridge_getAggregatedMetrics(
    env: JNIEnv,
    _class: JClass,
    time_range: JString,
    roles: jobjectArray,
) -> jbyteArray {
    let time_range_str: String = env.get_string(time_range).unwrap().into();
    let roles_vec: Vec<String> = string_array_to_vec(&env, roles);
    
    let rt = Runtime::new().expect("Failed to create Tokio runtime");
    let metrics = rt.block_on(async {
        let service = crate::services::metrics_service::MetricsService::new();
        let request = cpc_protos::metrics::MetricsRequest {
            time_range: time_range_str,
            member_roles: roles_vec,
        };
        service.get_aggregated_metrics(tonic::Request::new(request)).await
    }).unwrap().into_inner();
    
    let bytes = metrics.encode_to_vec();
    env.byte_array_from_slice(&bytes).unwrap()
}

#[no_mangle]
pub extern "system" fn Java_com_cpc_android_JniBridge_exportMetricsToPdf(
    env: JNIEnv,
    _class: JClass,
    time_range: JString,
    roles: jobjectArray,
) -> jbyteArray {
    let time_range_str: String = env.get_string(time_range).unwrap().into();
    let roles_vec: Vec<String> = string_array_to_vec(&env, roles);
    
    let rt = Runtime::new().expect("Failed to create Tokio runtime");
    let response = rt.block_on(async {
        let service = crate::services::metrics_service::MetricsService::new();
        let request = cpc_protos::metrics::MetricsRequest {
            time_range: time_range_str,
            member_roles: roles_vec,
        };
        service.export_metrics_to_pdf(tonic::Request::new(request)).await
    }).unwrap().into_inner();
    
    env.byte_array_from_slice(&response.pdf_data).unwrap()
}

// Helper function for converting string arrays
fn string_array_to_vec(env: &JNIEnv, array: jobjectArray) -> Vec<String> {
    let length = env.get_array_length(array).unwrap();
    let mut vec = Vec::with_capacity(length as usize);
    for i in 0..length {
        let jstring = env.get_object_array_element(array, i).unwrap();
        let string: String = env.get_string(jstring.into()).unwrap().into();
        vec.push(string);
    }
    vec
}
#[no_mangle]
pub extern "system" fn Java_com_cpc_android_JniBridge_calculateProductTax(
    env: JNIEnv,
    _class: JClass,
    product_ptr: jlong,
) -> jdouble {
    // Convert pointer to Product reference
    let product = unsafe { &*(product_ptr as *const Product) };
    
    // Calculate tax using our extension trait
    product.calculate_tax()
}