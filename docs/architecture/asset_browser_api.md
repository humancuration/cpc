# Asset Browser Backend API Design

## Overview
This document outlines the backend architecture for the Asset Browser feature. The asset browser allows users to:
- Browse and manage assets (images, videos, audio, documents, 3D models)
- Import new assets
- Preview assets
- Organize assets in folders

## Data Structures

### AssetMetadata
```rust
pub struct AssetMetadata {
    pub id: AssetId,           // UUID identifying the asset
    pub name: String,          // User-friendly name
    pub path: PathBuf,         // Full path to asset
    pub size: u64,             // File size in bytes
    pub asset_type: AssetType, // Type of asset
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub thumbnail_path: Option<PathBuf>, // Path to thumbnail if available
}
```

### AssetType
```rust
pub enum AssetType {
    Image,
    Video,
    Audio,
    Document,
    Model3D,
    Other,
}
```

### FolderNode
Represents a folder in the asset hierarchy:
```rust
pub struct FolderNode {
    pub path: PathBuf,     // Full path to folder
    pub name: String,      // Folder name
    pub children: Vec<FolderNode>, // Subfolders
}
```

### PreviewData
```rust
pub enum PreviewData {
    Image { path: PathBuf, width: u32, height: u32 },
    Video { thumbnail_path: PathBuf, duration: f32 },
    Audio { waveform_path: PathBuf },
    Document { page_count: u32 },
    Unavailable,
}
```

## Tauri Commands

### get_assets_in_path
Fetch assets in a given folder path
```rust
#[tauri::command]
async fn get_assets_in_path(path: String) -> Vec<AssetMetadata> {
    // Implementation goes here
}
```

### import_asset
Import one or more assets into the system
```rust
#[tauri::command]
async fn import_asset(files: Vec<PathBuf>) -> Result<AssetId> {
    // Implementation goes here
}
```

### delete_asset
Delete an asset by ID
```rust
#[tauri::command]
async fn delete_asset(id: AssetId) {
    // Implementation goes here
}
```

### get_asset_preview
Get preview data for an asset
```rust
#[tauri::command]
async fn get_asset_preview(id: AssetId) -> PreviewData {
    // Implementation goes here
}
```

## Events
Events are emitted for asset operations:

### asset_created
```rust
pub struct AssetCreatedEvent {
    pub asset_id: AssetId,
    pub metadata: AssetMetadata,
}
```

### asset_updated
```rust
pub struct AssetUpdatedEvent {
    pub asset_id: AssetId,
    pub metadata: AssetMetadata,
}
```

### asset_deleted
```rust
pub struct AssetDeletedEvent {
    pub asset_id: AssetId,
}
```

### asset_conflict
```rust
pub struct AssetConflictEvent {
    pub asset_id: AssetId,
    pub reason: String, // e.g., "Duplicate file", "Invalid format"
}
```

## Storage Recommendations
1. **Metadata Storage**: Use SQLite database with SeaORM for asset metadata
2. **Asset Files**: Store in platform-specific user directories:
   - Windows: `%APPDATA%/cpc/assets`
   - macOS: `~/Library/Application Support/cpc/assets`
   - Linux: `~/.local/share/cpc/assets`
3. **Thumbnails**: Generate and store in a `thumbnails` subdirectory
4. **Indexing**: Maintain a folder tree structure in memory for quick navigation

## Data Flow
1. On application start:
   - Load folder tree structure
   - Initialize database connection
   - Register event handlers

2. On user navigation:
   - Query database for assets in current path
   - Update folder tree if needed

3. On asset operations:
   - Perform file operations
   - Update database
   - Emit appropriate event