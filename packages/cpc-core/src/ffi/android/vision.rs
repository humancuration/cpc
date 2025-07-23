//! Android FFI bindings for image recognition
//!
//! Provides JNI bindings for Android to use the image recognition functionality

use std::ffi::CString;
use std::os::raw::c_char;
use jni::JNIEnv;
use jni::objects::{JClass, JObject, JString};
use jni::sys::{jstring, jobject, jint, jlong};
use image::{DynamicImage, Rgba};
use serde_json;
use crate::vision::{ImageRecognizer, Model, ModelType, RecognitionResult};

/// Convert Android bitmap to DynamicImage
#[cfg(target_os = "android")]
fn android_bitmap_to_image(env: &JNIEnv, bitmap: jobject) -> Result<DynamicImage, String> {
    use jni::sys::{jint, jboolean, JNI_TRUE};
    
    // Get Android bitmap info
    let bitmap_class = env.find_class("android/graphics/Bitmap")
        .map_err(|e| format!("Failed to find Bitmap class: {}", e))?;
    
    // Get width and height
    let width = env.call_method(bitmap_class, "getWidth", "()I", &[])
        .map_err(|e| format!("Failed to get bitmap width: {}", e))?
        .i()
        .map_err(|e| format!("Failed to get width value: {}", e))? as u32;
    
    let height = env.call_method(bitmap_class, "getHeight", "()I", &[])
        .map_err(|e| format!("Failed to get bitmap height: {}", e))?
        .i()
        .map_err(|e| format!("Failed to get height value: {}", e))? as u32;
    
    // Create buffer for pixel data
    let mut pixels = vec![0u8; (width * height * 4) as usize];
    
    // Copy pixels from Android bitmap
    let result = env.call_method(
        bitmap,
        "copyPixelsToBuffer",
        "(Ljava/nio/ByteBuffer;)V",
        &[env.new_direct_byte_buffer(&mut pixels)
            .map_err(|e| format!("Failed to create byte buffer: {}", e))?
            .into()]
    ).map_err(|e| format!("Failed to copy pixels: {}", e))?;
    
    // Convert Android ARGB to RGBA
    let mut rgba_pixels = Vec::with_capacity((width * height * 4) as usize);
    for chunk in pixels.chunks_exact(4) {
        let a = chunk[0];
        let r = chunk[1];
        let g = chunk[2];
        let b = chunk[3];
        rgba_pixels.extend_from_slice(&[r, g, b, a]);
    }
    
    // Create image from pixel data
    let image = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(width, height, rgba_pixels)
        .ok_or("Failed to create image buffer")?;
    
    Ok(DynamicImage::ImageRgba8(image))
}

/// JNI function for image recognition
#[no_mangle]
#[cfg(target_os = "android")]
pub extern "C" fn Java_com_cpc_vision_ImageRecognition_nativeRecognize(
    env: JNIEnv,
    _class: JClass,
    bitmap: jobject,
    model_path: JString,
) -> jstring {
    let result = std::panic::catch_unwind(|| {
        // Convert Java string to Rust string
        let model_path_str: String = env.get_string(model_path)
            .map_err(|e| format!("Failed to get model path: {}", e))?
            .into();
        
        // Convert Android bitmap to DynamicImage
        let image = android_bitmap_to_image(&env, bitmap)
            .map_err(|e| format!("Failed to convert bitmap: {}", e))?;
        
        // Create model configuration
        let model = Model {
            model_type: ModelType::ObjectDetection,
            path: std::path::PathBuf::from(model_path_str),
            input_size: (640, 640), // Default YOLO input size
            confidence_threshold: 0.5,
            labels: vec![
                "person".to_string(), "bicycle".to_string(), "car".to_string(),
                "motorcycle".to_string(), "airplane".to_string(), "bus".to_string(),
                "train".to_string(), "truck".to_string(), "boat".to_string(),
                "traffic light".to_string(), "fire hydrant".to_string(), "stop sign".to_string(),
                "parking meter".to_string(), "bench".to_string(), "bird".to_string(),
                "cat".to_string(), "dog".to_string(), "horse".to_string(),
                "sheep".to_string(), "cow".to_string(), "elephant".to_string(),
                "bear".to_string(), "zebra".to_string(), "giraffe".to_string(),
            ], // COCO classes
        };
        
        // Create recognizer
        let recognizer = ImageRecognizer::new(model)
            .map_err(|e| format!("Failed to create recognizer: {}", e))?;
        
        // Perform recognition
        let result = recognizer.recognize(&image)
            .map_err(|e| format!("Recognition failed: {}", e))?;
        
        // Serialize result to JSON
        let json = serde_json::to_string(&result)
            .map_err(|e| format!("Failed to serialize result: {}", e))?;
        
        Ok(json)
    });
    
    match result {
        Ok(Ok(json)) => {
            env.new_string(json)
                .unwrap_or_else(|_| env.new_string("{\"error\":\"Failed to create Java string\"}").unwrap())
                .into_raw()
        }
        Ok(Err(err)) => {
            let error_json = format!("{{\"error\":\"{}\"}}", err);
            env.new_string(error_json)
                .unwrap_or_else(|_| env.new_string("{\"error\":\"Failed to create error string\"}").unwrap())
                .into_raw()
        }
        Err(_) => {
            env.new_string("{\"error\":\"Panic in native code\"}")
                .unwrap()
                .into_raw()
        }
    }
}

