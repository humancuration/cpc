use tauri::PathResolver;
use cpc_core::vision::{ImageRecognizer, RecognitionResult};
use anyhow::Result;
use std::path::{PathBuf, Path};
use image::DynamicImage;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;

/// Shared state for vision models
pub struct VisionState {
    recognizers: Arc<Mutex<HashMap<String, Arc<ImageRecognizer>>>>,
    default_model: Arc<Mutex<Option<String>>>,
}

impl Default for VisionState {
    fn default() -> Self {
        Self {
            recognizers: Arc::new(Mutex::new(HashMap::new())),
            default_model: Arc::new(Mutex::new(None)),
        }
    }
}

/// Initialize the vision system with default model
#[tauri::command]
pub async fn initialize_vision(
    app: tauri::AppHandle,
    state: tauri::State<'_, VisionState>,
) -> Result<(), String> {
    // Try to load the default MobileNet model
    let model_path = resolve_model_path(&app, "mobilenetv2.onnx")
        .or_else(|_| resolve_model_path(&app, "model.onnx"))
        .map_err(|e| format!("Failed to resolve model path: {}", e))?;
    
    let recognizer = ImageRecognizer::new(&model_path)
        .map_err(|e| format!("Failed to load model: {}", e))?;
    
    let mut recognizers = state.recognizers.lock().await;
    let mut default_model = state.default_model.lock().await;
    
    let model_name = model_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("default")
        .to_string();
    
    recognizers.insert(model_name.clone(), Arc::new(recognizer));
    *default_model = Some(model_name);
    
    Ok(())
}

/// Tauri command for image recognition with enhanced functionality
#[tauri::command]
pub async fn recognize_image(
    path: PathBuf,
    app: tauri::AppHandle,
    model_name: Option<String>,
    confidence_threshold: Option<f32>,
    max_results: Option<usize>,
    state: tauri::State<'_, VisionState>,
) -> Result<Vec<RecognitionResult>, String> {
    let recognizer = get_recognizer(&state, model_name).await?;
    
    let image = image::open(&path)
        .map_err(|e| format!("Failed to open image: {}", e))?;
    
    let mut results = recognizer.recognize(&image)
        .map_err(|e| format!("Recognition failed: {}", e))?;
    
    // Apply confidence threshold
    let threshold = confidence_threshold.unwrap_or(0.1);
    results.retain(|r| r.confidence >= threshold);
    
    // Limit results
    let max = max_results.unwrap_or(5);
    results.truncate(max);
    
    Ok(results)
}

/// Tauri command for batch image recognition
#[tauri::command]
pub async fn recognize_batch(
    paths: Vec<PathBuf>,
    app: tauri::AppHandle,
    model_name: Option<String>,
    state: tauri::State<'_, VisionState>,
) -> Result<Vec<Vec<RecognitionResult>>, String> {
    let recognizer = get_recognizer(&state, model_name).await?;
    
    let mut all_results = Vec::new();
    
    for path in paths {
        let image = image::open(&path)
            .map_err(|e| format!("Failed to open image {}: {}", path.display(), e))?;
        
        let results = recognizer.recognize(&image)
            .map_err(|e| format!("Recognition failed for {}: {}", path.display(), e))?;
        
        all_results.push(results);
    }
    
    Ok(all_results)
}

/// Resolves the path to the model file
/// Tries multiple locations in order:
/// 1. App data directory
/// 2. Resources directory
/// 3. Current directory
fn resolve_model_path(app: &tauri::AppHandle, model_name: &str) -> Result<PathBuf> {
    use std::fs;
    
    // Try app data directory
    if let Some(app_dir) = app.path_resolver().app_data_dir() {
        let model_path = app_dir.join("models").join(model_name);
        if model_path.exists() {
            return Ok(model_path);
        }
    }
    
    // Try resources directory
    if let Some(resource_dir) = app.path_resolver().resource_dir() {
        let model_path = resource_dir.join("models").join(model_name);
        if model_path.exists() {
            return Ok(model_path);
        }
    }
    
    // Try current directory
    let current_dir = std::env::current_dir()
        .map_err(|e| anyhow::anyhow!("Failed to get current directory: {}", e))?;
    
    let model_path = current_dir.join(model_name);
    if model_path.exists() {
        return Ok(model_path);
    }
    
    // Try models subdirectory
    let model_path = current_dir.join("models").join(model_name);
    if model_path.exists() {
        return Ok(model_path);
    }
    
    Err(anyhow::anyhow!(
        "Model file '{}' not found in any expected location",
        model_name
    ))
}

