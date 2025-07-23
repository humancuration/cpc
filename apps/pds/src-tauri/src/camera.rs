use bevy::prelude::*;
use bevy::window::{PrimaryWindow, Window};
use image::{DynamicImage, ImageBuffer, Rgba, ImageOutputFormat};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Camera capture state
#[derive(Resource, Default)]
pub struct CameraState {
    pub is_capturing: bool,
    pub last_frame: Option<DynamicImage>,
}

/// Camera capture component
#[derive(Component)]
pub struct CameraCapture;

/// System to setup camera
pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

/// System to capture frame from camera
pub fn capture_frame(
    mut camera_state: ResMut<CameraState>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    if !camera_state.is_capturing {
        return;
    }

    if let Ok(window) = windows.get_single() {
        // In a real implementation, this would capture from actual camera
        // For now, we'll create a placeholder image
        let width = window.width() as u32;
        let height = window.height() as u32;
        
        // Create a test image (gradient)
        let mut img = ImageBuffer::<Rgba<u8>>::new(width, height);
        
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let r = (x as f32 / width as f32 * 255.0) as u8;
            let g = (y as f32 / height as f32 * 255.0) as u8;
            let b = 128;
            let a = 255;
            *pixel = Rgba([r, g, b, a]);
        }
        
        camera_state.last_frame = Some(DynamicImage::ImageRgba8(img));
    }
}

/// Tauri command to start camera capture
#[tauri::command]
pub async fn start_camera_capture(
    camera_state: tauri::State<'_, Arc<Mutex<CameraState>>>,
) -> Result<(), String> {
    let mut state = camera_state.lock().await;
    state.is_capturing = true;
    Ok(())
}

/// Tauri command to stop camera capture
#[tauri::command]
pub async fn stop_camera_capture(
    camera_state: tauri::State<'_, Arc<Mutex<CameraState>>>,
) -> Result<(), String> {
    let mut state = camera_state.lock().await;
    state.is_capturing = false;
    Ok(())
}

/// Tauri command to get latest frame
#[tauri::command]
pub async fn get_latest_frame(
    camera_state: tauri::State<'_, Arc<Mutex<CameraState>>>,
) -> Result<Option<Vec<u8>>, String> {
    let mut state = camera_state.lock().await;
    
    if let Some(image) = &state.last_frame {
        // Convert to JPEG bytes
        let mut buffer = Vec::new();
        image.write_to(&mut buffer, ImageOutputFormat::Jpeg(90))
            .map_err(|e| format!("Failed to encode image: {}", e))?;
        
        Ok(Some(buffer))
    } else {
        Ok(None)
    }
}