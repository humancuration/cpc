# Asset Management System Test Plan

## 1. Introduction

This document provides a comprehensive set of test cases for the Asset Management System in CPC Studio. It is intended for developers to ensure the system's functionality, reliability, and performance.

This test plan is based on the architecture defined in the [Asset Management UI Integration Architecture document](../architecture/asset_management_ui.md).

## 2. Test Environment Setup

### 2.1. Prerequisites
- A running instance of the CPC Studio desktop application.
- A configured P2P network with at least two peers for collaborative testing.
- Access to the development console to monitor logs and events.
- A running instance of Valkey and PostgreSQL.

### 2.2. Sample Test Assets
A collection of sample assets should be prepared, including:
- **Valid Image formats:** `.png`, `.jpg`, `.webp`
- **Invalid Image formats:** `.tiff`, `.bmp` (or other unsupported formats)
- **Audio formats:** `.mp3`, `.wav`, `.ogg`
- **3D Model formats:** `.gltf`, `.glb`
- **Large files:** > 100MB to test performance.
- **Files with long names and special characters.**

### 2.3. Simulating Collaborative Editing
- Run two instances of CPC Studio connected as peers on the same P2P network.
- Log in with different user accounts on each instance.
- Perform actions on the same assets from both instances to test real-time synchronization and locking.

## 3. Test Cases

### 3.1. Asset Upload and Processing
| Test Case ID | Description | Prerequisites | Test Steps | Expected Results | Validation Criteria |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **TC-UPLOAD-001** | Upload a single valid image file. | Asset Browser is open. | 1. Drag and drop a valid `.png` file into the Asset Browser. <br> 2. Wait for the import process to complete. | The image is successfully imported. A thumbnail is generated and displayed. | - The asset appears in the Asset Browser grid. <br> - A thumbnail is visible for the asset. <br> - No errors in the console. |
| **TC-UPLOAD-002** | Upload multiple valid files of different types. | Asset Browser is open. | 1. Select a `.jpg`, a `.wav`, and a `.gltf` file. <br> 2. Drag and drop them into the Asset Browser. | All files are imported successfully. Thumbnails are generated for visual assets. | - All three assets appear in the Asset Browser. <br> - The image has a thumbnail. <br> - The audio and model have placeholder icons. <br> - No errors in the console. |
| **TC-UPLOAD-003** | Attempt to upload an unsupported file type. | Asset Browser is open. | 1. Drag and drop a `.tiff` file into the Asset Browser. | An error message is displayed to the user. The file is not imported. | - A user-friendly error toast/notification is shown. <br> - The asset does not appear in the Asset Browser. |
| **TC-UPLOAD-004** | Attempt to upload a corrupted file. | Asset Browser is open. | 1. Create a corrupted image file. <br> 2. Drag and drop it into the Asset Browser. | The system handles the error gracefully. An error message is shown. | - A user-friendly error toast/notification is shown. <br> - The asset does not appear in the Asset Browser. <br> - The backend `asset_processor` logs an error but does not crash. |

### 3.2. Version History
| Test Case ID | Description | Prerequisites | Test Steps | Expected Results | Validation Criteria |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **TC-VERSION-001** | View version history of an asset. | An asset has been modified and saved at least once. | 1. Select an asset. <br> 2. Open the Inspector Panel. <br> 3. Navigate to the Version History tab/component. | The version history is displayed, showing at least two versions. | - `VersionHistoryViewer.svelte` shows a list of versions with timestamps and authors. |
| **TC-VERSION-002** | Restore a previous version of an asset. | An asset has multiple versions. | 1. Select an older version from the history. <br> 2. Click the "Restore" button. | The asset reverts to the selected version. A new version is created to mark the restoration. | - The asset's content and metadata in the Inspector Panel match the restored version. <br> - A new entry appears at the top of the version history. |
| **TC-VERSION-003** | Compare two versions of a text-based asset. | A text-based asset (e.g., script, markdown) has been modified. | 1. In the Version History viewer, select two different versions. <br> 2. Click "Show Diff". | A diff view is displayed showing the changes between the two versions. | - The `diffResult` in `VersionHistoryViewer.svelte` is populated with the text diff. |