/// Tauri command to get available model paths
#[tauri::command]
pub async fn get_available_models(app: tauri::AppHandle) -> Result<Vec<String>, String> {
    let mut models = Vec::new();
    
    // Check app data directory
    if let Some(app_dir) = app.path_resolver().app_data_dir() {
        let models_dir = app_dir.join("models");
        if let Ok(entries) = std::fs::read_dir(&models_dir) {
            for entry in entries.flatten() {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() && entry.path().extension()
                        .and_then(|s| s.to_str()) == Some("onnx") {
                        if let Some(name) = entry.file_name().to_str() {
                            models.push(format!("app_data:{}", name));
                        }
                    }
                }
            }
        }
    }
    
    // Check resources directory
    if let Some(resource_dir) = app.path_resolver().resource_dir() {
        let models_dir = resource_dir.join("models");
        if let Ok(entries) = std::fs::read_dir(&models_dir) {
            for entry in entries.flatten() {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() && entry.path().extension()
                        .and_then(|s| s.to_str()) == Some("onnx") {
                        if let Some(name) = entry.file_name().to_str() {
                            models.push(format!("resource:{}", name));
                        }
                    }
                }
            }
        }
    }
    
    // Check current directory
    let current_dir = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(_) => return Ok(models),
    };
    
    // Check for .onnx files in current directory
    if let Ok(entries) = std::fs::read_dir(&current_dir) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_file() && entry.path().extension()
                    .and_then(|s| s.to_str()) == Some("onnx") {
                    if let Some(name) = entry.file_name().to_str() {
                        models.push(format!("current:{}", name));
                    }
                }
            }
        }
    }
    
    Ok(models)
}

/// Tauri command to process image from camera
#[tauri::command]
pub async fn process_camera_image(
    image_data: Vec<u8>,
    width: u32,
    height: u32,
    format: String,
    confidence_threshold: Option<f32>,
    max_results: Option<usize>,
    state: tauri::State<'_, VisionState>,
) -> Result<Vec<RecognitionResult>, String> {
    let recognizer = get_recognizer(&state, None).await?;
    
    // Decode image based on format
    let image = match format.as_str() {
        "rgb" | "RGB" => {
            let img = image::RgbImage::from_raw(width, height, image_data)
                .ok_or("Failed to create RGB image from bytes")?;
            DynamicImage::ImageRgb8(img)
        },
        "rgba" | "RGBA" => {
            let img = image::RgbaImage::from_raw(width, height, image_data)
                .ok_or("Failed to create RGBA image from bytes")?;
            DynamicImage::ImageRgba8(img)
        },
        "bgr" | "BGR" => {
            // Convert BGR to RGB
            let mut rgb_data = Vec::with_capacity((width * height * 3) as usize);
            for chunk in image_data.chunks(3) {
                if chunk.len() == 3 {
                    rgb_data.extend_from_slice(&[chunk[2], chunk[1], chunk[0]]);
                }
            }
            let img = image::RgbImage::from_raw(width, height, rgb_data)
                .ok_or("Failed to create RGB image from BGR data")?;
            DynamicImage::ImageRgb8(img)
        },
        _ => {
            // Try to auto-detect format
            image::load_from_memory(&image_data)
                .map_err(|e| format!("Failed to load image: {}", e))?
        }
    };
    
    let mut results = recognizer.recognize(&image)
        .map_err(|e| format!("Recognition failed: {}", e))?;
    
    // Apply confidence threshold
    let threshold = confidence_threshold.unwrap_or(0.1);
    results.retain(|r| r.confidence >= threshold);
    
    // Limit results
    let max = max_results.unwrap_or(5);
    results.truncate(max);
    
    Ok(results)
}

/// Tauri command to process image from file path
#[tauri::command]
pub async fn process_image_path(
    path: PathBuf,
    confidence_threshold: Option<f32>,
    max_results: Option<usize>,
    state: tauri::State<'_, VisionState>,
) -> Result<Vec<RecognitionResult>, String> {
    recognize_image(path, tauri::AppHandle::default(), None, confidence_threshold, max_results, state).await
}

/// Helper function to get recognizer from state
async fn get_recognizer(
    state: &tauri::State<'_, VisionState>,
    model_name: Option<String>,
) -> Result<Arc<ImageRecognizer>, String> {
    let recognizers = state.recognizers.lock().await;
    let default_model = state.default_model.lock().await;
    
    let model_key = model_name
        .or_else(|| default_model.clone())
        .ok_or("No model available. Please initialize vision system first.")?;
    
    recognizers
        .get(&model_key)
        .cloned()
        .ok_or_else(|| format!("Model '{}' not found", model_key))
}

/// Tauri command to get model information
#[tauri::command]
pub async fn get_model_info(
    state: tauri::State<'_, VisionState>,
) -> Result<serde_json::Value, String> {
    let recognizers = state.recognizers.lock().await;
    let default_model = state.default_model.lock().await;
    
    let models: Vec<String> = recognizers.keys().cloned().collect();
    
    Ok(serde_json::json!({
        "available_models": models,
        "default_model": default_model.clone(),
        "model_count": models.len()
    }))
}

/// Tauri command to load a specific model
#[tauri::command]
pub async fn load_model(
    model_name: String,
    app: tauri::AppHandle,
    state: tauri::State<'_, VisionState>,
) -> Result<(), String> {
    let model_path = resolve_model_path(&app, &format!("{}.onnx", model_name))
        .map_err(|e| format!("Failed to resolve model path: {}", e))?;
    
    let recognizer = ImageRecognizer::new(&model_path)
        .map_err(|e| format!("Failed to load model: {}", e))?;
    
    let mut recognizers = state.recognizers.lock().await;
    recognizers.insert(model_name, Arc::new(recognizer));
    
    Ok(())
}

/// Tauri command to unload a model
#[tauri::command]
pub async fn unload_model(
    model_name: String,
    state: tauri::State<'_, VisionState>,
) -> Result<(), String> {
    let mut recognizers = state.recognizers.lock().await;
    recognizers.remove(&model_name);
    Ok(())
}