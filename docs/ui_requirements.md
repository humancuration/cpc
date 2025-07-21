# Asset Management UI Requirements

## 1. Introduction

This document outlines the user interface (UI) requirements for the asset management features within `cpc-studio`. These specifications are intended to guide the design and implementation of the SvelteKit frontend, ensuring a user-friendly and powerful workflow for managing project assets.

## 2. Asset Browser Panel

The Asset Browser is the central hub for all asset-related activities.

### 2.1. View and Layout

-   **Display:** Must be a dockable panel within the main editor interface.
-   **View Modes:** Should support at least two view modes:
    1.  **Grid View:** Displays assets as thumbnails with their names below. Ideal for visual assets like textures and models.
    2.  **List View:** Displays assets in a table with sortable columns (e.g., Name, Type, Last Modified).
-   **Folder Tree:** A collapsible folder hierarchy should be visible alongside the main view to allow for easy navigation of the `assets/` directory structure.

### 2.2. Functionality

-   **Navigation:**
    -   Double-clicking a folder navigates into it.
    -   Breadcrumbs should be displayed to show the current path and allow for quick navigation up the hierarchy.
-   **Asset Operations (Context Menu / Toolbar):**
    -   **Import:** Open a file dialog to import new assets into the current folder.
    -   **Create:** Allow for the creation of new assets from scratch (e.g., "Create Material", "Create Prefab").
    -   **Rename:** In-place renaming of assets and folders.
    -   **Delete:** Move selected assets/folders to a project-specific trash or delete them permanently after confirmation.
    -   **Show in Filesystem:** A context menu option to reveal the selected asset file in the operating system's file explorer.
-   **Drag and Drop:**
    -   **Import:** Dragging files from the user's desktop into the Asset Browser should initiate the import process.
    -   **Organization:** Dragging assets and folders within the browser should move them to the new location.
    -   **Usage:** Dragging an asset (e.g., a Texture) into the scene viewport or an Inspector Panel field should assign it.
-   **Search and Filtering:**
    -   A search bar to filter assets in the current view by name.
    -   A filter button to show/hide assets by type (e.g., show only Textures).

## 3. Preview Generation and Display

Visual feedback is critical for an efficient workflow.

-   **Thumbnails:** The `Asset Processor` backend service must generate thumbnails for visual assets (`Texture`, `Model`). These thumbnails are displayed in the Asset Browser's Grid View.
-   **Preview Pane:** When an asset is selected in the Asset Browser, a dedicated Preview Pane (which could be part of the Inspector) should show a larger, more detailed preview:
    -   **Textures:** Display the image.
    -   **Models:** Display an interactive 3D view of the model, allowing the user to rotate and zoom.
    -   **Audio:** Display audio waveform and provide playback controls (play, pause, stop).
    -   **Scripts/Text-based assets:** Display the content with syntax highlighting.

## 4. Metadata Editing (Inspector Panel)

When an asset is selected, the Inspector Panel must display its metadata for editing.

-   **Common Metadata:** The top of the inspector should show the common asset properties (`name`). The `asset_id` should be visible but read-only.
-   **Type-Specific Properties:** Below the common section, the UI must dynamically render the appropriate controls for the selected asset's type-specific properties, as defined in `asset_types_spec.md`.
    -   **Example Controls:** Use checkboxes for booleans, dropdowns for enums, sliders/number inputs for numerical values.
-   **Apply/Revert:** Changes made in the inspector should be applied immediately, generating the appropriate `P2PEvent`. There is no "Save" button; the UI state is the source of truth for an update action.

## 5. Conflict Resolution Interface

In a collaborative environment, conflicts are inevitable and the UI must help the user resolve them.

-   **Notification:** When the `ReconciliationEngine` detects a conflict that requires user intervention (e.g., two peers modify the same file's content), a non-intrusive notification should appear in the UI. This could be a small icon on the affected asset in the Asset Browser.
-   **Conflict Resolution View:** Clicking the notification or the conflicted asset should open a dedicated modal or view.
-   **Required Elements:**
    -   **Identification:** Clearly state which asset is in conflict.
    -   **Version Display:** Show a side-by-side comparison of the conflicting versions.
        -   For visual assets like textures, show both images.
        -   For text-based assets, show a diff view.
    -   **Source Information:** Indicate which peer (or "local changes") each version came from.
    -   **Actions:** Provide clear buttons for the user to choose a version:
        -   "Keep My Version"
        -   "Accept Remote Version"
-   **Resolution:** Once a choice is made, the system will generate a new "resolving" `P2PEvent` to ensure all peers converge on the chosen version.