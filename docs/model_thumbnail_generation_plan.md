# 3D Model Thumbnail Generation Plan

## 1. Overview

This document outlines the technical design for generating thumbnails for 3D model assets (`.glb` and `.gltf`). This process is more complex than texture thumbnailing as it requires a 3D rendering environment. The goal is to create a robust, performant, and headless solution that integrates with the existing `cpc-studio` asset management system.

This system will leverage the Bevy engine for rendering, ensuring consistency with the main editor and runtime environment.

## 2. Core Requirements

-   **Headless Rendering:** Thumbnails must be generated in the background without creating a visible window.
-   **Bevy Integration:** The solution must use the Bevy engine for rendering.
-   **Performance:** The process must be asynchronous to avoid blocking the main application thread.
-   **Permissive Licensing:** All new dependencies must have MIT or Apache 2.0 licenses.
-   **Output:** Generated thumbnails will be 128x128 PNG images stored in `assets/thumbnails/`.

## 3. Proposed Architecture

The model thumbnail generation will be handled by the `AssetProcessor`. When a model asset needs a thumbnail, the processor will spawn a dedicated, short-lived, headless Bevy application on a background thread.

### 3.1. High-Level Workflow

```mermaid
sequenceDiagram
    participant AP as AssetProcessor
    participant ThreadPool as Background Thread
    participant Bevy as Headless Bevy App
    participant FS as Filesystem

    AP->>ThreadPool: Request thumbnail for model_asset.glb
    ThreadPool->>+Bevy: Spawn Bevy App (headless)
    Bevy->>FS: Load model_asset.glb using AssetServer
    Bevy->>Bevy: Setup Scene (Camera, Light, Model)
    Bevy->>Bevy: Render scene to off-screen texture
    Bevy->>ThreadPool: Extract image data from texture
    ThreadPool->>-Bevy: Shutdown Bevy App
    ThreadPool->>FS: Save image data as {asset_id}.png
    FS-->>AP: Return thumbnail path
```

## 4. Implementation Details

### 4.1. Crate Dependencies

We will need to ensure the following crates and features are configured in `apps/cpc-studio/src-tauri/Cargo.toml`.

```toml
[dependencies]
bevy = { version = "0.13", features = [
    "bevy_asset",
    "bevy_core_pipeline",
    "bevy_gltf",
    "bevy_pbr",
    "bevy_render",
    "png" # For saving the output
] }
# image crate is already a dependency for texture thumbnails
```

We must ensure Bevy's `render` feature is enabled to allow for headless rendering.

### 4.2. Headless Bevy Application

A new function, `generate_model_thumbnail`, will be created. This function will configure and run the headless Bevy instance.

**Configuration:**

-   **`WindowPlugin`:** Configure the primary window to be hidden (`visible: false`) and non-interactive.
-   **`ImagePlugin`:** Use a specific image format suitable for rendering to, like `Rgba8UnormSrgb`.
-   **Plugins:** A minimal set of plugins will be used:
    -   `CorePlugins`
    -   `RenderPlugins`
    -   `PbrPlugin`
    -   `GltfPlugin`

**Scene Setup:**

A simple Bevy system will be responsible for setting up the scene once the model is loaded.

1.  **Load Asset:** The `AssetServer` will be used to load the target `.glb`/`.gltf` file.
2.  **Spawn Model:** Once loaded, the model will be spawned into the scene.
3.  **Setup Camera:** A 3D camera (`Camera3dBundle`) will be positioned to frame the model. We will need a simple algorithm to calculate an appropriate distance and angle based on the model's bounding box to ensure it fits in the frame.
4.  **Add Lighting:** A `DirectionalLight` will be added to illuminate the model.
5.  **Render and Capture:**
    - The scene will be rendered for one frame.
    - A system will access the rendered image from the `Image` asset store. Bevy's `ScreenshotManager` or a similar manual approach can be used to grab the pixel data from the render target.
6.  **Shutdown:** After capturing the image, the Bevy App will be gracefully exited using `AppExit`.

### 4.3. Asynchronous Execution

The entire `generate_model_thumbnail` function will be designed to run on a background thread to prevent UI lock-ups. We will use Tauri's `async_runtime` to spawn the task.

```rust
// In asset_processor.rs

pub async fn generate_thumbnail_async(metadata: AssetMetadata) -> Result<PathBuf> {
    // This will be called from the main thread
    tauri::async_runtime::spawn(async move {
        match metadata.asset_type {
            AssetType::Texture => {
                // Existing texture logic
            }
            AssetType::Model => {
                // Call the new model thumbnail generation logic
                generate_model_thumbnail_headless(&metadata).await
            }
            _ => { /* ... */ }
        }
    }).await? // This might need adjustment based on how we handle the result
}

async fn generate_model_thumbnail_headless(metadata: &AssetMetadata) -> Result<PathBuf> {
    // 1. Setup and run the headless Bevy app on this background thread.
    // 2. The Bevy app will run, render one frame, and save the image.
    // 3. Return the path to the saved thumbnail.
    Ok(PathBuf::new()) // Placeholder
}
```

### 4.4. `asset_processor.rs` Modifications

The existing `generate_thumbnail` function will be refactored to support this new asynchronous workflow and delegate to the appropriate generator based on `AssetType`.

```rust
// In apps/cpc-studio/src-tauri/src/editor_core/asset_processor.rs

// ... (imports)

pub fn generate_thumbnail(metadata: &AssetMetadata) -> Result<PathBuf> {
    match metadata.asset_type {
        AssetType::Texture => {
            // current implementation
            // ...
        }
        AssetType::Model => {
            // This synchronous function will now block on the async one.
            // Or, the caller can be updated to be async.
            // For simplicity, let's assume we can block here for now.
            let handle = tauri::async_runtime::spawn(
                generate_model_thumbnail_headless(metadata.clone())
            );
            futures::executor::block_on(handle)?
        }
        _ => {
            anyhow::bail!("Thumbnail generation not supported for {:?} assets", metadata.asset_type)
        }
    }
}

// This function will contain the Bevy headless logic
fn generate_model_thumbnail_headless(metadata: AssetMetadata) -> impl Future<Output = Result<PathBuf>> {
    // ... Bevy App setup and run logic ...
}
```
*Note: The exact async/sync interaction will need to be refined based on the calling context.*

## 5. Error Handling and Fallbacks

-   **Model Loading Failure:** If the `.glb`/`.gltf` file is invalid or cannot be loaded, the process will log the error and return an error.
-   **Rendering Issues:** If the headless Bevy app crashes or fails to render, the error will be caught.
-   **Fallback Thumbnail:** In any failure case, instead of returning an error that might break the UI, we should copy a default "model" icon to the thumbnail path. This ensures the UI always has something to display. A pre-made `default_model.png` can be included in the application assets.

## 6. Performance Considerations

-   **Minimal Bevy:** The headless Bevy instance will be as lightweight as possible, with no unnecessary plugins (e.g., no physics, UI, or audio).
-   **Single Frame Render:** The app will render only a single frame and then exit, conserving resources.
-   **Thread Management:** Using a managed thread pool like `tauri::async_runtime` prevents us from spawning an unbounded number of threads if many models are imported at once.

This plan provides a clear path to implementing a powerful and integrated model thumbnail generation system.