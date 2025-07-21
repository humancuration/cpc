use std::fs;
use std::path::Path;
use base64::encode;
use tauri::api::file::read_binary;
use tauri::command;

#[derive(serde::Serialize)]
pub struct FilePreview {
    content_type: String,
    data: String,
}

#[command]
pub async fn download_file(path: String) -> Result<(), String> {
    let path = Path::new(&path);
    if !path.exists() {
        return Err("File not found".to_string());
    }

    // In a real implementation, we would stream the file to the user
    // For this example, we'll just log the download
    println!("Downloading file: {:?}", path);
    Ok(())
}

#[command]
pub async fn get_file_preview(path: String) -> Result<FilePreview, String> {
    let path = Path::new(&path);
    if !path.exists() {
        return Err("File not found".to_string());
    }

    let content_type = mime_guess::from_path(path).first_or_octet_stream().to_string();

    // Only read and encode text and image files
    let data = if content_type.starts_with("text/") || content_type.starts_with("image/") {
        match read_binary(&path) {
            Ok(bytes) => encode(bytes),
            Err(e) => return Err(format!("Failed to read file: {}", e)),
        }
    } else {
        String::new()
    };

    Ok(FilePreview {
        content_type,
        data,
    })
}

#[tauri::command]
async fn watch_directory(
    path: String,
    state: State<'_, AppState>
) -> Result<(), String> {
    let mut watcher = state.file_watcher.lock().await;
    watcher.watch(Path::new(&path)).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn get_conflicts(
    state: State<'_, AppState>
) -> Result<Vec<FileConflict>, String> {
    let resolver = state.conflict_resolver.lock().await;
    Ok(resolver.get_conflicts())
}