/// JNI function to initialize recognizer
#[no_mangle]
#[cfg(target_os = "android")]
pub extern "C" fn Java_com_cpc_vision_ImageRecognition_nativeInitRecognizer(
    env: JNIEnv,
    _class: JClass,
    model_path: JString,
    model_type: jint,
    input_width: jint,
    input_height: jint,
    confidence_threshold: jstring,
) -> jlong {
    let result = std::panic::catch_unwind(|| {
        let model_path_str: String = env.get_string(model_path)
            .map_err(|e| format!("Failed to get model path: {}", e))?
            .into();
        
        let confidence_str: String = env.get_string(unsafe { JString::from_raw(confidence_threshold) })
            .map_err(|e| format!("Failed to get confidence threshold: {}", e))?
            .into();
        
        let confidence: f32 = confidence_str.parse()
            .map_err(|e| format!("Failed to parse confidence: {}", e))?;
        
        let model_type = match model_type {
            0 => ModelType::ObjectDetection,
            1 => ModelType::Classification,
            2 => ModelType::FeatureExtraction,
            3 => ModelType::TextRecognition,
            _ => return Err("Invalid model type".to_string()),
        };
        
        let model = Model {
            model_type,
            path: std::path::PathBuf::from(model_path_str),
            input_size: (input_width as u32, input_height as u32),
            confidence_threshold: confidence,
            labels: vec![], // Can be populated from Java
        };
        
        let recognizer = ImageRecognizer::new(model)
            .map_err(|e| format!("Failed to create recognizer: {}", e))?;
        
        // Return pointer to recognizer
        Ok(Box::into_raw(Box::new(recognizer)) as jlong)
    });
    
    match result {
        Ok(Ok(ptr)) => ptr,
        Ok(Err(_)) | Err(_) => 0,
    }
}

/// JNI function to cleanup recognizer
#[no_mangle]
#[cfg(target_os = "android")]
pub extern "C" fn Java_com_cpc_vision_ImageRecognition_nativeDestroyRecognizer(
    _env: JNIEnv,
    _class: JClass,
    recognizer_ptr: jlong,
) {
    if recognizer_ptr != 0 {
        unsafe {
            let _ = Box::from_raw(recognizer_ptr as *mut ImageRecognizer);
        }
    }
}

/// JNI function to recognize with existing recognizer
#[no_mangle]
#[cfg(target_os = "android")]
pub extern "C" fn Java_com_cpc_vision_ImageRecognition_nativeRecognizeWithRecognizer(
    env: JNIEnv,
    _class: JClass,
    recognizer_ptr: jlong,
    bitmap: jobject,
) -> jstring {
    let result = std::panic::catch_unwind(|| {
        if recognizer_ptr == 0 {
            return Err("Invalid recognizer pointer".to_string());
        }
        
        let recognizer = unsafe { &*(recognizer_ptr as *const ImageRecognizer) };
        
        let image = android_bitmap_to_image(&env, bitmap)
            .map_err(|e| format!("Failed to convert bitmap: {}", e))?;
        
        let result = recognizer.recognize(&image)
            .map_err(|e| format!("Recognition failed: {}", e))?;
        
        let json = serde_json::to_string(&result)
            .map_err(|e| format!("Failed to serialize result: {}", e))?;
        
        Ok(json)
    });
    
    match result {
        Ok(Ok(json)) => {
            env.new_string(json)
                .unwrap_or_else(|_| env.new_string("{\"error\":\"Failed to create Java string\"}").unwrap())
                .into_raw()
        }
        Ok(Err(err)) => {
            let error_json = format!("{{\"error\":\"{}\"}}", err);
            env.new_string(error_json)
                .unwrap_or_else(|_| env.new_string("{\"error\":\"Failed to create error string\"}").unwrap())
                .into_raw()
        }
        Err(_) => {
            env.new_string("{\"error\":\"Panic in native code\"}")
                .unwrap()
                .into_raw()
        }
    }
}