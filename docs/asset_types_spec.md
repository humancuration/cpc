# Asset Types Specification

## 1. Introduction

This document provides a detailed specification for the various asset types supported by the `cpc-studio` asset management system. For each asset type, it defines a common metadata structure as well as type-specific properties. This specification is the source of truth for the data models used in the `AssetMetadataStore` and the UI components in the editor's Inspector Panel.

## 2. Common Asset Metadata

All assets, regardless of their type, will share a common set of metadata fields. This base metadata is essential for identification, versioning, and organization within the project.

| Field Name      | Data Type            | Description                                                                                             |
|-----------------|----------------------|---------------------------------------------------------------------------------------------------------|
| `asset_id`      | `UUID`               | A universally unique identifier for the asset. Generated upon import and remains constant.              |
| `name`          | `String`             | The user-facing name of the asset (e.g., "PlayerSprite"). Can be renamed by the user.                    |
| `path`          | `PathBuf`            | The relative path to the asset's source file within the project's `assets/` directory.                  |
| `thumbnail_path`| `Option<PathBuf>`    | An optional, locally-generated path to a preview thumbnail (e.g., for models and textures). Not synced. |
| `asset_type`    | `Enum`               | The type of the asset (e.g., `Texture`, `Model`, `Audio`).                                              |
| `version`       | `u64`                | A version number that is incremented on each modification to the asset's metadata or content.           |
| `vector_clock`  | `Map<PeerID, u64>`   | The vector clock used by the `ReconciliationEngine` to manage concurrent edits and resolve conflicts.   |

This metadata will be stored in the `AssetMetadataStore` and synchronized across all peers.

## 3. Asset-Specific Properties

Below are the detailed specifications for each supported asset type.

### 3.1. Textures (Images)

-   **`asset_type`**: `Texture`
-   **Supported Formats**: `.png`, `.jpg`, `.jpeg`, `.bmp`, `.gif` (Permissive formats only)
-   **Description**: 2D images used for sprites, UI elements, and model materials.

**Type-Specific Properties:**

| Property Name   | Data Type | UI Control      | Description                                                                    |
|-----------------|-----------|-----------------|--------------------------------------------------------------------------------|
| `format`        | `Enum`    | Dropdown        | Compression format to use in-engine (e.g., `Uncompressed`, `DXT5`, `BC7`).      |
| `srgb`          | `bool`    | Checkbox        | Whether the texture contains color data in the sRGB color space.               |
| `generate_mips` | `bool`    | Checkbox        | Whether to automatically generate mipmaps to improve rendering quality at a distance. |
| `filter_mode`   | `Enum`    | Dropdown        | Texture filtering mode (`Nearest`, `Linear`).                                  |
| `wrap_mode`     | `Enum`    | Dropdown        | How the texture should be sampled outside of its 0-1 UV range (`Clamp`, `Repeat`). |

### 3.2. 3D Models

-   **`asset_type`**: `Model`
-   **Supported Formats**: `.gltf`, `.glb`
-   **Description**: 3D meshes, with optional materials and animations. Upon import, a thumbnail is automatically generated for preview in the editor. See the [Thumbnail System Documentation](./thumbnail_system.md) for more details.

**Type-Specific Properties:**

| Property Name         | Data Type | UI Control | Description                                                                   |
|-----------------------|-----------|------------|-------------------------------------------------------------------------------|
| `import_materials`    | `bool`    | Checkbox   | Whether to import materials defined within the model file.                    |
| `import_animations`   | `bool`    | Checkbox   | Whether to import animations defined within the model file.                   |
| `scale_factor`        | `f32`     | Number     | A uniform scaling factor to apply to the model on import.                     |
| `mesh_compression`    | `Enum`    | Dropdown   | Level of mesh compression to apply (`None`, `Medium`, `High`).                |

### 3.3. Audio Files

-   **`asset_type`**: `Audio`
-   **Supported Formats**: `.wav`, `.ogg`
-   **Description**: Sound effects and music tracks.

**Type-Specific Properties:**

| Property Name | Data Type | UI Control | Description                                                              |
|---------------|-----------|------------|--------------------------------------------------------------------------|
| `looping`     | `bool`    | Checkbox   | Whether the audio clip should loop by default when played.               |
| `stream`      | `bool`    | Checkbox   | Whether to stream the audio from disk or load it entirely into memory.   |
| `volume`      | `f32`     | Slider     | The default volume for the audio clip (0.0 to 1.0).                      |

### 3.4. Scripts

-   **`asset_type`**: `Script`
-   **Supported Formats**: `.rs` (for Bevy scripting)
-   **Description**: Code files that define component logic and game behaviors.

**Type-Specific Properties:**
*(Scripts currently have no specific import settings beyond the common metadata. This section can be expanded if a scripting-specific compilation pipeline is introduced.)*

### 3.5. Prefabs

-   **`asset_type`**: `Prefab`
-   **Supported Formats**: `.prefab.json`
-   **Description**: A pre-configured collection of entities and components that can be instantiated in a scene. Prefabs are themselves assets, created within the editor.

**Type-Specific Properties:**
*(Prefabs are defined by their content—a serialized Bevy scene snippet—and do not have additional import settings. Their content is managed directly by the scene editor.)*