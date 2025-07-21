# Thumbnail Generation Implementation Plan

This document outlines the plan for implementing thumbnail generation for texture assets in the CPC Studio.

## 1. Overview

The goal is to replace the stubbed thumbnail generation logic in `asset_processor.rs` with a robust implementation that can handle common image formats, generate 128x128 thumbnails while preserving aspect ratio, and store them in a designated directory.

## 2. Crate Selection

We will use the `image` crate for this task.

-   **License:** MIT/Apache 2.0, which aligns with our permissive license policy.
-   **Features:** It provides a comprehensive set of tools for image loading, processing, and saving, supporting a wide range of formats like PNG, JPEG, BMP, and more.
-   **Functionality:** It includes functions for resizing images, which is exactly what we need for creating thumbnails.

The `image` crate will be added to the `[dependencies]` section of `apps/cpc-studio/src-tauri/Cargo.toml`.

```toml
[dependencies]
image = "0.24"
```

## 3. Step-by-Step Workflow

The thumbnail generation process will be as follows:

1.  The `generate_thumbnail` function in `apps/cpc-studio/src-tauri/src/editor_core/asset_processor.rs` is called with the `AssetMetadata`.
2.  The function checks if the `AssetType` is `Texture`.
3.  It constructs the path to the source image using `metadata.path`.
4.  It defines the output directory for thumbnails: `assets/thumbnails/`. The code will ensure this directory exists, creating it if necessary.
5.  The output path for the thumbnail is determined, e.g., `assets/thumbnails/{asset_id}.png`.
6.  The source image is loaded into memory using `image::open()`.
7.  A 128x128 thumbnail is generated using `image::DynamicImage::thumbnail()`. This function preserves the aspect ratio, fitting the image within the 128x128 bounds.
8.  The generated thumbnail is saved to the output path in PNG format.
9.  The `PathBuf` of the newly created thumbnail is returned.
10. The `thumbnail_path` field in the corresponding `AssetMetadata` will be updated with this path.

## 4. Error Handling Strategy

Robust error handling is crucial. The `generate_thumbnail` function will return a `anyhow::Result`.

-   **File I/O:** `std::fs::create_dir_all` and file saving operations can fail. These `std::io::Error`s will be propagated using `anyhow`.
-   **Image Loading/Processing:** The `image` crate's functions return `ImageResult<T>`. Any errors (e.g., unsupported format, corrupted file) will be converted into an `anyhow::Error` and returned.
-   **Unsupported Asset Types:** The function will continue to return an error for asset types that do not support thumbnails, as it does now.

Example error handling:

```rust
use anyhow::{Context, Result};

// inside generate_thumbnail
let img = image::open(&metadata.path)
    .with_context(|| format!("Failed to open image at {:?}", &metadata.path))?;
```

## 5. Performance Considerations

Image processing can be CPU-intensive.

-   **Asynchronous Execution:** For a responsive UI, thumbnail generation for multiple assets should not block the main thread. While the current implementation will be synchronous, we should consider moving this logic to a dedicated thread pool managed by Tauri's `async_runtime` in the future if performance becomes an issue, especially during initial asset library scanning.
-   **Caching:** If a thumbnail for a specific asset version already exists, we can skip regeneration. This can be checked by seeing if the thumbnail file already exists at the target path.

## 6. Testing Approach

While we are not implementing tests now, a future testing strategy would include:

-   **Unit Tests:** Create a set of test images (PNG, JPEG, different sizes, and aspect ratios).
-   **Test Cases:**
    -   Verify that thumbnails are created with the correct dimensions (max 128x128).
    -   Confirm that aspect ratio is maintained.
    -   Test error handling with corrupted or non-existent image files.
    -   Test with different asset types to ensure only supported types generate thumbnails.

## 7. Changes to Existing Structures

### `apps/cpc-studio/src-tauri/src/editor_core/asset_processor.rs`

The `generate_thumbnail` function will be completely rewritten to implement the logic described above.

### `apps/cpc-studio/src-tauri/src/editor_core/assets.rs`

No changes are required to the data structures in this file. The `AssetMetadata` struct already has the `thumbnail_path: Option<PathBuf>` field.

### `apps/cpc-studio/src-tauri/Cargo.toml`

The `image` crate will be added as a dependency.

```toml
[dependencies]
image = "0.24"
```

This plan provides a clear path forward for implementing the thumbnail generation feature.