### 3.3. Concurrent Editing and Synchronization
| Test Case ID | Description | Prerequisites | Test Steps | Expected Results | Validation Criteria |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **TC-CONCUR-001** | Lock an asset. | Two users (A and B) are viewing the same asset folder. | 1. User A right-clicks an asset and selects "Lock". | The asset is shown as locked for both users. User B cannot edit the asset. | - A lock icon appears on the asset for both users. <br> - User B's "Edit" button in the Inspector Panel is disabled for that asset. <br> - A `asset-locked` event is received by User B. |
| **TC-CONCUR-002** | Unlock an asset. | User A has an asset locked. | 1. User A right-clicks the locked asset and selects "Unlock". | The lock is removed for both users. User B can now lock and edit the asset. | - The lock icon disappears for both users. <br> - User B's "Edit" button becomes enabled. <br> - A `asset-unlocked` event is received by User B. |
| **TC-CONCUR-003** | Handle edit conflict. | Two users (A and B) are editing the same asset's metadata without locking. | 1. User A opens the Inspector and starts editing a property. <br> 2. Before User A saves, User B edits and saves the same asset. <br> 3. User A attempts to save their changes. | User A is presented with a conflict resolution dialog. | - The `conflictData` in `InspectorPanel.svelte` is populated. <br> - User A sees a dialog to "Use Server Version" or "Keep My Changes". |
| **TC-CONCUR-004** | Resolve conflict by accepting server version. | A conflict has occurred as in TC-CONCUR-003. | 1. User A clicks "Use Server Version". | User A's local changes are discarded, and the UI is updated with the latest version from the server. | - User A's Inspector Panel now shows the metadata saved by User B. <br> - The conflict dialog disappears. |

### 3.4. Bevy Asset Integration
| Test Case ID | Description | Prerequisites | Test Steps | Expected Results | Validation Criteria |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **TC-BEVY-001** | Load a texture asset in a Bevy scene. | A valid texture is in the asset library. | 1. Create a simple Bevy scene. <br> 2. Use the `AssetServer` to load the texture by its path. <br> 3. Apply the texture to a material on a 2D sprite or 3D mesh. | The texture is loaded and displayed correctly in the Bevy application window. | - The `CustomAssetLoader` in `bevy_asset_bridge.rs` is invoked for the texture. <br> - The texture appears on the object in the rendered scene. |
| **TC-BEVY-002** | Load an audio asset in a Bevy scene. | A valid audio file is in the asset library. | 1. Create a simple Bevy scene. <br> 2. Use the `AssetServer` to load the audio asset. <br> 3. Play the audio using Bevy's audio system. | The audio plays correctly. | - The `CustomAssetLoader` is invoked for the audio file. <br> - Sound is audible. |

### 3.5. Error Handling
| Test Case ID | Description | Prerequisites | Test Steps | Expected Results | Validation Criteria |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **TC-ERROR-001** | Handle network disconnection during an operation. | A user is performing an action that requires network (e.g., locking, saving). | 1. Disconnect the user's machine from the network. <br> 2. Attempt to lock an asset. | A user-friendly error message is displayed. The application does not crash. | - A toast notification indicates a network error. <br> - The application remains responsive. |
| **TC-ERROR-002** | Handle Valkey cache unavailability. | Valkey server is running. | 1. Stop the Valkey server. <br> 2. Navigate to a folder in the Asset Browser. | Assets are still loaded, but from the primary database (PostgreSQL). Performance may be slower. | - Assets are displayed correctly in the browser. <br> - The backend logs show an error connecting to Valkey, but the request succeeds by falling back to the database. |